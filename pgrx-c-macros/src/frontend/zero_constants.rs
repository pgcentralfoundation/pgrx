//LICENSE Portions Copyright 2026 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.

//! Retain source-level null-constant identity without folding generated operations.

use super::{FrontendError, driver_arguments, run_compiler_with_input};
use crate::{AnalysisSession, ExpressionKind, MacroScanner, NodeId, TypeCategory, TypeExpression};
use clang::{EntityKind, EntityVisitResult};
use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Write;

const MAX_CANDIDATES: usize = 16_384;
const MAX_SOURCE_BYTES: usize = 1024 * 1024;
const MAX_TOTAL_SOURCE_BYTES: usize = 8 * 1024 * 1024;
const MAX_PROBE_RUNS: usize = 64;

struct Candidate {
    macro_name: String,
    nodes: Vec<NodeId>,
    expression: String,
}

pub(crate) fn probe(
    scanner: &MacroScanner,
    session: &AnalysisSession<'_>,
) -> Result<BTreeMap<String, BTreeSet<NodeId>>, FrontendError> {
    let mut candidates = Vec::new();
    let mut total_source_bytes = 0usize;
    // Each node and each selected source token is visited once. Only maximal
    // pure subtrees are copied, rather than every overlapping ancestor span.
    for (name, expansion) in &session.expansions().results {
        let crate::ExpansionResult::Expanded { expansion } = expansion else { continue };
        let analysis = session.analyze(name);
        let Some(expression) = &analysis.expression else { continue };
        let mut pure = Vec::with_capacity(expression.syntax.nodes.len());
        let mut enclosed = vec![false; expression.syntax.nodes.len()];
        let mut constant_nodes = vec![false; expression.syntax.nodes.len()];
        for constant in &expression.constants {
            constant_nodes[constant.node] = true;
        }
        for (index, node) in expression.syntax.nodes.iter().enumerate() {
            let mut children = [None; 3];
            let allowed = match &node.kind {
                ExpressionKind::IntegerLiteral { .. } => true,
                ExpressionKind::Identifier { .. } => constant_nodes[index],
                ExpressionKind::Group { operand } | ExpressionKind::Unary { operand, .. } => {
                    children[0] = Some(*operand);
                    pure[*operand]
                }
                ExpressionKind::Cast { type_name, operand } => {
                    children[0] = Some(*operand);
                    pure[*operand]
                        && crate::analysis::resolve_type_info(
                            type_name,
                            session.frontend().declarations(),
                            &session.frontend().profile().target,
                        )
                        .is_some_and(|ty| matches!(ty.category, TypeCategory::Integer(_)))
                }
                ExpressionKind::Binary { left, right, .. } => {
                    children[0] = Some(*left);
                    children[1] = Some(*right);
                    pure[*left] && pure[*right]
                }
                ExpressionKind::Conditional { condition, then_value, else_value } => {
                    children = [Some(*condition), Some(*then_value), Some(*else_value)];
                    pure[*condition] && pure[*then_value] && pure[*else_value]
                }
                _ => false,
            };
            let allowed = allowed
                && matches!(
                    expression.types[index],
                    TypeExpression::Concrete { .. }
                        | TypeExpression::Promotion { .. }
                        | TypeExpression::Common { .. }
                );
            if allowed {
                for child in children.into_iter().flatten() {
                    enclosed[child] = true;
                }
            }
            pure.push(allowed);
        }
        // Successful analysis already proved the flat function-macro signature.
        let body_start = expansion
            .definition
            .tokens
            .iter()
            .position(|token| token.spelling == ")")
            .expect("candidate function-macro signature")
            + 1;
        let tokens = &expansion.definition.tokens[body_start..];
        for (index, node) in expression.syntax.nodes.iter().enumerate() {
            if !pure[index] || enclosed[index] {
                continue;
            }
            // Simple zeros already have a local syntactic proof. They need no
            // compiler round trip; grouping alone does not make them compound.
            let mut inner = index;
            let mut equivalent_groups = vec![index];
            while let ExpressionKind::Group { operand } = expression.syntax.nodes[inner].kind {
                inner = operand;
                equivalent_groups.push(inner);
            }
            if matches!(
                expression.syntax.nodes[inner].kind,
                ExpressionKind::IntegerLiteral { .. } | ExpressionKind::Identifier { .. }
            ) {
                continue;
            }
            let mut source = String::new();
            for token in &tokens[node.tokens.start..node.tokens.end] {
                if token.kind == crate::TokenKind::Comment {
                    continue;
                }
                if !source.is_empty() {
                    source.push(' ');
                }
                source.push_str(&token.spelling);
            }
            total_source_bytes = total_source_bytes.saturating_add(source.len());
            if source.len() > MAX_SOURCE_BYTES
                || total_source_bytes > MAX_TOTAL_SOURCE_BYTES
                || candidates.len() >= MAX_CANDIDATES
            {
                return Err(FrontendError::Output(
                    "integer ICE proofs exceed their source/count budget".into(),
                ));
            }
            candidates.push(Candidate {
                macro_name: name.clone(),
                nodes: equivalent_groups,
                expression: source,
            });
        }
    }
    let mut result = BTreeMap::<String, BTreeSet<NodeId>>::new();
    let mut pending = Vec::new();
    let mut start = 0;
    let mut bytes = 0usize;
    for (index, candidate) in candidates.iter().enumerate() {
        let next = candidate.expression.len().saturating_add(80);
        if bytes.saturating_add(next) > MAX_SOURCE_BYTES / 2 && index > start {
            pending.push((start, index));
            start = index;
            bytes = 0;
        }
        bytes += next;
    }
    if start < candidates.len() {
        pending.push((start, candidates.len()));
    }
    let mut runs = 0;
    while let Some((start, end)) = pending.pop() {
        runs += 1;
        if runs > MAX_PROBE_RUNS {
            return Err(FrontendError::Output(
                "integer ICE proofs exceed the 64-run budget".into(),
            ));
        }
        let source = source(session, &candidates[start..end])?;
        match collect(scanner, session, &source) {
            Ok(zeros) => {
                for index in zeros {
                    let candidate = &candidates[start + index];
                    result
                        .entry(candidate.macro_name.clone())
                        .or_default()
                        .extend(candidate.nodes.iter().copied());
                }
            }
            Err(
                FrontendError::CompilerFailed { .. }
                | FrontendError::Discovery(crate::Error::Diagnostics(_)),
            ) => {
                if end - start > 1 {
                    let middle = start + (end - start) / 2;
                    pending.extend([(start, middle), (middle, end)]);
                }
            }
            Err(error) => return Err(error),
        }
    }
    Ok(result)
}

