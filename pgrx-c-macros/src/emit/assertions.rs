//LICENSE Portions Copyright 2026 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.

//! Lower PostgreSQL's process-stopping calls, and keep assertions from changing safety.
//!
//! An assert-enabled PostgreSQL defines `Assert` and its relatives to call
//! `ExceptionalCondition`, and `pg_unreachable()` to call `abort()`. A release build omits
//! the assertions and marks unreachable code with `__builtin_unreachable()`, or MSVC's
//! `__assume`. These calls lower to safe support functions that stop the process, so
//! neither build makes a native call to stop.
//!
//! A generated macro needs the caller's `unsafe` in an assert-enabled build exactly when
//! it does in a release build. A check whose condition needs `unsafe`, such as one that
//! reads a global or a pointer, is therefore omitted when nothing else in the expansion
//! needs `unsafe`, as a release build omits it. Otherwise the caller already holds
//! `unsafe` for the rest, and every check stays.

use super::typed::Context;
use crate::syntax::{
    BinaryOperator, ExpressionKind, ExpressionNode, NodeId, Statement, StringPart,
};
use crate::{
    BuiltinKind, DeclarationCatalog, FrontendOutput, IntegerKind, MacroAnalysis, TypeCategory,
    TypeInfo, TypeShapeKind,
};
use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Write;

/// What batch generation knows about a translated inline function that its callers expand.
#[derive(Clone, Debug, Default)]
pub(super) struct Translation {
    /// The Rust panic conditions of the translation, including those it expands.
    pub(super) panics: BTreeSet<&'static str>,
    /// Whether expanding the translation needs the caller's `unsafe`.
    pub(super) caller_unsafe: bool,
}

/// A call that stops the process instead of returning.
#[derive(Clone, Copy)]
pub(crate) enum Stop {
    /// `ExceptionalCondition` reporting a failed check: the C text of its condition and,
    /// before PostgreSQL 16, its error type. Its file and line are the Rust invocation's.
    Assertion {
        /// A string literal or stringification of the checked condition.
        condition: NodeId,
        /// PostgreSQL 15's error type: a string literal, stringification or caller operand.
        error_type: Option<NodeId>,
    },
    /// C's `abort()`.
    Abort,
    /// `__builtin_unreachable()`.
    Unreachable,
}

/// Recognize a call that stops the process.
///
/// `ExceptionalCondition` is PostgreSQL's assertion report, which aborts. `abort` is
/// reserved for C's standard library. Only their exact prototypes, with diagnostic text,
/// `__FILE__` and `__LINE__` arguments for an assertion, are recognized; any other call
/// to them remains a native call.
pub(crate) fn stop(
    declarations: &DeclarationCatalog,
    nodes: &[ExpressionNode],
    call: NodeId,
) -> Option<Stop> {
    let ExpressionKind::Call { callee, arguments } = &nodes[call].kind else { return None };
    let ExpressionKind::Identifier { name } = &nodes[ungroup(nodes, *callee)].kind else {
        return None;
    };
    if let Some(builtin) = declarations.builtins.get(name) {
        return (builtin.kind == BuiltinKind::Unreachable && arguments.is_empty())
            .then_some(Stop::Unreachable);
    }
    let signature = &declarations.function_signatures.get(name)?.signature;
    let parameters = signature.parameters.as_ref()?;
    if signature.result.category != TypeCategory::Void
        || signature.variadic
        || parameters.len() != arguments.len()
    {
        return None;
    }
    match name.as_str() {
        "abort" => parameters.is_empty().then_some(Stop::Abort),
        "ExceptionalCondition" => {
            let (text, location) = parameters.split_at(parameters.len().checked_sub(2)?);
            let [file, line] = location else { return None };
            if !matches!(text.len(), 1 | 2)
                || !text.iter().chain([file]).all(|ty| const_char_pointer(declarations, ty))
                || line.category != TypeCategory::Integer(IntegerKind::Int)
            {
                return None;
            }
            let argument = |index: usize| ungroup(nodes, arguments[index]);
            let diagnostic = |node: NodeId| {
                matches!(
                    nodes[node].kind,
                    ExpressionKind::StringLiteral { .. } | ExpressionKind::Stringification { .. }
                )
            };
            let condition = argument(0);
            let error_type = (text.len() == 2).then(|| argument(1));
            let valid = diagnostic(condition)
                && error_type.is_none_or(|node| {
                    diagnostic(node) || matches!(nodes[node].kind, ExpressionKind::Parameter { .. })
                })
                && matches!(nodes[argument(text.len())].kind, ExpressionKind::InvocationFile)
                && matches!(nodes[argument(text.len() + 1)].kind, ExpressionKind::InvocationLine);
            valid.then_some(Stop::Assertion { condition, error_type })
        }
        _ => None,
    }
}

