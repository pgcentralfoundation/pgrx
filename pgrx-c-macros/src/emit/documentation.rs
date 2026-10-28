//LICENSE Portions Copyright 2026 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.

//! Documentation shells: each generated macro's documentation, invocation forms
//! and result type, without its implementation.
//!
//! A shell arm never evaluates its operands. An expression shell names the real
//! macro's result type with a witness: the real value arm, rendered with each
//! helper call whose signature fixes its result type replaced by `type_witness`.
//! Dependents of a documentation build therefore type-check against the same
//! result types as ordinary builds.

use super::{arguments, typed, write_doc_comments};
use crate::{MacroAnalysis, ParameterRole};
use std::fmt::Write;

/// How a typed inline-function adapter calls and converts.
pub(super) const INLINE_CALL: &str = "Calls the original function through its inspected C prototype. Each operand is evaluated once and converted using C parameter assignment rules. The result retains the function's original C type; `.get()` extracts its native storage, including `()` for a void result. Native calls retain the backend thread, PostgreSQL error, and caller safety contracts of the generated guarded binding.";
/// How a translated inline function body evaluates.
pub(super) const INLINE_BODY: &str = "Translates the original function definition. Each operand is evaluated once and converted to its parameter type using C assignment rules, the statements run in order, and the returned value is converted to the function's C return type. `.get()` extracts its native storage. Calls inside the body keep the backend thread and PostgreSQL error contracts of their guarded bindings.";
/// How `__FILE__` and `__LINE__` translate.
pub(super) const INVOCATION_DIAGNOSTICS: &str = "Invocation diagnostics use Rust `file!()` and `line!()` at the outer Rust source invocation. The filename is a static UTF-8 byte array with a final zero; the line must fit the inspected C int. Source-line-dependent C preprocessing and integer-constant-expression identity are outside this diagnostic contract.";
/// How dependency stringification translates.
pub(super) const STRINGIFICATION: &str = "Dependency stringification retains the compiler-expanded C diagnostic template and uses Rust `stringify!` on the original outer Rust operand tokens. It never evaluates those tokens. Operand spelling follows Rust whitespace and token rendering, requires ASCII, and preserves all groups and punctuation; it does not promise C preprocessor stringification for Rust-specific syntax. Such strings are static zero-terminated native const-char arguments only. Their C array extent, address, integer-constant-expression identity and return as a string value are unsupported. Root macros containing # remain explicit skips.";
/// How C `return` statements behave in the caller.
pub(super) const CALLER_RETURNS: &str = "C return statements in this macro exit the enclosing Rust function or closure. Call it directly, without an outer `return`. The enclosing result must have an unambiguous C identity; otherwise use `@__pgrx_c_return_as [CMarker];` before the arguments to specify the original C function's return type. Return conversion uses C assignment rules, including truncation and pointer qualification. Rust caller cleanup follows normal Rust return behavior. Pointer access and native calls keep their usual caller safety obligations.";
/// How statement macros execute.
pub(super) const STATEMENTS: &str = "This macro executes C statements in order and yields no value. Local blocks retain their C scope. Pointer access and native calls keep their usual caller safety obligations.";
/// Caller obligations shared by every generated macro.
pub(super) const SAFETY: &str = "Preserve each operand's original C identity; a Rust alias or bindgen constant's storage type may differ from its C expression type. For pointer or place operations, the caller must establish provenance, allocation bounds, alignment, initialization, valid values, lifetimes and aliasing required by the original C operation. Mutation requires writable storage. Native calls and globals require the permitted PostgreSQL backend thread, valid PostgreSQL resource ownership and the original function's preconditions. Generated guards preserve error boundaries; they do not prove these obligations.";

