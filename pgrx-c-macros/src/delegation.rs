//LICENSE Portions Copyright 2026 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.

//! Recover direct macro calls from compiler-expanded, analyzed expression trees.
//!
//! The original tokens establish only the call's name and arity. Clang has already
//! performed argument prescan and replacement-list rescan for both expressions.
//! Matching the callee tree against the caller tree proves that substitution of
//! the recovered argument subtrees produces the same C expression. No unexpanded
//! token is translated or treated as an independently typed expression.

//! Direct macro wrappers can retain calls to generated callee macros instead of duplicating
//! their fully expanded bodies. This module compares independently expanded expression trees
//! and C type facts, recovering arguments only when every occurrence agrees. Cycles, unused
//! arguments that cannot be recovered, and statement bodies outside the proof are rejected,
//! leaving ordinary emission to preserve the compiler-expanded definition.

use crate::{
    AnalysisSession, AnalysisStatus, AnalyzedExpression, ExpressionKind, MacroAnalysis,
    MacroDefinition, MacroKind, NodeId, TokenKind, TypeExpression,
};

/// Argument roots refer to the caller's trusted analyzed expression arena.
pub(crate) struct Delegation {
    /// The generated macro or callable identity whose arguments are retained in original order.
    pub callee: String,
    /// Caller arena nodes recovered consistently for every occurrence of each callee formal.
    pub arguments: Vec<NodeId>,
}

/// Recognize an entire replacement list which is a call to another active macro.
///
/// The expression trees retain each parameter occurrence and every lazy branch.
/// A recovered argument is inserted into the Rust callee as an expression fragment,
/// without evaluating it first. Its occurrences therefore follow the callee's
/// substitution and lazy evaluation, just as in the matched original C tree.
/// Repeated holes must match identical caller subtrees, including C type identity.
///
/// Both parsers bound their trees and depth. Matching visits disjoint argument
/// subtrees; repeated-argument comparisons revisit the first occurrence at most
/// once per corresponding occurrence. Time and temporary storage are linear in
/// the combined expanded trees and the original caller replacement tokens.
pub(crate) fn direct_delegation(
    session: &AnalysisSession<'_>,
    caller: &MacroAnalysis,
) -> Result<Option<Delegation>, String> {
    let Some(active) = session.frontend().environment().active.get(&caller.name) else {
        return Ok(None);
    };
    let Some((callee_name, arity)) = direct_call(&active.definition)? else {
        return Ok(None);
    };
    if caller.parameters.iter().any(|parameter| parameter.name == callee_name) {
        return Err("callee name is a macro parameter rather than a fixed C binding".into());
    }
    let Some(callee_active) = session.frontend().environment().active.get(callee_name) else {
        return Err(format!("{callee_name} is not an active function-like macro"));
    };
    if callee_active.definition.kind != MacroKind::FunctionLike {
        return Err(format!("{callee_name} is an object-like macro rather than a direct callee"));
    }
    let callee = session.analyze(callee_name);
    if !matches!(callee.status, AnalysisStatus::Candidate) {
        return Err(format!("{callee_name} has no supported prepared macro body"));
    }
    if callee.name == caller.name
        || callee.dependencies.iter().any(|dependency| dependency.name == caller.name)
    {
        return Err(format!("{callee_name} may recursively depend on {}", caller.name));
    }
    if callee.parameters.len() != arity {
        return Err(format!("{callee_name} call arity differs from its analyzed signature"));
    }
    let (Some(pattern), Some(candidate)) = (&callee.expression, &caller.expression) else {
        return Err("direct call has no complete compiler-analyzed expression".into());
    };
    if pattern.syntax.statement_body.is_some() != candidate.syntax.statement_body.is_some()
        || [&pattern.syntax, &candidate.syntax].iter().any(|syntax| {
            syntax.statement_body.as_ref().is_some_and(|body| {
                body.return_tokens.is_none()
                    || body.walk().any(|statement| {
                        !matches!(
                            statement,
                            crate::Statement::Return { .. } | crate::Statement::Block { .. }
                        )
                    })
            })
        })
    {
        return Err(
            "root expression matching does not establish complete statement-body equivalence"
                .into(),
        );
    }
    let mut arguments = vec![None; arity];
    let mut pending =
        vec![(ungroup(pattern, pattern.syntax.root), ungroup(candidate, candidate.syntax.root))];
    while let Some((left, right)) = pending.pop() {
        if let ExpressionKind::Parameter { index } = pattern.syntax.nodes[left].kind {
            let Some(argument) = arguments.get_mut(index) else {
                return Err("callee parameter index is outside its analyzed signature".into());
            };
            match *argument {
                None => *argument = Some(right),
                Some(previous) if equivalent_subtree(candidate, previous, right) => {}
                Some(_) => {
                    return Err(format!(
                        "{callee_name} parameter {index} expands differently across occurrences"
                    ));
                }
            }
        } else if !match_node(pattern, left, candidate, right, &mut pending) {
            return Err(format!(
                "{callee_name} expression structure differs after compiler expansion"
            ));
        }
    }
    let arguments = arguments.into_iter().collect::<Option<Vec<_>>>().ok_or_else(|| {
        format!("{callee_name} has an unused argument whose expression cannot be recovered")
    })?;
    Ok(Some(Delegation { callee: callee_name.to_owned(), arguments }))
}

