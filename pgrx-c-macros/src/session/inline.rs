//LICENSE Portions Copyright 2026 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.

//! Derive explicit invocation roots from compiler-owned original inline definitions.
//!
//! These private immutable session roots remain separate from the real active macro
//! map and its compiler expansion results. A verified original C prototype and full
//! definition supply protected `(name)(arguments...)` call syntax to the common
//! analyzer. Private formal holes prevent a caller label from capturing the callee;
//! actual formals, physical provenance and source are retained separately for reports
//! and documentation. No function body is translated into Rust or fabricated as an
//! active C preprocessor definition.
//!
//! The existing typed renderer and native capability planner own parameter assignment
//! conversions, qualifiers, original result identity and guarded ABI calls. Native
//! operands evaluate once, while actual macro roots retain their substitution rules
//! and take precedence on name collisions. Unsupported prototype, convention, storage,
//! source and shared count/token/byte budgets use the same structured refusal path;
//! selection does not bypass profile or input-consistency checks.

use crate::{
    ActiveMacro, ActiveProvenance, AnalysisStatus, ExpansionLimits, FrontendOutput,
    InlineFunctionDefinition, IntegerConstant, MacroAnalysis, MacroDefinition, MacroKind,
    SkipReason, SkipReasonCode, Token, TokenKind,
};
use std::collections::{BTreeMap, BTreeSet};

/// Retain original source separately from the private call syntax consumed by the common analyzer.
pub(super) enum InlineRoot<'a> {
    /// A proved fixed native call, with actual definition labels retained for reporting.
    Ready {
        /// Borrow the verified physical C source without duplicating retained header bodies.
        source: &'a InlineFunctionDefinition,
        /// Private formal holes and protected native call; never inserted into the macro environment.
        call: ActiveMacro,
        /// Actual definition labels, with deterministic labels for unnamed parameters.
        formals: Vec<String>,
    },
    /// A selected function lacks the exact native proof or fits outside the bounded call budget.
    Skipped(SkipReason),
}

/// Analyze private call holes before restoring original labels so a formal cannot capture its callee.
impl InlineRoot<'_> {
    /// Use the common expression/type analyzer for the exact compiler-prototype-backed call.
    pub(super) fn analyze(
        &self,
        frontend: &FrontendOutput,
        name: &str,
        constants: &BTreeMap<String, IntegerConstant>,
    ) -> MacroAnalysis {
        match self {
            Self::Ready { call, formals, .. } => {
                let mut analysis = crate::analysis::analyze_active_with_constants(
                    frontend,
                    name,
                    Some(call),
                    constants,
                    true,
                );
                for (parameter, name) in analysis.parameters.iter_mut().zip(formals) {
                    parameter.name.clone_from(name);
                }
                analysis
            }
            Self::Skipped(reason) => {
                let mut analysis = crate::analyze(frontend, name);
                analysis.provenance = reason.spans.first().cloned();
                analysis.expression = None;
                analysis.status = AnalysisStatus::Skipped { reason: reason.clone() };
                analysis
            }
        }
    }

    /// Expose actual function source only for an admitted private call root.
    pub(super) fn source(&self) -> Option<&InlineFunctionDefinition> {
        match self {
            Self::Ready { source, .. } => Some(source),
            Self::Skipped(_) => None,
        }
    }
}