fn source(
    session: &AnalysisSession<'_>,
    candidates: &[Candidate],
) -> Result<String, FrontendError> {
    let header = session
        .frontend()
        .profile()
        .header
        .to_str()
        .ok_or_else(|| FrontendError::Output("integer ICE probe header is not UTF-8".into()))?;
    if header.chars().any(|character| matches!(character, '\n' | '\r' | '\0')) {
        return Err(FrontendError::Output("invalid integer ICE probe header path".into()));
    }
    let header = header.replace('\\', "\\\\").replace('"', "\\\"");
    let mut source = format!("#include \"{header}\"\n");
    for (index, candidate) in candidates.iter().enumerate() {
        writeln!(
            source,
            "enum {{ __pgrx_integer_zero_{index} = (({}) == 0) }};\nconst int __pgrx_integer_zero_driver_{index} = __pgrx_integer_zero_{index};",
            candidate.expression
        )
        .expect("String output");
        if source.len() > MAX_SOURCE_BYTES {
            return Err(FrontendError::Output(
                "integer ICE proof exceeds its 1 MiB source budget".into(),
            ));
        }
    }
    Ok(source)
}

fn collect(
    scanner: &MacroScanner,
    session: &AnalysisSession<'_>,
    source: &str,
) -> Result<BTreeSet<usize>, FrontendError> {
    let profile = session.frontend().profile();
    let mut arguments = driver_arguments(
        &profile.arguments,
        &["-S", "-emit-llvm", "-O1", "-o", "-", "-Werror=integer-overflow"],
        None,
    );
    arguments.push("-".into());
    let driver =
        run_compiler_with_input(&profile.compiler.executable, &arguments, Some(source.to_owned()))?
            .stdout;
    let mut driver_zeros = BTreeSet::<usize>::new();
    let driver_type =
        format!("constant i{} ", profile.target.integers[&crate::IntegerKind::Int].bits);
    for line in driver.lines() {
        let Some((name, value)) = line
            .strip_prefix("@__pgrx_integer_zero_driver_")
            .and_then(|line| line.split_once(" = "))
        else {
            continue;
        };
        let Some((_, value)) = value
            .split_once(&format!(" {driver_type}"))
            .or_else(|| value.strip_prefix(&driver_type).map(|value| ("", value)))
        else {
            continue;
        };
        if value.split(',').next() == Some("1")
            && let Ok(index) = name.parse()
        {
            driver_zeros.insert(index);
        }
    }
    let header = std::env::temp_dir().join("pgrx-c-macros-integer-zeros.h");
    scanner
        .with_declarations(&header, &profile.arguments, Some(source), |unit| {
            let mut zeros = BTreeSet::new();
            unit.get_entity().visit_children(|entity, _| {
                if entity.get_kind() == EntityKind::EnumDecl {
                    entity.visit_children(|constant, _| {
                        if let Some(index) = constant.get_name().and_then(|name| {
                            name.strip_prefix("__pgrx_integer_zero_")
                                .and_then(|suffix| suffix.parse().ok())
                        }) && constant
                            .get_enum_constant_value()
                            .is_some_and(|(value, _)| value == 1)
                        {
                            zeros.insert(index);
                        }
                        EntityVisitResult::Continue
                    });
                }
                EntityVisitResult::Continue
            });
            // Neither compiler's result substitutes for the other's witness.
            zeros.retain(|index| driver_zeros.contains(index));
            Ok(zeros)
        })
        .map_err(FrontendError::from)
}