/// Remove explicit group nodes only for structural comparison of independently expanded expressions.
fn ungroup(expression: &AnalyzedExpression, mut node: NodeId) -> NodeId {
    while let ExpressionKind::Group { operand } = expression.syntax.nodes[node].kind {
        node = operand;
    }
    node
}

/// Check whether repeated recovered arguments have the same supported structure and C facts.
fn equivalent_subtree(expression: &AnalyzedExpression, left: NodeId, right: NodeId) -> bool {
    let mut pending = vec![(left, right)];
    while let Some((left, right)) = pending.pop() {
        if !match_node(expression, left, expression, right, &mut pending) {
            return false;
        }
    }
    true
}

/// Compare one pair of expression nodes and enqueue children without recursively walking deep trees.
fn match_node(
    pattern: &AnalyzedExpression,
    left: NodeId,
    candidate: &AnalyzedExpression,
    right: NodeId,
    pending: &mut Vec<(NodeId, NodeId)>,
) -> bool {
    match (&pattern.syntax.nodes[left].kind, &candidate.syntax.nodes[right].kind) {
        (
            ExpressionKind::OffsetOf { record: left_record, fields: left_fields },
            ExpressionKind::OffsetOf { record: right_record, fields: right_fields },
        ) => {
            left_record == right_record
                && left_fields == right_fields
                && same_concrete_type(pattern, left, candidate, right)
        }
        (ExpressionKind::Parameter { index: left }, ExpressionKind::Parameter { index: right }) => {
            left == right
        }
        (
            ExpressionKind::IntegerLiteral { literal: left_literal },
            ExpressionKind::IntegerLiteral { literal: right_literal },
        ) => left_literal == right_literal && same_concrete_type(pattern, left, candidate, right),
        (
            ExpressionKind::StringLiteral { bytes: left_bytes },
            ExpressionKind::StringLiteral { bytes: right_bytes },
        ) => left_bytes == right_bytes,
        (ExpressionKind::InvocationFile, ExpressionKind::InvocationFile)
        | (ExpressionKind::InvocationLine, ExpressionKind::InvocationLine) => true,
        (
            ExpressionKind::Identifier { name: left_name },
            ExpressionKind::Identifier { name: right_name },
        ) => {
            // Each identifier's analyzed constant comes from the immutable compiler
            // catalog. Name plus C identity therefore implies the same resolved value.
            left_name == right_name && same_concrete_type(pattern, left, candidate, right)
        }
        (ExpressionKind::Group { operand: left }, ExpressionKind::Group { operand: right }) => {
            pending.push((*left, *right));
            true
        }
        (
            ExpressionKind::Unary { operator: left_operator, operand: left },
            ExpressionKind::Unary { operator: right_operator, operand: right },
        ) => {
            pending.push((*left, *right));
            left_operator == right_operator
        }
        (
            ExpressionKind::Binary {
                operator: left_operator,
                left: left_operand,
                right: left_other,
            },
            ExpressionKind::Binary {
                operator: right_operator,
                left: right_operand,
                right: right_other,
            },
        ) => {
            pending.extend([(*left_operand, *right_operand), (*left_other, *right_other)]);
            left_operator == right_operator
        }
        (
            ExpressionKind::Cast { type_name: left_name, operand: left_operand },
            ExpressionKind::Cast { type_name: right_name, operand: right_operand },
        ) => {
            pending.push((*left_operand, *right_operand));
            left_name == right_name && same_concrete_type(pattern, left, candidate, right)
        }
        (
            ExpressionKind::Conditional {
                condition: left_condition,
                then_value: left_then,
                else_value: left_else,
            },
            ExpressionKind::Conditional {
                condition: right_condition,
                then_value: right_then,
                else_value: right_else,
            },
        ) => {
            pending.extend([
                (*left_condition, *right_condition),
                (*left_then, *right_then),
                (*left_else, *right_else),
            ]);
            true
        }
        _ => false,
    }
}

