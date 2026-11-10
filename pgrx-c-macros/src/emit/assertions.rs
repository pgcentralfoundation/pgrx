//LICENSE Portions Copyright 2026 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.

//! Lower PostgreSQL's process-stopping calls, and run assertions only where PostgreSQL does.
//!
//! An assert-enabled PostgreSQL defines `Assert` and its relatives to call
//! `ExceptionalCondition`, and `pg_unreachable()` to call `abort()`. These calls lower to
//! safe support functions that stop the process, so no build makes a native call to stop.
//!
//! Both build flavors translate the assert-enabled definitions, so a macro has one form,
//! one set of operands and one need for the caller's `unsafe`. Whether a check that ends in
//! `ExceptionalCondition` runs is a constant of the installation, which the generated
//! `__pgrx_c_assert_checking!()` supplies. No check is ever left out of a macro: against a
//! release installation its condition is compiled and never evaluated, as in release C.

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

/// The generated constant that says whether the installation enables assertions.
pub(super) const CHECKING: &str = "$crate::__pgrx_c_assert_checking!()";

/// Define the constant [`CHECKING`] names, for the support of one generated macro set.
///
/// It is the only difference between the macros of an assert-enabled and a release build
/// of one PostgreSQL version, apart from layout facts.
pub(super) fn checking(enabled: bool) -> String {
    format!(
        "/// Whether the PostgreSQL these macros were generated for enables assertions.\n#[doc(hidden)]\n#[macro_export]\nmacro_rules! __pgrx_c_assert_checking {{\n    () => {{\n        {enabled}\n    }};\n}}\n"
    )
}

/// Functions a generated macro calls from safe code, each with the reason that is sound.
///
/// They are PostgreSQL's assertion functions that take no pointer, as defined in
/// PostgreSQL 18's `varsup.c`, `relcache.c` and `bufmgr.c`. Each needs only what pgrx's safe
/// wrappers already assume: a backend attached to shared memory, on the thread that passes
/// pgrx's active-thread check. A check that fails aborts the process or raises an error; it
/// does not corrupt memory. [`safe_call`] also requires the reviewed prototype and the
/// guarded binding, which makes that thread check and turns an error into a panic.
const SAFE_CALLS: &[(&str, &str)] = &[
    (
        "AssertCouldGetRelation",
        "it takes no argument, asserts that a transaction is in progress, and compares the buffer locks this backend holds with the shared buffer descriptors",
    ),
    (
        "AssertPendingSyncs_RelationCache",
        "it takes no argument, opens the relations this transaction has locked through the relation cache, and compares each cache entry with the pending syncs, raising an error or aborting when it cannot",
    ),
    (
        "AssertTransactionIdInAllowableRange",
        "it takes a transaction id by value and compares it with the oldest and next transaction ids in shared memory",
    ),
];