/// Write the safe support call that replaces a stopping call.
///
/// `operand` names a caller operand's descriptor, whose source spelling stands in for
/// the C spelling, as for other operand diagnostics.
pub(super) fn render_stop(
    nodes: &[ExpressionNode],
    stop: Stop,
    operand: impl Fn(usize) -> String,
    rust: &mut String,
) {
    let support = format!("{}::assertion", super::types::EXPRESSION);
    match stop {
        Stop::Abort => write!(rust, "{support}::abort()"),
        Stop::Unreachable => write!(
            rust,
            "{support}::unreachable_reached(::core::file!(), ::core::line!())"
        ),
        Stop::Assertion { condition, error_type } => {
            let condition = text(nodes, condition, &operand);
            let error_type = error_type.map_or_else(
                || "::core::option::Option::None".to_owned(),
                |node| format!("::core::option::Option::Some({})", text(nodes, node, &operand)),
            );
            write!(
                rust,
                "{support}::assertion_failure({condition}, {error_type}, ::core::file!(), ::core::line!())"
            )
        }
    }
    .expect("String output");
}

/// A Rust `&'static str` expression for diagnostic C text.
fn text(nodes: &[ExpressionNode], node: NodeId, operand: &impl Fn(usize) -> String) -> String {
    let stringify =
        |index: usize| format!("$crate::__pgrx_c_operand!(@stringify; {})", operand(index));
    match &nodes[node].kind {
        ExpressionKind::StringLiteral { bytes } => {
            let bytes = bytes.strip_suffix(&[0]).unwrap_or(bytes);
            format!("{:?}", String::from_utf8_lossy(bytes))
        }
        ExpressionKind::Stringification { parts } => {
            let mut rust = String::from("::core::concat!(");
            for part in parts {
                match part {
                    StringPart::Text { bytes } => {
                        write!(rust, "{:?}, ", String::from_utf8_lossy(bytes))
                    }
                    StringPart::Parameter { index } => write!(rust, "{}, ", stringify(*index)),
                }
                .expect("String output");
            }
            rust.push(')');
            rust
        }
        ExpressionKind::Parameter { index } => stringify(*index),
        _ => unreachable!("stop recognition admits only diagnostic text"),
    }
}

/// Which checks a rendering omits, and whether the rest needs the caller's `unsafe`.
#[derive(Default)]
pub(super) struct Plan {
    /// Omitted checks: an `if` statement's condition, a `||` node, or an `__assume` call.
    pub(super) dropped: BTreeSet<NodeId>,
    /// Whether the rendering needs the caller's `unsafe`.
    pub(super) caller_unsafe: bool,
}