/// Documentation paragraphs that many generated macros repeat verbatim.
///
/// Documentation shells refer to each one through the macro named here instead of
/// repeating its text; [`documentation_shell_support`] defines those macros.
const SHARED_PARAGRAPHS: [(&str, &str); 7] = [
    ("__pgrx_c_doc_inline_call", INLINE_CALL),
    ("__pgrx_c_doc_inline_body", INLINE_BODY),
    ("__pgrx_c_doc_invocation_diagnostics", INVOCATION_DIAGNOSTICS),
    ("__pgrx_c_doc_stringification", STRINGIFICATION),
    ("__pgrx_c_doc_caller_returns", CALLER_RETURNS),
    ("__pgrx_c_doc_statements", STATEMENTS),
    ("__pgrx_c_doc_safety", SAFETY),
];

/// Rustdoc-facing parts and result type of one emitted macro.
///
/// A documentation snapshot shows these instead of the generated implementation.
/// The defining location is kept apart from the rest of the comment, so identical
/// definitions from several PostgreSQL versions can share one documented macro.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MacroDocumentation {
    /// Exported Rust macro identifier.
    pub identifier: String,
    /// First documentation line before the location, such as `C macro TYPEALIGN`.
    pub title: String,
    /// Defining header filename and first line, such as `c.h:801`.
    pub location: Option<String>,
    /// Documentation after the title line: original definition and invocation contract.
    pub body: String,
    /// Invocation matchers for the forms the generated macro accepts.
    pub forms: Vec<String>,
    /// Whether the generated macro is hidden from documentation.
    pub hidden: bool,
    /// What every shell arm expands to.
    pub result: DocumentedResult,
}

/// The type a documentation shell gives its invocation.
///
/// Every variant is Rust source to place in a shell arm; none of it executes.
/// Each block borrows the evaluated operands that its expression does not use,
/// so an operand's own type errors still surface.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DocumentedResult {
    /// The real macro yields `()`. `body` is a block that borrows each evaluated operand.
    Unit {
        /// A block of type `()` that borrows every evaluated operand.
        body: String,
    },
    /// The real public arm is `finish(v)`; `witness` is a block whose type equals `v`'s.
    Value {
        /// A block of the same type as the real value arm.
        witness: String,
    },
    /// Every path returns from the caller.
    Diverges,
    /// The witness could not be rendered; the shell keeps the `!` placeholder.
    Untyped {
        /// Why the witness could not be rendered.
        reason: String,
    },
}

/// Define the macros that documentation shells use.
///
/// Every shell arm expands to `__pgrx_c_documentation_shell!`, and each documentation
/// paragraph that many shells repeat is defined once as a hidden macro for their doc
/// comments. Include this once in the crate that defines the shells rendered by
/// [`MacroDocumentation::render_shell`], together with that crate's generated support
/// and runtime.
pub fn documentation_shell_support() -> String {
    let mut rust = String::from(
        "/// Expansion of every documentation-snapshot shell. Real C macros are generated for each build.\n\
         ///\n\
         /// Typed arms name the real macro's result type behind `if false`; every arm panics if run.\n\
         #[doc(hidden)]\n\
         #[macro_export]\n\
         macro_rules! __pgrx_c_documentation_shell {\n    \
             () => {\n        \
                 ::core::unimplemented!(\"pgrx documentation snapshot; C macros are generated for each build\")\n    \
             };\n    \
             (@unit $body:block) => {\n        \
                 if false $body else { $crate::__pgrx_c_documentation_shell!() }\n    \
             };\n    \
             (@value $witness:block) => {\n        \
                 if false {\n            \
                     $crate::__pgrx_c_macros::expression_result::finish($witness)\n        \
                 } else {\n            \
                     $crate::__pgrx_c_documentation_shell!()\n        \
                 }\n    \
             };\n\
         }\n",
    );
    for (name, paragraph) in SHARED_PARAGRAPHS {
        // Doc comments keep the space after `///`; the shared line keeps it too.
        writeln!(
            rust,
            "/// One documentation paragraph shared by snapshot shells.\n#[doc(hidden)]\n#[macro_export]\nmacro_rules! {name} {{\n    () => {{\n        {:?}\n    }};\n}}",
            format!(" {paragraph}")
        )
        .expect("String output");
    }
    rust
}

