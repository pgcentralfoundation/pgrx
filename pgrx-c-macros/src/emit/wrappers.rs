//LICENSE Portions Copyright 2026 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.

//! Rust functions that run translated static inline functions under their binding names.
//!
//! The binding of a static inline function calls its C wrapper through the crate's FFI
//! guard, which sets a nonlocal-error boundary on every call. When a batch translates the
//! function's body, a function with the binding's exact signature can run that translation
//! instead. Translations never make an unguarded native call: they refuse variadic calls,
//! and their direct and indirect calls go through guarded bindings or generated adapters.
//! So a PostgreSQL error raised under the translation stops at the guard of the native call
//! that raised it, and that guard checks the thread. The function itself checks the thread
//! only when the translation reads or writes a C global variable, the one kind of backend
//! state it touches without a native call. Other translations stay callable from any thread,
//! as their macros are, and do not claim pgrx's active thread on a worker thread.
//!
//! The functions are definitions inside one exported macro. The defining crate cannot expand
//! its own `include!`-generated exported macros through `$crate` paths
//! (rust-lang/rust#52234), so a dependent crate invokes the macro in a module that glob-imports
//! the bindings. Each function there shadows the binding of the same name.

use super::{
    BindingCatalog, EmissionStatus, GeneratedAdapters, MacroEmission, SUPPORT, direct_calls,
    rust_identifier, translated_inline,
    types::{self, rust_path},
};
use crate::{AnalysisSession, ExpressionKind, MacroAnalysis, RustBindingType, TypeCategory};
use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Write;

/// Render the exported macro, `__pgrx_c_inline_functions!`, that defines a function for each
/// translated static inline function whose binding signature the translation can take and
/// return exactly.
///
/// Returns an empty string unless the catalog has both a thread check and an FFI guard: the
/// functions replace guarded bindings, which is sound only while every native call a
/// translation makes has the guard, and the translations that touch globals need the check.
/// A function whose signature does not reconcile is left out, so its guarded binding stays
/// in use.
pub(super) fn render(
    session: &AnalysisSession<'_>,
    macros: &[MacroEmission],
    bindings: &BindingCatalog,
    adapters: &GeneratedAdapters,
) -> Result<String, String> {
    let (Some(thread_check), Some(_)) = (&bindings.thread_check, &bindings.ffi_boundary) else {
        return Ok(String::new());
    };
    let thread_check = rust_path(thread_check)?;
    let lowering = types::Lowering::new(
        session.frontend().declarations(),
        bindings,
        adapters.types(),
        &session.frontend().profile().target,
    );
    let translations = macros
        .iter()
        .filter(|emission| {
            matches!(emission.status, EmissionStatus::Emitted { .. })
                && translated_inline(session, &emission.analysis)
        })
        .map(|emission| (emission.analysis.name.as_str(), &emission.analysis))
        .collect::<BTreeMap<_, _>>();
    let mut known = BTreeMap::new();
    let mut functions = String::new();
    for name in translations.keys() {
        let check = touches_globals(session, name, &translations, &mut known)
            .then_some(thread_check.as_str());
        if let Ok(function) = function(session, name, bindings, &lowering, check) {
            functions.push_str(&function);
        }
    }
    crate::format_rust_macros(&format!(
        "#[doc(hidden)]\n#[macro_export]\nmacro_rules! __pgrx_c_inline_functions {{\n() => {{\n{functions}}};\n}}\n"
    ))
}

/// Whether the translation of `name` reads or writes a C global variable, directly or through
/// the translations of the functions it calls. Its other native calls check the thread.
fn touches_globals<'a>(
    session: &AnalysisSession<'_>,
    name: &'a str,
    translations: &BTreeMap<&'a str, &'a MacroAnalysis>,
    known: &mut BTreeMap<&'a str, Option<bool>>,
) -> bool {
    match known.get(name) {
        Some(Some(touches)) => return *touches,
        // A function that reaches itself calls itself natively; checking costs only a check.
        Some(None) => return true,
        None => {}
    }
    known.insert(name, None);
    let analysis = translations[name];
    let variables = &session.frontend().declarations().variables;
    let nodes = analysis.expression.as_ref().map_or(&[][..], |expression| &expression.syntax.nodes);
    // Compiler expansion leaves each global a macro in the body names as an identifier. A
    // local that shares a global's name counts as well, which only adds a check.
    let touches = nodes.iter().any(|node| {
        matches!(&node.kind, ExpressionKind::Identifier { name } if variables.contains_key(name))
    }) || direct_calls(analysis)
        .filter_map(|callee| translations.get_key_value(callee).map(|(callee, _)| *callee))
        .any(|callee| touches_globals(session, callee, translations, known));
    known.insert(name, Some(touches));
    touches
}

