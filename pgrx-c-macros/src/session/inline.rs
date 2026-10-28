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
//! and documentation.
//!
//! When the definition's body uses no macro, it is also offered to the analyzer as a
//! private statement root that declares each parameter from its hole. The analyzer
//! translates that root like a C macro body when it can; otherwise the call root
//! remains. Neither root is added to the active C preprocessor definitions.
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
        /// The definition's body as a statement macro that declares each parameter as a
        /// local initialized from its hole, when the body's tokens can be analyzed.
        body: Option<Box<ActiveMacro>>,
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
            Self::Ready { call, formals, body, .. } => {
                // Prefer translating the definition itself. Its analysis must keep exactly
                // the function's parameters, since a capture would mean an unresolved name,
                // and every path must return a value.
                if let Some(body) = body {
                    let analysis = analyze_root(frontend, name, body, formals, constants);
                    if matches!(analysis.status, AnalysisStatus::Candidate)
                        && analysis.parameters.len() == formals.len()
                        && analysis.expression.as_ref().is_some_and(|expression| {
                            expression
                                .syntax
                                .statement_body
                                .as_ref()
                                .is_some_and(|body| body.always_returns)
                        })
                    {
                        return analysis;
                    }
                }
                analyze_root(frontend, name, call, formals, constants)
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

    /// Analyze the native call root even when the definition itself is translatable.
    pub(super) fn analyze_call(
        &self,
        frontend: &FrontendOutput,
        name: &str,
        constants: &BTreeMap<String, IntegerConstant>,
    ) -> Option<MacroAnalysis> {
        match self {
            Self::Ready { call, formals, .. } => {
                Some(analyze_root(frontend, name, call, formals, constants))
            }
            Self::Skipped(_) => None,
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

/// Analyze one private root and restore the definition's parameter labels.
fn analyze_root(
    frontend: &FrontendOutput,
    name: &str,
    root: &ActiveMacro,
    formals: &[String],
    constants: &BTreeMap<String, IntegerConstant>,
) -> MacroAnalysis {
    let mut analysis =
        crate::analysis::analyze_active_with_constants(frontend, name, Some(root), constants, true);
    for (parameter, name) in analysis.parameters.iter_mut().zip(formals) {
        parameter.name.clone_from(name);
    }
    analysis
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
    // Body tokens are unexpanded, and a macro may have been replaced or undefined after
    // the definition, so any name ever defined as a macro makes a body untranslatable.
    let macros = frontend
        .environment()
        .active
        .keys()
        .map(String::as_str)
        .chain(frontend.inventory().macros.iter().map(|definition| definition.name.as_str()))
        .collect::<BTreeSet<_>>();
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
        let body_identifiers = source
            .body
            .iter()
            .flatten()
            .filter(|token| token.kind == TokenKind::Identifier)
            .map(|token| token.spelling.as_str())
            .collect::<BTreeSet<_>>();
        let occupied = |identifier: &str| {
            identifier == name
                || body_identifiers.contains(identifier)
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
        // A void function yields no value, so only value-returning bodies are translated.
        // A body over the shared budgets leaves only the call root.
        let body = source
            .body
            .as_ref()
            .filter(|_| function.signature.result.category != crate::TypeCategory::Void)
            .and_then(|statements| {
                body_tokens(name, &holes, source, parameters, statements, &macros)
            })
            .filter(|tokens| {
                let added = tokens.iter().map(|token| token.spelling.len()).sum::<usize>();
                let fits = tokens.len() <= limits.macro_tokens
                    && added <= limits.source_bytes.saturating_sub(bytes);
                if fits {
                    bytes += added;
                }
                fits
            });
        result.insert(
            name.clone(),
            InlineRoot::Ready {
                body: body.map(|tokens| {
                    Box::new(ActiveMacro {
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
                    })
                }),
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

/// Build `NAME(holes...) do { T0 p0 = (hole0); ...; statements } while (0)`.
///
/// Each parameter becomes a local initialized from its hole, so every argument is
/// evaluated exactly once and converted to its parameter type by C assignment rules,
/// as in a call. Parameters and the body's top-level declarations share one block
/// scope, as they do in C. The body is refused when a parameter is unnamed or has a
/// type that cannot be spelled as a flat declaration, or when any token names a macro.
fn body_tokens(
    name: &str,
    holes: &[String],
    source: &InlineFunctionDefinition,
    parameters: &[crate::TypeInfo],
    statements: &[Token],
    macros: &BTreeSet<&str>,
) -> Option<Vec<Token>> {
    let token = |kind, spelling: &str| Token { kind, spelling: spelling.into() };
    let mut tokens = vec![token(TokenKind::Identifier, name), token(TokenKind::Punctuation, "(")];
    for (index, hole) in holes.iter().enumerate() {
        if index != 0 {
            tokens.push(token(TokenKind::Punctuation, ","));
        }
        tokens.push(token(TokenKind::Identifier, hole));
    }
    tokens.extend([
        token(TokenKind::Punctuation, ")"),
        token(TokenKind::Keyword, "do"),
        token(TokenKind::Punctuation, "{"),
    ]);
    for ((parameter, ty), hole) in source.parameters.iter().zip(parameters).zip(holes) {
        let parameter = parameter.as_ref()?;
        if ty.spelling.contains(['(', '[', ',']) {
            return None;
        }
        for word in ty.spelling.replace('*', " * ").split_whitespace() {
            let kind = match word {
                "*" => TokenKind::Punctuation,
                "const" | "volatile" | "restrict" | "struct" | "union" | "enum" | "unsigned"
                | "signed" | "char" | "short" | "int" | "long" | "float" | "double" | "void"
                | "_Bool" => TokenKind::Keyword,
                _ => TokenKind::Identifier,
            };
            tokens.push(token(kind, word));
        }
        tokens.extend([
            token(TokenKind::Identifier, parameter),
            token(TokenKind::Punctuation, "="),
            token(TokenKind::Punctuation, "("),
            token(TokenKind::Identifier, hole),
            token(TokenKind::Punctuation, ")"),
            token(TokenKind::Punctuation, ";"),
        ]);
    }
    tokens.extend(statements.iter().cloned());
    if tokens[1..].iter().any(|token| {
        token.kind == TokenKind::Identifier && macros.contains(token.spelling.as_str())
    }) {
        return None;
    }
    tokens.extend([
        token(TokenKind::Punctuation, "}"),
        token(TokenKind::Keyword, "while"),
        token(TokenKind::Punctuation, "("),
        token(TokenKind::Literal, "0"),
        token(TokenKind::Punctuation, ")"),
    ]);
    Some(tokens)
}