/// Render documentation comments and shell macros for snapshots.
impl MacroDocumentation {
    /// Complete doc comment, using `location` in place of this macro's own location.
    pub fn comment(&self, location: Option<&str>) -> String {
        comment(&self.title, location, &self.body)
    }

    /// Render a documentation-only macro that shows the supported invocation forms.
    ///
    /// Each arm expands to the defining crate's [`documentation_shell_support`] macro
    /// with this macro's [`DocumentedResult`], so dependents type-check a call as they
    /// would in an ordinary build, but running it panics. Doc lines that repeat a shared
    /// paragraph name its macro instead of repeating the text. `attributes` is written
    /// before the doc comment, for example a `#[cfg(...)]` line.
    pub fn render_shell(&self, location: Option<&str>, attributes: &str) -> Result<String, String> {
        let mut comment = String::new();
        write_doc_comments(&mut comment, &self.comment(location)).ok_or_else(|| {
            format!("documentation for {} exceeds the output bound", self.identifier)
        })?;
        let mut rust = String::from(attributes);
        for line in comment.lines() {
            match SHARED_PARAGRAPHS
                .iter()
                .find(|(_, paragraph)| line.strip_prefix("/// ") == Some(paragraph))
            {
                Some((name, _)) => writeln!(rust, "#[doc = crate::{name}!()]"),
                None => writeln!(rust, "{line}"),
            }
            .expect("String output");
        }
        if self.hidden {
            rust.push_str("#[doc(hidden)]\n");
        }
        writeln!(rust, "#[macro_export]\nmacro_rules! {} {{", self.identifier)
            .expect("String output");
        for (form, body) in self.arms() {
            writeln!(rust, "    ({form}) => {{ $crate::__pgrx_c_documentation_shell!({body}) }};")
                .expect("String output");
        }
        rust.push_str("}\n");
        crate::format_rust_macros(&rust)
    }

    /// Each shell arm's matcher and expansion, in matching order.
    ///
    /// The real macros read a named pointer's dereference, written `*pointer` or
    /// `(*pointer)`, through the pointer, so where C reads that operand as a place,
    /// such as the base of a `.` member access, the result keeps the pointer's `*mut` or
    /// `*const` qualification. A shell's `expr` operand cannot be taken apart, so when
    /// its witness reads exactly one operand as a place, arms for those two spellings
    /// come first and read the operand the same way. A dereference written another way,
    /// or of an operand in a witness that reads several operands as places, is still
    /// read as a native place.
    fn arms(&self) -> Vec<(String, String)> {
        let mut arms = Vec::new();
        let body = match &self.result {
            DocumentedResult::Unit { body } => format!("@unit {body}"),
            DocumentedResult::Value { witness } => {
                if let Some(name) = single_read_place(witness) {
                    let native = format!("(@native [${name}])");
                    let operand = format!("${name}:expr");
                    for form in self.forms.iter().filter(|form| form.matches(&operand).count() == 1)
                    {
                        for (matcher, spelling) in [
                            (format!("* ${name}:ident"), format!("* ${name}")),
                            (format!("(* ${name}:ident)"), format!("(* ${name})")),
                        ] {
                            let witness = witness
                                .replace(&native, &format!("(@deref [{spelling}] [${name}])"));
                            arms.push((
                                form.replace(&operand, &matcher),
                                format!("@value {witness}"),
                            ));
                        }
                    }
                }
                format!("@value {witness}")
            }
            DocumentedResult::Diverges | DocumentedResult::Untyped { .. } => String::new(),
        };
        arms.extend(self.forms.iter().map(|form| (form.clone(), body.clone())));
        arms
    }
}

/// The one operand that `witness` reads as a native place, if it reads exactly one.
fn single_read_place(witness: &str) -> Option<&str> {
    const READ_PLACE: &str = "@read_place; (@native [$";
    let mut names = witness.match_indices(READ_PLACE).filter_map(|(start, _)| {
        let rest = &witness[start + READ_PLACE.len()..];
        rest.find("])").map(|end| &rest[..end])
    });
    let first = names.next()?;
    names.all(|name| name == first).then_some(first)
}