/// Decide which checks to omit so the expansion needs `unsafe` only where its release
/// form does.
///
/// A check is an `if` whose only statement stops, `holds || (stop, 0)`, or
/// `__assume(holds)`. Nested checks are decided first. When the expansion without the
/// checks whose conditions need `unsafe` needs no `unsafe`, those checks are omitted;
/// otherwise all checks stay.
pub(super) fn plan(
    frontend: &FrontendOutput,
    analysis: &MacroAnalysis,
    translations: &BTreeMap<String, Translation>,
) -> Plan {
    let Some(expression) = &analysis.expression else { return Plan::default() };
    let nodes = &expression.syntax.nodes;
    let body = expression.syntax.statement_body.as_ref();
    let safety = Safety {
        frontend,
        nodes,
        translations,
        locals: body
            .iter()
            .flat_map(|body| body.walk())
            .filter_map(|statement| match statement {
                Statement::Declaration { name, .. } => Some(name.as_str()),
                _ => None,
            })
            .collect(),
    };
    // Children precede parents in the arena, and statements hold no expression checks'
    // parents, so expression checks in arena order and then statement checks in source
    // order decide nested checks first.
    let mut checks = Vec::new();
    for (index, node) in nodes.iter().enumerate() {
        if let Some(holds) = expression_check(frontend, nodes, node) {
            checks.push((index, holds));
        }
    }
    for statement in body.iter().flat_map(|body| body.walk()) {
        if let Statement::If { condition, then_branch, else_branch: None, .. } = statement
            && stops(frontend.declarations(), nodes, then_branch)
        {
            checks.push((*condition, *condition));
        }
    }
    // Analysis already omitted checks that read a local before its initialization.
    let omitted = &expression.omitted_checks;
    let mut unsafe_checks = omitted.clone();
    for (check, holds) in checks {
        if !omitted.contains(&check) && safety.expression(holds, Context::Value, &unsafe_checks) {
            unsafe_checks.insert(check);
        }
    }
    let rest = match body {
        Some(body) => safety.statements(&body.statements, &unsafe_checks),
        None => safety.expression(expression.syntax.root, Context::Value, &unsafe_checks),
    };
    if rest {
        Plan { dropped: omitted.clone(), caller_unsafe: true }
    } else {
        Plan { dropped: unsafe_checks, caller_unsafe: false }
    }
}

/// Parameters whose operands a rendering never evaluates: unused ones, and those that
/// appear only in omitted checks or a stop's diagnostic text.
///
/// Their operands are checked in dead code, so an operand that needs `unsafe` needs it
/// whether or not a build's C evaluates it, as `Assert` does only when assertions are
/// enabled.
pub(super) fn unevaluated_parameters(
    frontend: &FrontendOutput,
    analysis: &MacroAnalysis,
    dropped: &BTreeSet<NodeId>,
) -> Vec<usize> {
    use crate::ParameterRole;
    let Some(expression) = &analysis.expression else { return Vec::new() };
    let nodes = &expression.syntax.nodes;
    let mut pending = Vec::new();
    match &expression.syntax.statement_body {
        Some(body) => {
            let mut statements = body.statements.iter().collect::<Vec<_>>();
            while let Some(statement) = statements.pop() {
                match statement {
                    Statement::Expression { expression, .. }
                    | Statement::Return { expression, .. } => pending.push(*expression),
                    Statement::Declaration { initializer, .. } => pending.extend(*initializer),
                    Statement::Block { statements: inner, .. } => statements.extend(inner),
                    Statement::If { condition, .. } if dropped.contains(condition) => {}
                    Statement::If { condition, then_branch, else_branch, .. } => {
                        pending.push(*condition);
                        statements.push(then_branch);
                        statements.extend(else_branch.as_deref());
                    }
                }
            }
        }
        None => pending.push(expression.syntax.root),
    }
    let mut evaluated = BTreeSet::new();
    while let Some(index) = pending.pop() {
        if dropped.contains(&index) || stop(frontend.declarations(), nodes, index).is_some() {
            continue;
        }
        match &nodes[index].kind {
            ExpressionKind::Parameter { index } => {
                evaluated.insert(*index);
            }
            ExpressionKind::Group { operand }
            | ExpressionKind::Cast { operand, .. }
            | ExpressionKind::Unary { operand, .. }
            | ExpressionKind::TypeParameterCast { operand, .. }
            | ExpressionKind::Dereference { operand }
            | ExpressionKind::AddressOf { operand }
            | ExpressionKind::SizeOfExpression { operand }
            | ExpressionKind::Update { operand, .. }
            | ExpressionKind::Member { base: operand, .. } => pending.push(*operand),
            ExpressionKind::Binary { left, right, .. } | ExpressionKind::Comma { left, right } => {
                pending.extend([*left, *right]);
            }
            ExpressionKind::Index { base, index } => pending.extend([*base, *index]),
            ExpressionKind::Assignment { place, value, .. } => pending.extend([*place, *value]),
            ExpressionKind::Conditional { condition, then_value, else_value } => {
                pending.extend([*condition, *then_value, *else_value]);
            }
            ExpressionKind::Call { callee, arguments } => {
                pending.push(*callee);
                pending.extend(arguments);
            }
            _ => {}
        }
    }
    analysis
        .parameters
        .iter()
        .enumerate()
        .filter(|(index, parameter)| {
            !evaluated.contains(index)
                && parameter
                    .roles
                    .iter()
                    .any(|role| matches!(role, ParameterRole::Value | ParameterRole::Unused))
        })
        .map(|(index, _)| index)
        .collect()
}