/// Render one function with its binding's signature that converts each argument to its C
/// identity, evaluates the translation, and converts the result back to binding storage.
/// `thread_check` is the check to run first, for a translation that touches globals.
fn function(
    session: &AnalysisSession<'_>,
    name: &str,
    bindings: &BindingCatalog,
    lowering: &types::Lowering<'_>,
    thread_check: Option<&str>,
) -> Result<String, String> {
    let binding = bindings.functions.get(name).ok_or("no binding to replace")?;
    // Only bindgen's static wrappers are bindings of an inline function; an ordinary
    // symbol of the same name is a different definition.
    if !binding.uses_cshim || !binding.guarded {
        return Err("binding is not a guarded static inline wrapper".into());
    }
    let function_name = rust_identifier(name).ok_or("C name is not a Rust identifier")?;
    if binding.path.last() != Some(&function_name) {
        return Err("binding is not named after its C function".into());
    }
    let signature = &session
        .frontend()
        .declarations()
        .function_signatures
        .get(name)
        .ok_or("no C prototype")?
        .signature;
    // Records, enums and plain char reach Rust through generated native transport, whose
    // storage can differ from the binding's.
    if super::functions::native_transport(signature).is_some() {
        return Err("prototype needs native value transport".into());
    }
    let parameters = signature.parameters.as_ref().ok_or("no C prototype")?;
    if parameters.len() != binding.parameters.len() {
        return Err("binding arity differs from the C prototype".into());
    }
    let names = session.inline_definition(name).map(|definition| &definition.parameters);
    let mut used = BTreeSet::new();
    let mut inputs = Vec::new();
    let mut types = Vec::new();
    let mut conversions = String::new();
    let mut arguments = Vec::new();
    for (index, (ty, storage)) in parameters.iter().zip(&binding.parameters).enumerate() {
        let lowered = lowering.resolve_with_storage(ty, storage)?;
        let parameter = names
            .and_then(|names| names.get(index)?.as_deref())
            .and_then(rust_identifier)
            .filter(|parameter| !parameter.starts_with("__pgrx") && used.insert(parameter.clone()))
            .unwrap_or_else(|| format!("__pgrx_arg{index}"));
        // The macro classifies identifier operands by spelling, so it receives locals
        // that no C declaration can name.
        let local = format!("__pgrx_c_arg{index}");
        inputs.push(format!("{parameter}: {}", lowered.storage));
        types.push(lowered.storage);
        writeln!(
            conversions,
            "let {local} = <{} as {SUPPORT}::expression::CType>::from_storage({parameter});",
            lowered.marker
        )
        .expect("String output");
        arguments.push(local);
    }
    let call = format!("$crate::{function_name}!({})", arguments.join(", "));
    let (output, evaluation) = if signature.result.category == TypeCategory::Void {
        if binding.result != RustBindingType::Unit {
            return Err("binding returns a value from a void C function".into());
        }
        (String::new(), format!("let _ = {call};"))
    } else {
        let lowered = lowering.resolve_with_storage(&signature.result, &binding.result)?;
        (
            format!(" -> {}", lowered.storage),
            format!(
                "{SUPPORT}::expression_result::return_value_as::<{}, _>({call}.into_value())",
                lowered.marker
            ),
        )
    };
    let (check, checked, established) = match thread_check {
        Some(check) => (
            format!("{check}();\n"),
            " The body reads or writes PostgreSQL globals, so the function first checks that it \
             runs on the backend thread.",
            ", and the check above established the backend thread",
        ),
        None => (String::new(), "", ". The body touches no PostgreSQL global itself"),
    };
    let documentation = format!(
        "Run the Rust translation of the C inline function `{name}`.\n\n\
         This build translates the function's body, so a call evaluates it in Rust as the \
         generated macro `{function_name}!` does. It sets no error boundary of its own: every \
         PostgreSQL function the body calls keeps its guard, which checks the thread and turns \
         an error into a Rust panic.{checked} The body panics where the macro documents.\n\n\
         # Safety\n\n\
         Pointer arguments must remain live and valid for every access the original function \
         performs, with its required alignment, initialization, bounds, and aliasing. The \
         caller must uphold the original function's resource and thread requirements."
    );
    let binding_path = rust_path(&binding.path)?;
    let mut rust = String::new();
    writeln!(
        rust,
        "#[doc = {documentation:?}]\n\
         #[inline]\n\
         #[track_caller]\n\
         #[allow(non_snake_case, unused_unsafe, clippy::too_many_arguments)]\n\
         pub unsafe fn {function_name}({}){output} {{\n\
         // Replacing the binding must not change what its callers can pass or receive.\n\
         const _: [unsafe fn({}){output}; 2] = [{function_name}, {binding_path}];\n\
         {check}\
         {conversions}\
         // SAFETY: the caller upholds the original function's contract, which the \
         translation shares{established}.\n\
         unsafe {{ {evaluation} }}\n\
         }}",
        inputs.join(", "),
        types.join(", ")
    )
    .expect("String output");
    Ok(rust)
}
