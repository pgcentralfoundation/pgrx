//LICENSE Portions Copyright 2026 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.

//! Ordered C statements share the expression arena and preserve lexical blocks.
//!
//! The statement renderer uses the same contextual expression lowering as value
//! macros, retaining ordered effects, declarations, nested scopes, and returns.
//! Delegation is limited to proved whole-body wrappers, and a token guard rejects
//! arguments whose local-name capture would differ under Rust macro hygiene.

use super::{BindingCatalog, INLINE_LABEL, SUPPORT, macro_identifier, typed, write_fallback};
use crate::{AnalysisSession, MacroAnalysis, SkipReason, syntax::Statement};
use std::fmt::Write;

/// What a C `return` statement does in the lowered body.
#[derive(Clone, Copy)]
pub(super) enum Returns<'a> {
    /// Exit the Rust function or closure that invoked the macro, converting to the
    /// explicit C marker when one is given.
    Caller(Option<&'a str>),
    /// Yield the value of a translated inline function: break out of the labeled block
    /// holding its body, converted to the function's C return type marker.
    Value(&'a str),
}

/// Lower an entire admitted statement body, preserving ordered effects and return context.
///
/// Cross-macro calls remain visible only when delegation proves equivalence of the
/// whole simple wrapper; other dependencies receive an expansion explanation.
pub(super) fn render(
    session: &AnalysisSession<'_>,
    analysis: &MacroAnalysis,
    bindings: &BindingCatalog,
    renderer: &typed::Renderer<'_>,
    returns: Returns<'_>,
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
    if let Returns::Caller(explicit_marker) = returns
        && let Ok(Some(delegation)) = crate::delegation::direct_delegation(session, analysis)
        && bindings.macros.contains(&delegation.callee)
        && let Some(callee) = macro_identifier(&delegation.callee)
    {
        let mode = if let Some(marker) = explicit_marker {
            format!("return_as; ${marker}, ")
        } else {
            String::from("public; ")
        };
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
    render_statements(&body.statements, renderer, returns, &mut rust)?;
    rust.push('}');
    Ok(rust)
}

/// Emit declarations, expressions, blocks, branches, and returns in original statement order.
fn render_statements(
    statements: &[Statement],
    renderer: &typed::Renderer<'_>,
    returns: Returns<'_>,
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
                render_statements(statements, renderer, returns, rust)?;
                rust.push_str("} ");
            }
            Statement::Return { expression, .. } => {
                match returns {
                    Returns::Caller(Some(marker)) => write!(
                        rust,
                        "return {SUPPORT}::expression_result::return_value_as::<${marker}, _>("
                    ),
                    Returns::Caller(None) => {
                        write!(rust, "return {SUPPORT}::expression_result::return_value(")
                    }
                    Returns::Value(marker) => write!(
                        rust,
                        "break '{INLINE_LABEL} {SUPPORT}::expression::implicit::<{marker}, _>("
                    ),
                }
                .expect("String output");
                renderer.render(*expression, typed::Context::Value, rust)?;
                rust.push_str("); ");
            }
            Statement::If { condition, then_branch, else_branch, .. } => {
                write!(rust, "if {SUPPORT}::expression::truth(").expect("String output");
                renderer.render(*condition, typed::Context::Value, rust)?;
                rust.push_str(") { ");
                render_statements(
                    std::slice::from_ref(then_branch.as_ref()),
                    renderer,
                    returns,
                    rust,
                )?;
                rust.push_str("} ");
                if let Some(else_branch) = else_branch {
                    rust.push_str("else { ");
                    render_statements(
                        std::slice::from_ref(else_branch.as_ref()),
                        renderer,
                        returns,
                        rust,
                    )?;
                    rust.push_str("} ");
                }
            }
        }
    }
    Ok(())
}

/// Inspect stringified tokens before normalization, including forwarded opaque
/// expression fragments. Field, path and string matches are conservative skips.
pub(super) fn guard(
    analysis: &MacroAnalysis,
    normalizer: Option<&str>,
) -> Result<Option<String>, SkipReason> {
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
    if names.is_empty() {
        return Ok(None);
    }
    let Some(normalizer) = normalizer else {
        return Ok(None);
    };
    let mut rust =
        String::from("(@__pgrx_c_guard_locals $mode:ident [$($done:tt)*]; $($raw:tt)*) => {{ ");
    let message = format!(
        "C macro argument mentions local {}; C substitution and Rust hygiene would resolve it differently, or the 4096-byte scope-check bound was exceeded",
        names.join(", ")
    );
    write!(rust, "const _: () = ::core::assert!({SUPPORT}::statements::local_scope_allowed(::core::stringify!($($raw)*), &{names:?}), {message:?}); ").expect("String output");
    writeln!(rust, "$crate::{normalizer}!(@collect $mode [$($done)*]; $($raw)*) }}}};")
        .expect("String output");
    Ok(Some(rust))
}