/// Why a call to the function `name` needs no `unsafe` from the caller, when it needs none.
///
/// The reason holds for the reviewed prototype only: no result and no parameter but integers
/// and enums. It also needs the defining crate's guard around the native call.
pub(super) fn safe_call(
    declarations: &DeclarationCatalog,
    bindings: &super::BindingCatalog,
    name: &str,
) -> Option<&'static str> {
    let (_, reason) = SAFE_CALLS.iter().find(|(function, _)| *function == name)?;
    let signature = &declarations.function_signatures.get(name)?.signature;
    let scalar =
        |ty: &TypeInfo| matches!(ty.category, TypeCategory::Integer(_) | TypeCategory::Enum);
    let guarded = bindings.ffi_boundary.is_some()
        && bindings.functions.get(name).is_none_or(|binding| binding.guarded);
    (guarded
        && signature.result.category == TypeCategory::Void
        && !signature.variadic
        && signature.parameters.as_ref().is_some_and(|parameters| parameters.iter().all(scalar)))
    .then_some(*reason)
}

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
        crate::frontend::ASSERTION_REPORT => {
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
        Stop::Unreachable => write!(rust, "{support}::unreachable_reached()"),
        Stop::Assertion { condition, error_type } => {
            let condition = text(nodes, condition, &operand);
            let error_type = error_type.map_or_else(
                || "::core::option::Option::None".to_owned(),
                |node| format!("::core::option::Option::Some({})", text(nodes, node, &operand)),
            );
            // The report runs only against an installation that enables assertions.
            write!(
                rust,
                "if {CHECKING} {{ {support}::assertion_failure({condition}, {error_type}) }}"
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

/// Whether rendering an analysis needs the caller's `unsafe`.
///
/// Every check counts, whether or not the installation runs it, so the answer is the same
/// for both build flavors.
pub(super) fn caller_unsafe(
    frontend: &FrontendOutput,
    bindings: &super::BindingCatalog,
    analysis: &MacroAnalysis,
    translations: &BTreeMap<String, Translation>,
) -> bool {
    let Some(expression) = &analysis.expression else { return false };
    let body = expression.syntax.statement_body.as_ref();
    let safety = Safety {
        frontend,
        bindings,
        nodes: &expression.syntax.nodes,
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
    match body {
        Some(body) => safety.statements(&body.statements),
        None => safety.expression(expression.syntax.root, Context::Value),
    }
}

/// Parameters whose operands a rendering never names as values: unused ones, and those
/// that appear only in a stop's diagnostic text.
///
/// Their operands are checked in dead code, so an operand that needs `unsafe` needs it
/// although the C never evaluates it.
pub(super) fn unevaluated_parameters(
    frontend: &FrontendOutput,
    analysis: &MacroAnalysis,
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
        if stop(frontend.declarations(), nodes, index).is_some() {
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

/// Whether a `||` node is `holds || (report, 0)`, as `AssertMacro` expands: an expression
/// that is true unless its report of a failed assertion stops the process.
///
/// Against an installation without assertions the node is true and `holds` is not
/// evaluated.
pub(super) fn assertion_alternative(
    declarations: &DeclarationCatalog,
    nodes: &[ExpressionNode],
    node: NodeId,
) -> bool {
    let ExpressionKind::Binary { operator: BinaryOperator::LogicalOr, right, .. } =
        &nodes[node].kind
    else {
        return false;
    };
    let ExpressionKind::Comma { left: call, right: zero } = &nodes[ungroup(nodes, *right)].kind
    else {
        return false;
    };
    matches!(&nodes[ungroup(nodes, *zero)].kind, ExpressionKind::IntegerLiteral { literal } if literal.value == 0)
        && matches!(stop(declarations, nodes, ungroup(nodes, *call)), Some(Stop::Assertion { .. }))
}

/// Whether a branch only reports a failed assertion, as the failure branch of `Assert` does.
///
/// Against an installation without assertions the `if` statement that guards it does not
/// evaluate its condition.
pub(super) fn reports(
    declarations: &DeclarationCatalog,
    nodes: &[ExpressionNode],
    statement: &Statement,
) -> bool {
    match statement {
        Statement::Block { statements, .. } => {
            matches!(statements.as_slice(), [only] if reports(declarations, nodes, only))
        }
        Statement::Expression { expression, .. } => matches!(
            stop(declarations, nodes, ungroup(nodes, *expression)),
            Some(Stop::Assertion { .. })
        ),
        _ => false,
    }
}

/// Whether rendering an expression or statement needs the caller's `unsafe`.
///
/// This follows the renderer's contexts. Loads, stores, member projections, dereferenced
/// values, indirect calls and native calls need `unsafe`; caller operands are the caller's
/// own, generated locals are owned, unevaluated operands sit in generated `unsafe`, the
/// rendering checks pgrx's active thread before reading a global that `checked_global`
/// admits, and it wraps a call that `safe_call` admits.
struct Safety<'a> {
    /// Declarations that distinguish globals, builtins and stopping calls.
    frontend: &'a FrontendOutput,
    /// Bindings, which decide whether the rendering can check the thread before a read.
    bindings: &'a super::BindingCatalog,
    /// The analyzed expression arena.
    nodes: &'a [ExpressionNode],
    /// Translated inline functions whose calls expand their translation.
    translations: &'a BTreeMap<String, Translation>,
    /// Statement locals, whose generated storage needs no caller `unsafe` to read.
    locals: BTreeSet<&'a str>,
}

impl Safety<'_> {
    /// Whether rendering `statements` needs `unsafe`.
    fn statements(&self, statements: &[Statement]) -> bool {
        statements.iter().any(|statement| match statement {
            Statement::Expression { expression, .. } => {
                self.expression(*expression, Context::Discard)
            }
            Statement::Declaration { initializer, .. } => {
                initializer.is_some_and(|initializer| self.expression(initializer, Context::Value))
            }
            Statement::Return { expression, .. } => self.expression(*expression, Context::Value),
            Statement::Block { statements, .. } => self.statements(statements),
            Statement::If { condition, then_branch, else_branch, .. } => {
                self.expression(*condition, Context::Value)
                    || self.statements(std::slice::from_ref(then_branch))
                    || else_branch.as_ref().is_some_and(|branch| {
                        self.statements(std::slice::from_ref(branch.as_ref()))
                    })
            }
        })
    }

    /// Whether rendering `root` in `context` needs `unsafe`.
    fn expression(&self, root: NodeId, context: Context) -> bool {
        let mut pending = vec![(root, context)];
        while let Some((index, context)) = pending.pop() {
            if matches!(context, Context::Size) {
                continue;
            }
            let value = |node: &NodeId| (*node, Context::Value);
            match &self.nodes[index].kind {
                ExpressionKind::Group { operand } => pending.push((*operand, context)),
                ExpressionKind::Identifier { name } => {
                    let declarations = self.frontend.declarations();
                    if matches!(context, Context::Value | Context::Discard)
                        && !self.locals.contains(name.as_str())
                        && declarations.variables.contains_key(name)
                        && !super::checked_global(declarations, self.bindings, name)
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
                    let declarations = self.frontend.declarations();
                    match self.translations.get(name) {
                        Some(translation) if translation.caller_unsafe => return true,
                        Some(_) => {}
                        None if declarations.builtins.contains_key(name) => {}
                        None if safe_call(declarations, self.bindings, name).is_some() => {}
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
                | ExpressionKind::TypeParameterPaste { .. }
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