/// Construct bounded native call syntax from actual prototypes, without preprocessing a fake macro.
pub(super) fn prepare<'a>(
    frontend: &'a FrontendOutput,
    names: &[String],
    limits: ExpansionLimits,
    macro_roots: usize,
) -> BTreeMap<String, InlineRoot<'a>> {
    let mut result = BTreeMap::new();
    let mut bytes = 0_usize;
    for name in names {
        if frontend.environment().active.contains_key(name) {
            continue;
        }
        let declaration = frontend.declarations().function_signatures.get(name);
        let source = declaration.and_then(|function| function.definition.as_ref());
        let refusal = |code, message: &str| {
            InlineRoot::Skipped(SkipReason {
                code,
                message: message.into(),
                tokens: None,
                spans: source.into_iter().map(|source| source.provenance.clone()).collect(),
            })
        };
        let Some(function) = declaration.filter(|function| {
            function.is_static
                && function.is_inline
                && function.definition_available
                && function.linkage == Some(crate::DeclarationLinkage::Internal)
                && source.is_some()
        }) else {
            result.insert(
                name.clone(),
                refusal(
                    SkipReasonCode::Call,
                    "invocation root requires an original compiler-proven static inline definition",
                ),
            );
            continue;
        };
        let Some(parameters) = &function.signature.parameters else {
            result.insert(
                name.clone(),
                refusal(SkipReasonCode::Call, "inline function has no complete C prototype"),
            );
            continue;
        };
        if function.signature.variadic {
            result.insert(
                name.clone(),
                refusal(
                    SkipReasonCode::Variadic,
                    "variadic calls require a separate argument promotion contract",
                ),
            );
            continue;
        }
        if function.signature.calling_convention.as_deref() != Some("Cdecl") {
            result.insert(
                name.clone(),
                refusal(
                    SkipReasonCode::Call,
                    "inline function calling convention is not the verified default C convention",
                ),
            );
            continue;
        }
        let source = source.expect("admission requires verified physical source");
        if source.parameters.len() != parameters.len() {
            result.insert(
                name.clone(),
                refusal(
                    SkipReasonCode::Call,
                    "original inline formals differ from the complete C prototype",
                ),
            );
            continue;
        }
        if result.len().saturating_add(macro_roots) >= limits.macros
            || parameters.len().saturating_mul(4).saturating_add(8) > limits.macro_tokens
        {
            result.insert(
                name.clone(),
                refusal(
                    SkipReasonCode::BudgetExceeded,
                    "inline invocation roots exceed the shared count or token budget",
                ),
            );
            continue;
        }
        let mut used = source.parameters.iter().flatten().cloned().collect::<BTreeSet<_>>();
        if used.len() != source.parameters.iter().flatten().count() {
            result.insert(
                name.clone(),
                refusal(
                    SkipReasonCode::Call,
                    "original inline definition contains conflicting parameter labels",
                ),
            );
            continue;
        }
        let declarations = frontend.declarations();
        let occupied = |identifier: &str| {
            identifier == name
                || frontend.environment().active.contains_key(identifier)
                || declarations.function_signatures.contains_key(identifier)
                || declarations.variables.contains_key(identifier)
                || declarations.types.contains_key(identifier)
                || declarations.integer_constants.contains_key(identifier)
                || used.contains(identifier)
        };
        let holes = (0..parameters.len())
            .map(|index| {
                let mut hole = format!("__pgrx_inline_parameter_{index}");
                while occupied(&hole) {
                    hole.push('_');
                }
                hole
            })
            .collect::<Vec<_>>();
        let token = |kind, spelling: &str| Token { kind, spelling: spelling.into() };
        let identifier = |spelling: &str| token(TokenKind::Identifier, spelling);
        let punctuation = |spelling: &str| token(TokenKind::Punctuation, spelling);
        let mut tokens = vec![identifier(name), punctuation("(")];
        for (index, hole) in holes.iter().enumerate() {
            if index != 0 {
                tokens.push(punctuation(","));
            }
            tokens.push(identifier(hole));
        }
        tokens.extend([
            punctuation(")"),
            punctuation("("),
            identifier(name),
            punctuation(")"),
            punctuation("("),
        ]);
        for (index, hole) in holes.iter().enumerate() {
            if index != 0 {
                tokens.push(punctuation(","));
            }
            tokens.push(identifier(hole));
        }
        tokens.push(punctuation(")"));
        let added = tokens.iter().map(|token| token.spelling.len()).sum::<usize>();
        if added > limits.source_bytes.saturating_sub(bytes) {
            result.insert(
                name.clone(),
                refusal(
                    SkipReasonCode::BudgetExceeded,
                    "inline call source exceeds the shared source budget",
                ),
            );
            continue;
        }
        bytes += added;
        let formals = source
            .parameters
            .iter()
            .enumerate()
            .map(|(index, name)| {
                name.clone().unwrap_or_else(|| {
                    let mut label = format!("arg{index}");
                    while used.contains(&label) {
                        label.push('_');
                    }
                    used.insert(label.clone());
                    label
                })
            })
            .collect();
        result.insert(
            name.clone(),
            InlineRoot::Ready {
                source,
                call: ActiveMacro {
                    definition: MacroDefinition {
                        name: name.clone(),
                        kind: MacroKind::FunctionLike,
                        location: None,
                        provenance: Some(source.provenance.clone()),
                        tokens,
                        builtin: false,
                        main_file: false,
                    },
                    provenance: ActiveProvenance::Resolved,
                },
                formals,
            },
        );
    }
    result
}