/// A doc comment's title, location and body.
pub(super) fn comment(title: &str, location: Option<&str>, body: &str) -> String {
    match location {
        Some(location) => format!("{title} from {location}{body}"),
        None => format!("{title}{body}"),
    }
}

/// Invocation matchers matching the public arms of the generated macro.
///
/// Required boundary markers and the explicit return-type form are shown as in
/// the generated macro. Type, identifier and member designator operands use the
/// generated normalizer's fragments. An argument that the real macro never parses,
/// because it is unused or only stringified, accepts any tokens when it is last. A
/// token repetition before a comma would make the matcher ambiguous, so such an
/// argument followed by another is shown as an expression, as other operands are.
pub(super) fn documentation_forms(
    analysis: &MacroAnalysis,
    returning: bool,
    statement_boundary: bool,
    explicit_boundary: bool,
) -> Vec<String> {
    let mut operands = String::new();
    let mut trailing_comma = !analysis.parameters.is_empty();
    for (index, parameter) in analysis.parameters.iter().enumerate() {
        if index != 0 {
            operands.push_str(", ");
        }
        let name = arguments::name(analysis, index);
        let roles = parameter.roles.as_slice();
        let unparsed = !roles.is_empty()
            && roles
                .iter()
                .all(|role| matches!(role, ParameterRole::Unused | ParameterRole::Spelling));
        if unparsed && index + 1 == analysis.parameters.len() {
            // The real normalizer takes up to its comma, or none for an unused argument.
            // The repetition also takes a trailing comma. No block or witness names
            // this operand, so binding it inside a repetition is safe.
            let repetition = if roles == [ParameterRole::Unused] { '*' } else { '+' };
            write!(operands, "$(${name}:tt){repetition}").expect("String output");
            trailing_comma = false;
        } else if roles == [ParameterRole::Type] {
            write!(operands, "${name}:ty").expect("String output");
        } else if roles.contains(&ParameterRole::Identifier) {
            write!(operands, "${name}:ident").expect("String output");
        } else if roles == [ParameterRole::FieldDesignator] {
            // Witnesses never name a designator: `offsetof` always collapses.
            write!(operands, "$(${name}:ident).+").expect("String output");
        } else {
            write!(operands, "${name}:expr").expect("String output");
        }
    }
    if trailing_comma {
        operands.push_str(" $(,)?");
    }
    let boundary = if statement_boundary {
        "@__pgrx_c_statement; "
    } else if explicit_boundary {
        "@__pgrx_c_expression; "
    } else {
        ""
    };
    let mut forms = vec![format!("{boundary}{operands}").trim_end().to_owned()];
    if returning {
        let marker = arguments::return_marker(analysis);
        forms.push(
            format!("{boundary}@__pgrx_c_return_as [${marker}:ty]; {operands}")
                .trim_end()
                .to_owned(),
        );
    }
    forms
}

/// Classify what a successfully emitted macro yields and render its shell's block.
///
/// Statement bodies yield `()` unless every path returns from the caller, and an
/// empty macro yields `()`. Any other expression renders its value root as a
/// witness; a witness that cannot be rendered never affects the real macro. A
/// translated inline function body has no witness yet.
pub(super) fn result(
    analysis: &MacroAnalysis,
    renderer: typed::Renderer<'_>,
    inline_body: bool,
) -> DocumentedResult {
    if inline_body {
        return DocumentedResult::Untyped {
            reason: "a translated inline function body has no type witness".into(),
        };
    }
    let expression = analysis.expression.as_ref().expect("emitted macros have an expression");
    if let Some(statements) = &expression.syntax.statement_body {
        return if statements.always_returns {
            DocumentedResult::Diverges
        } else {
            DocumentedResult::Unit { body: block(analysis, "") }
        };
    }
    if matches!(
        expression.syntax.nodes[expression.syntax.root].kind,
        crate::syntax::ExpressionKind::Empty
    ) {
        return DocumentedResult::Unit { body: block(analysis, "") };
    }
    let mut spine = String::new();
    match renderer.witness().render(expression.syntax.root, typed::Context::Value, &mut spine) {
        Ok(()) => DocumentedResult::Value { witness: block(analysis, &spine) },
        Err(reason) => DocumentedResult::Untyped { reason: reason.message },
    }
}