/// Require matching C integer identities when structural equivalence depends on a concrete constant
/// or cast.
fn same_concrete_type(
    pattern: &AnalyzedExpression,
    left: NodeId,
    candidate: &AnalyzedExpression,
    right: NodeId,
) -> bool {
    matches!((&pattern.types[left], &candidate.types[right]),
        (TypeExpression::Concrete { ty: left }, TypeExpression::Concrete { ty: right }) if left == right)
}

/// Recognize a replacement consisting entirely of one function-macro invocation and count its
/// top-level arguments.
fn direct_call(definition: &MacroDefinition) -> Result<Option<(&str, usize)>, String> {
    let tokens = definition
        .tokens
        .iter()
        .filter(|token| token.kind != TokenKind::Comment)
        .collect::<Vec<_>>();
    if tokens.get(1).is_none_or(|token| token.spelling != "(") {
        return Ok(None);
    }
    let Some(signature_end) = tokens.iter().skip(2).position(|token| token.spelling == ")") else {
        return Err("direct wrapper has no complete parameter list".into());
    };
    let mut start = signature_end + 3;
    let mut end = tokens.len();
    let mut closing = vec![None; tokens.len()];
    let mut openings = Vec::new();
    for (index, token) in tokens.iter().enumerate().skip(start) {
        match token.spelling.as_str() {
            "(" => openings.push(index),
            ")" => {
                let Some(opening) = openings.pop() else {
                    return Err("unbalanced direct wrapper replacement".into());
                };
                closing[opening] = Some(index);
            }
            _ => {}
        }
    }
    if !openings.is_empty() {
        return Err("unbalanced direct wrapper replacement".into());
    }
    // Remove only parentheses enclosing the complete replacement list. Interior
    // parentheses remain part of the independently expanded expression proof.
    while tokens.get(start).is_some_and(|token| token.spelling == "(")
        && closing[start] == Some(end - 1)
    {
        start += 1;
        end -= 1;
    }
    let body = &tokens[start..end];
    let Some(callee) = body.first() else { return Ok(None) };
    if !matches!(callee.kind, TokenKind::Identifier | TokenKind::Keyword)
        || body.get(1).is_none_or(|token| token.spelling != "(")
        || closing[start + 1] != Some(end - 1)
    {
        return Ok(None);
    }
    let arguments = &body[2..body.len() - 1];
    let mut arity = usize::from(!arguments.is_empty());
    let mut depth = 0usize;
    for token in arguments {
        match token.spelling.as_str() {
            "(" => depth += 1,
            ")" => depth = depth.checked_sub(1).ok_or("unbalanced direct call arguments")?,
            "," if depth == 0 => arity += 1,
            _ => {}
        }
    }
    if depth != 0 {
        return Err("unbalanced direct call arguments".into());
    }
    Ok(Some((&callee.spelling, arity)))
}
