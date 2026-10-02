//LICENSE Portions Copyright 2026 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.

//! Ordered C statements share the expression arena and preserve lexical blocks.

use super::{BindingCatalog, SUPPORT, macro_identifier, typed, write_fallback};
use crate::{AnalysisSession, MacroAnalysis, SkipReason, syntax::Statement};
use std::fmt::Write;

pub(super) fn render(
    session: &AnalysisSession<'_>,
    analysis: &MacroAnalysis,
    bindings: &BindingCatalog,
    renderer: &typed::Renderer<'_>,
    explicit_marker: bool,
) -> Result<String, SkipReason> {
    let expression = analysis.expression.as_ref().expect("candidate statement expression");
    let body = expression.syntax.statement_body.as_ref().expect("statement body");
    let mut rust = String::from("{ ");
    if let Some(crate::ExpansionResult::Expanded { expansion }) =
        session.expansions().results.get(&analysis.name)
    {
        for fallback in &expansion.constant_fallbacks {
            write_fallback(&mut rust, &fallback.name, &fallback.reason);
        }
    }
    // Root-only expression matching cannot establish equivalence of local
    // declarations or ordered side effects. Preserve only complete simple-return
    // wrappers until statement-aware delegation proves the entire body.
    if let Ok(Some(delegation)) = crate::delegation::direct_delegation(session, analysis)
        && bindings.macros.contains(&delegation.callee)
        && let Some(callee) = macro_identifier(&delegation.callee)
    {
        let mode = if explicit_marker { "return_as; $__pgrx_c_return, " } else { "public; " };
        write!(rust, "$crate::{callee}!(@__pgrx_emit_{mode}").expect("String output");
        for root in delegation.arguments {
            renderer.delegated_argument(root, &mut rust);
        }
        rust.push_str(") }");
        return Ok(rust);
    }
    for dependency in &analysis.dependencies {
        if dependency.kind == crate::MacroKind::FunctionLike {
            write_fallback(
                &mut rust,
                &dependency.name,
                "preserving the complete statement body and its order has not been proved equivalent to C substitution",
            );
        }
    }
    render_statements(&body.statements, renderer, explicit_marker, &mut rust)?;
    rust.push('}');
    Ok(rust)
}

fn render_statements(
    statements: &[Statement],
    renderer: &typed::Renderer<'_>,
    explicit_marker: bool,
    rust: &mut String,
) -> Result<(), SkipReason> {
    for statement in statements {
        match statement {
            Statement::Declaration { name, initializer, .. } => {
                renderer.declare_local(name, *initializer, rust)?
            }
            Statement::Expression { expression, .. } => {
                renderer.render(*expression, typed::Context::Discard, rust)?;
                rust.push_str("; ");
            }
            Statement::Block { statements, .. } => {
                rust.push_str("{ ");
                render_statements(statements, renderer, explicit_marker, rust)?;
                rust.push_str("} ");
            }
            Statement::Return { expression, .. } => {
                let helper = if explicit_marker {
                    "return_value_as::<$__pgrx_c_return, _>"
                } else {
                    "return_value"
                };
                write!(rust, "return {SUPPORT}::expression_result::{helper}(")
                    .expect("String output");
                renderer.render(*expression, typed::Context::Value, rust)?;
                rust.push_str("); ");
            }
        }
    }
    Ok(())
}

/// Inspect stringified tokens before normalization, including forwarded opaque
/// expression fragments. Field, path and string matches are conservative skips.
pub(super) fn guard(analysis: &MacroAnalysis, normalizer: &str) -> Result<String, SkipReason> {
    let body = analysis
        .expression
        .as_ref()
        .and_then(|expression| expression.syntax.statement_body.as_ref())
        .expect("statement body");
    let names = body
        .walk()
        .filter_map(|statement| match statement {
            Statement::Declaration { name, .. } => Some(name.as_str()),
            _ => None,
        })
        .collect::<Vec<_>>();
    if names.len() > 64 {
        return Err(super::skip(
            analysis,
            crate::SkipReasonCode::BudgetExceeded,
            "C local-scope checking exceeds its 64-declaration bound",
            Some(body.tokens),
        ));
    }
    let mut rust =
        String::from("(@__pgrx_c_guard_locals $mode:ident [$($done:tt)*]; $($raw:tt)*) => {{ ");
    if !names.is_empty() {
        let message = format!(
            "C macro argument mentions local {}; C substitution and Rust hygiene would resolve it differently, or the 4096-byte scope-check bound was exceeded",
            names.join(", ")
        );
        write!(rust, "const _: () = ::core::assert!({SUPPORT}::statements::local_scope_allowed(::core::stringify!($($raw)*), &{names:?}), {message:?}); ").expect("String output");
    }
    writeln!(rust, "$crate::{normalizer}!(@collect $mode [$($done)*]; $($raw)*) }}}};")
        .expect("String output");
    Ok(rust)
}