/// Wrap `spine` in a block that also borrows each evaluated operand it does not use.
///
/// The real macro evaluates these operands, so borrowing them keeps their own type
/// errors. Operands the spine already uses are not borrowed again, which would be a
/// second use of a moved value. Spelling, type, identifier and unused operands are
/// never evaluated by the real macro and are not referenced.
fn block(analysis: &MacroAnalysis, spine: &str) -> String {
    let mut rust = String::from("{");
    for (index, parameter) in analysis.parameters.iter().enumerate() {
        let name = format!("${}", arguments::name(analysis, index));
        if parameter.roles.contains(&ParameterRole::Value) && !mentions(spine, &name) {
            write!(rust, " let _ = &{name};").expect("String output");
        }
    }
    if !spine.is_empty() {
        write!(rust, " {spine}").expect("String output");
    }
    rust.push_str(" }");
    rust
}

/// Whether `source` contains the metavariable `name` as a whole word.
fn mentions(source: &str, name: &str) -> bool {
    source.match_indices(name).any(|(start, _)| {
        !source[start + name.len()..]
            .bytes()
            .next()
            .is_some_and(|byte| byte.is_ascii_alphanumeric() || byte == b'_')
    })
}

/// Check metavariable matching without a compiler.
#[cfg(test)]
mod tests {
    use super::{DocumentedResult, MacroDocumentation, mentions};

    /// A metavariable that prefixes a longer one is not mentioned by it.
    #[test]
    fn metavariable_mentions_respect_word_boundaries() {
        assert!(mentions("f($x)", "$x"));
        assert!(mentions("$x", "$x"));
        assert!(!mentions("f($xy)", "$x"));
        assert!(!mentions("$__pgrx_c_capture_10", "$__pgrx_c_capture_1"));
        assert!(mentions("$__pgrx_c_capture_10 + $__pgrx_c_capture_1", "$__pgrx_c_capture_1"));
    }

    /// A witness that reads one operand as a place gets arms, matched first, that read
    /// `*pointer` and `(*pointer)` through the pointer. One that reads two does not.
    #[test]
    fn one_place_operand_gets_dereference_arms() {
        let shell = |witness: &str| MacroDocumentation {
            identifier: "NAME".into(),
            title: "C macro NAME".into(),
            location: None,
            body: String::new(),
            forms: vec!["$name:expr, $other:expr $(,)?".into()],
            hidden: false,
            result: DocumentedResult::Value { witness: witness.into() },
        };
        let place = "$crate::__pgrx_c_operand!(@read_place; (@native [$name]))";
        let witness = format!("{{ let _ = &$other; f({place}, {place}) }}");
        let arms = shell(&witness).arms();
        let forms = arms.iter().map(|(form, _)| form.as_str()).collect::<Vec<_>>();
        assert_eq!(
            forms,
            [
                "* $name:ident, $other:expr $(,)?",
                "(* $name:ident), $other:expr $(,)?",
                "$name:expr, $other:expr $(,)?"
            ]
        );
        for (body, spelling) in [(&arms[0].1, "* $name"), (&arms[1].1, "(* $name)")] {
            assert!(!body.contains("(@native [$name])"), "{body}");
            assert_eq!(body.matches(&format!("(@deref [{spelling}] [$name])")).count(), 2);
        }
        assert_eq!(arms[2].1, format!("@value {witness}"));
        let other = "$crate::__pgrx_c_operand!(@read_place; (@native [$other]))";
        assert_eq!(shell(&format!("{{ f({place}, {other}) }}")).arms().len(), 1);
    }
}