/// The condition of a `holds || (stop, 0)` or `__assume(holds)` check.
fn expression_check(
    frontend: &FrontendOutput,
    nodes: &[ExpressionNode],
    node: &ExpressionNode,
) -> Option<NodeId> {
    match &node.kind {
        ExpressionKind::Binary { operator: BinaryOperator::LogicalOr, left, right } => {
            let ExpressionKind::Comma { left: call, right: zero } =
                &nodes[ungroup(nodes, *right)].kind
            else {
                return None;
            };
            let zero = matches!(&nodes[ungroup(nodes, *zero)].kind, ExpressionKind::IntegerLiteral { literal } if literal.value == 0);
            (zero && stop(frontend.declarations(), nodes, ungroup(nodes, *call)).is_some())
                .then_some(*left)
        }
        ExpressionKind::Call { callee, arguments } => {
            let ExpressionKind::Identifier { name } = &nodes[ungroup(nodes, *callee)].kind else {
                return None;
            };
            let builtin = frontend.declarations().builtins.get(name)?;
            match (builtin.kind, arguments.as_slice()) {
                (BuiltinKind::Assume, [holds]) => Some(*holds),
                _ => None,
            }
        }
        _ => None,
    }
}

/// Whether a branch only stops the process, as the failure branch of `Assert` does.
pub(crate) fn stops(
    declarations: &DeclarationCatalog,
    nodes: &[ExpressionNode],
    statement: &Statement,
) -> bool {
    match statement {
        Statement::Block { statements, .. } => {
            matches!(statements.as_slice(), [only] if stops(declarations, nodes, only))
        }
        Statement::Expression { expression, .. } => {
            stop(declarations, nodes, ungroup(nodes, *expression)).is_some()
        }
        _ => false,
    }
}

/// Whether rendering an expression or statement needs the caller's `unsafe`.
///
/// This follows the renderer's contexts. Loads, stores, member projections, dereferenced
/// values, indirect calls and native calls need `unsafe`; caller operands are the caller's
/// own, generated locals are owned, and unevaluated operands sit in generated `unsafe`.
struct Safety<'a> {
    /// Declarations that distinguish globals, builtins and stopping calls.
    frontend: &'a FrontendOutput,
    /// The analyzed expression arena.
    nodes: &'a [ExpressionNode],
    /// Translated inline functions whose calls expand their translation.
    translations: &'a BTreeMap<String, Translation>,
    /// Statement locals, whose generated storage needs no caller `unsafe` to read.
    locals: BTreeSet<&'a str>,
}

impl Safety<'_> {
    /// Whether evaluating `statements` needs `unsafe`, without the checks in `dropped`.
    fn statements(&self, statements: &[Statement], dropped: &BTreeSet<NodeId>) -> bool {
        statements.iter().any(|statement| match statement {
            Statement::Expression { expression, .. } => {
                self.expression(*expression, Context::Discard, dropped)
            }
            Statement::Declaration { initializer, .. } => initializer
                .is_some_and(|initializer| self.expression(initializer, Context::Value, dropped)),
            Statement::Return { expression, .. } => {
                self.expression(*expression, Context::Value, dropped)
            }
            Statement::Block { statements, .. } => self.statements(statements, dropped),
            Statement::If { condition, .. } if dropped.contains(condition) => false,
            Statement::If { condition, then_branch, else_branch, .. } => {
                self.expression(*condition, Context::Value, dropped)
                    || self.statements(std::slice::from_ref(then_branch), dropped)
                    || else_branch.as_ref().is_some_and(|branch| {
                        self.statements(std::slice::from_ref(branch.as_ref()), dropped)
                    })
            }
        })
    }

    /// Whether evaluating `root` in `context` needs `unsafe`, without the checks in `dropped`.
    fn expression(&self, root: NodeId, context: Context, dropped: &BTreeSet<NodeId>) -> bool {
        let mut pending = vec![(root, context)];
        while let Some((index, context)) = pending.pop() {
            if dropped.contains(&index) || matches!(context, Context::Size) {
                continue;
            }
            let value = |node: &NodeId| (*node, Context::Value);
            match &self.nodes[index].kind {
                ExpressionKind::Group { operand } => pending.push((*operand, context)),
                ExpressionKind::Identifier { name } => {
                    if matches!(context, Context::Value | Context::Discard)
                        && !self.locals.contains(name.as_str())
                        && self.frontend.declarations().variables.contains_key(name)
                    {
                        return true;
                    }
                }
                ExpressionKind::Cast { operand, .. }
                | ExpressionKind::Unary { operand, .. }
                | ExpressionKind::TypeParameterCast { operand, .. } => pending.push(value(operand)),
                ExpressionKind::Binary { left, right, .. } => {
                    pending.extend([value(left), value(right)]);
                }
                ExpressionKind::Conditional { condition, then_value, else_value } => {
                    pending.extend([value(condition), value(then_value), value(else_value)]);
                }
                ExpressionKind::Comma { left, right } => {
                    pending.extend([(*left, Context::Discard), value(right)]);
                }
                ExpressionKind::Dereference { operand } => {
                    if matches!(context, Context::Value | Context::Discard) {
                        return true;
                    }
                    pending.push(value(operand));
                }
                ExpressionKind::AddressOf { operand } => {
                    match &self.nodes[ungroup(self.nodes, *operand)].kind {
                        ExpressionKind::Dereference { operand } => pending.push(value(operand)),
                        _ => pending.push((*operand, Context::Place)),
                    }
                }
                ExpressionKind::Index { base, index: offset } => {
                    if matches!(context, Context::Value | Context::Discard) {
                        return true;
                    }
                    pending.extend([value(base), value(offset)]);
                }
                ExpressionKind::Member { .. }
                | ExpressionKind::Assignment { .. }
                | ExpressionKind::Update { .. } => return true,
                ExpressionKind::Call { callee, arguments } => {
                    if stop(self.frontend.declarations(), self.nodes, index).is_some() {
                        continue;
                    }
                    let ExpressionKind::Identifier { name } =
                        &self.nodes[ungroup(self.nodes, *callee)].kind
                    else {
                        return true;
                    };
                    let builtin = self.frontend.declarations().builtins.contains_key(name);
                    match self.translations.get(name) {
                        Some(translation) if translation.caller_unsafe => return true,
                        Some(_) => {}
                        None if builtin => {}
                        None => return true,
                    }
                    pending.extend(arguments.iter().map(value));
                }
                ExpressionKind::Empty
                | ExpressionKind::Parameter { .. }
                | ExpressionKind::IntegerLiteral { .. }
                | ExpressionKind::StringLiteral { .. }
                | ExpressionKind::Stringification { .. }
                | ExpressionKind::InvocationFile
                | ExpressionKind::InvocationLine
                | ExpressionKind::SizeOfType { .. }
                | ExpressionKind::SizeOfExpression { .. }
                | ExpressionKind::AlignOfType { .. }
                | ExpressionKind::SizeOfTypeParameter { .. }
                | ExpressionKind::AlignOfTypeParameter { .. }
                | ExpressionKind::OffsetOf { .. } => {}
            }
        }
        false
    }
}

/// Look through grouping parentheses.
fn ungroup(nodes: &[ExpressionNode], mut node: NodeId) -> NodeId {
    while let ExpressionKind::Group { operand } = nodes[node].kind {
        node = operand;
    }
    node
}

/// Whether a type is `const char *`, as `ExceptionalCondition`'s text parameters are.
fn const_char_pointer(declarations: &DeclarationCatalog, ty: &TypeInfo) -> bool {
    matches!(
        declarations.type_shapes.get(&ty.canonical_spelling).map(|shape| &shape.kind),
        Some(TypeShapeKind::Pointer { pointee })
            if pointee.category == TypeCategory::Integer(IntegerKind::Char)
                && pointee.is_const
                && !pointee.is_volatile
    )
}
