//LICENSE Portions Copyright 2026 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.

//! Lower C values, places and prototype conversions through one expression arena.

use super::arguments::{self, ArgumentContext};
use super::types::{EXPRESSION, Lowering, rust_path};
use super::{BindingCatalog, MAX_EMISSION_BYTES, SUPPORT, binary_helper, render_expression, skip};
use crate::analysis::{MacroAnalysis, ResolvedConstant, SkipReason, SkipReasonCode};
use crate::syntax::{
    BinaryOperator, ExpressionKind, NodeId, OffsetComponent, OffsetRecord, UnaryOperator,
};
use crate::{FrontendOutput, SignedOverflow, TypeCategory, TypeInfo, TypeShapeKind};
use std::collections::BTreeMap;
use std::fmt::Write;

#[derive(Clone, Copy)]
pub(super) enum Context {
    Value,
    Place,
    ReadPlace,
    Size,
    Discard,
}

enum Task {
    Node(NodeId, Context),
    Text(String),
}

struct Local {
    index: usize,
    ty: TypeInfo,
    lowered: Result<super::types::LoweredType, String>,
}

/// Per-macro expression facts borrowing the pass's immutable storage reconciliation.
pub(super) struct Renderer<'a> {
    frontend: &'a FrontendOutput,
    analysis: &'a MacroAnalysis,
    bindings: &'a BindingCatalog,
    constants: &'a [Option<&'a ResolvedConstant>],
    lowering: &'a Lowering<'a>,
    floats: bool,
    places: Vec<bool>,
    objects: Vec<Option<TypeInfo>>,
    locals: BTreeMap<String, Local>,
}

impl<'a> Renderer<'a> {
    pub(super) fn new(
        frontend: &'a FrontendOutput,
        analysis: &'a MacroAnalysis,
        bindings: &'a BindingCatalog,
        constants: &'a [Option<&'a ResolvedConstant>],
        lowering: &'a Lowering<'a>,
    ) -> Self {
        let expression = analysis.expression.as_ref().expect("candidate expression");
        let locals = expression
            .syntax
            .statement_body
            .iter()
            .flat_map(|body| body.walk())
            .filter_map(|statement| {
                let crate::syntax::Statement::Declaration { name, type_name, .. } = statement
                else {
                    return None;
                };
                crate::analysis::resolve_type_info(
                    type_name,
                    frontend.declarations(),
                    &frontend.profile().target,
                )
                .map(|ty| (name.clone(), ty))
            })
            .enumerate()
            .map(|(index, (name, ty))| {
                let lowered = lowering.resolve(&ty);
                (name, Local { index, ty, lowered })
            })
            .collect::<BTreeMap<_, _>>();
        // Native Rust operands and public macro results are evaluated value
        // boundaries. The fully expanded arena includes contraction across
        // recovered source-level macro calls.
        let multiplication = expression.syntax.nodes.iter().any(|node| {
            matches!(
                node.kind,
                ExpressionKind::Binary { operator: BinaryOperator::Multiply, .. }
                    | ExpressionKind::Assignment { operator: Some(BinaryOperator::Multiply), .. }
            )
        });
        let addition = expression.syntax.nodes.iter().any(|node| {
            matches!(
                node.kind,
                ExpressionKind::Binary {
                    operator: BinaryOperator::Add | BinaryOperator::Subtract,
                    ..
                } | ExpressionKind::Assignment {
                    operator: Some(BinaryOperator::Add | BinaryOperator::Subtract),
                    ..
                }
            )
        });
        let floats = frontend.profile().supports_float_values().is_ok()
            && (!(multiplication && addition)
                || frontend.profile().supports_float_expressions().is_ok());
        // Children precede their parents in the expression arena. One pass
        // carries lvalue status through groups and nested direct field access,
        // rather than rescanning an arbitrarily deep record expression.
        let mut places = Vec::with_capacity(expression.syntax.nodes.len());
        for node in &expression.syntax.nodes {
            places.push(match &node.kind {
                ExpressionKind::Group { operand } => places[*operand],
                ExpressionKind::Member { base, indirect, .. } => *indirect || places[*base],
                ExpressionKind::Identifier { name } => {
                    locals.contains_key(name)
                        || frontend.declarations().variables.contains_key(name)
                }
                ExpressionKind::Parameter { .. }
                | ExpressionKind::Dereference { .. }
                | ExpressionKind::Index { .. } => true,
                _ => false,
            });
        }
        let objects = declared_objects(frontend, analysis, lowering, &locals);
        Self { frontend, analysis, bindings, constants, lowering, floats, places, objects, locals }
    }

    pub(super) fn declare_local(
        &self,
        name: &str,
        initializer: Option<NodeId>,
        rust: &mut String,
    ) -> Result<(), SkipReason> {
        let local = self.locals.get(name).ok_or_else(|| {
            skip(
                self.analysis,
                SkipReasonCode::UnsupportedType,
                format!("{name}: local declaration has no established C type"),
                None,
            )
        })?;
        let lowered = local.lowered.as_ref().map_err(|message| {
            skip(self.analysis, SkipReasonCode::UnsupportedType, message.clone(), None)
        })?;
        write!(
            rust,
            "let mut __pgrx_c_local{} = ::core::mem::MaybeUninit::<{}>::uninit(); ",
            local.index, lowered.storage
        )
        .expect("String output");
        if let Some(initializer) = initializer {
            rust.push_str("let __pgrx_c_initializer = ");
            self.render(initializer, Context::Value, rust)?;
            write!(rust, "; /* SAFETY: this raw place addresses the new, aligned local slot; initialization writes without reading its previous bytes or creating a reference. */ #[allow(unused_unsafe)] unsafe {{ {EXPRESSION}::assign({EXPRESSION}::place::<{}>(::core::ptr::addr_of_mut!(__pgrx_c_local{}).cast::<{}>()), __pgrx_c_initializer); }} ", lowered.marker, local.index, lowered.storage).expect("String output");
        }
        Ok(())
    }

    /// Preserve each requested context lazily at a recovered macro-call boundary.
    pub(super) fn delegated_argument(&self, root: NodeId, rust: &mut String) {
        let expression = self.analysis.expression.as_ref().expect("candidate expression");
        let mut ungrouped = root;
        while let ExpressionKind::Group { operand } = expression.syntax.nodes[ungrouped].kind {
            ungrouped = operand;
        }
        if let ExpressionKind::Parameter { index } = expression.syntax.nodes[ungrouped].kind {
            write!(rust, "${}, ", arguments::name(self.analysis, index)).expect("String output");
            return;
        }
        rust.push_str("(@compiled ");
        for context in [Context::Value, Context::Place, Context::ReadPlace, Context::Size] {
            rust.push('[');
            let mut argument = String::new();
            match self.render(root, context, &mut argument) {
                Ok(()) => rust.push_str(&argument),
                Err(reason) => {
                    write!(rust, "compile_error!({:?})", reason.message).expect("String output")
                }
            }
            rust.push_str("] ");
        }
        rust.push_str("), ");
    }

    pub(super) fn render(
        &self,
        root: NodeId,
        context: Context,
        rust: &mut String,
    ) -> Result<(), SkipReason> {
        let (frontend, analysis, bindings, constants, lowering, floats) = (
            self.frontend,
            self.analysis,
            self.bindings,
            self.constants,
            self.lowering,
            self.floats,
        );
        let expression = analysis.expression.as_ref().expect("candidate expression");
        let policy = match analysis.signed_overflow {
            SignedOverflow::Undefined => "Undefined",
            SignedOverflow::Wrapping => "Wrapping",
            SignedOverflow::Trapping => {
                return Err(skip(
                    analysis,
                    SkipReasonCode::UnsupportedProfile,
                    "trapping arithmetic has no established expression policy",
                    None,
                ));
            }
        };
        let mut tasks = vec![Task::Node(root, context)];
        while let Some(task) = tasks.pop() {
            match task {
                Task::Text(text) => rust.push_str(&text),
                Task::Node(index, context) => {
                    let node = &expression.syntax.nodes[index];
                    let failure = |message: String| {
                        skip(analysis, SkipReasonCode::UnsupportedType, message, Some(node.tokens))
                    };
                    let value = |node| Task::Node(node, Context::Value);
                    let place = |node| Task::Node(node, Context::Place);
                    let text = |text: &str| Task::Text(text.into());
                    if matches!(context, Context::Value | Context::Discard)
                        && self.places[index]
                        && self.objects[index].as_ref().is_some_and(volatile_record)
                    {
                        return Err(failure("volatile whole-record access requires a compiler-verified aggregate access primitive".into()));
                    }
                    if matches!(context, Context::Value)
                        && expression.integer_zero_constants.contains(&index)
                        && !matches!(node.kind, ExpressionKind::Group { .. })
                    {
                        write!(rust, "{EXPRESSION}::null_constant(").expect("String output");
                        tasks.push(text(")"));
                    }
                    if matches!(context, Context::Size | Context::Discard) {
                        if matches!(context, Context::Size)
                            && let ExpressionKind::Assignment {
                                operator: None,
                                place: destination,
                                value: source,
                            } = node.kind
                            && let Some(ty) = self.objects[index]
                                .as_ref()
                                .filter(|ty| ty.category == TypeCategory::Record)
                        {
                            let lowered = lowering.resolve(ty).map_err(failure)?;
                            let raw = format!("{EXPRESSION}::CRawRecord<{}>", lowered.storage);
                            write!(
                                rust,
                                "{{ let _ = {EXPRESSION}::size_of_place_type(if false {{ Some("
                            )
                            .expect("String output");
                            tasks.extend([
                                text(")) } else { None }) }"),
                                value(source),
                                Task::Text(format!(" ) }} else {{ None }}); {EXPRESSION}::size_of_value_type(if false {{ Some({EXPRESSION}::implicit::<{raw}, _>(")),
                                place(destination),
                            ]);
                            continue;
                        }
                        if let ExpressionKind::Group { operand } = node.kind {
                            tasks.push(Task::Node(operand, context));
                        } else if let ExpressionKind::Parameter { index: parameter } = node.kind {
                            arguments::render_operand(
                                analysis,
                                parameter,
                                if matches!(context, Context::Size) {
                                    ArgumentContext::Size
                                } else {
                                    ArgumentContext::Discard
                                },
                                floats,
                                rust,
                            );
                        } else if matches!(context, Context::Discard) {
                            rust.push_str("{ let _ = ");
                            tasks.extend([text("; }"), value(index)]);
                        } else {
                            if matches!(&node.kind, ExpressionKind::Identifier { name } if frontend.declarations().function_signatures.contains_key(name) && !frontend.declarations().variables.contains_key(name))
                            {
                                return Err(failure("sizeof a function designator requires a separately verified compiler extension".into()));
                            }
                            if let ExpressionKind::Member { base, field, field_parameter, .. } =
                                &node.kind
                                && !self.places[index]
                            {
                                let marker = match field_parameter {
                                    Some(parameter) => format!("$crate::__pgrx_c_field_marker!(${})", arguments::name(analysis, *parameter)),
                                    None => bindings.field_capabilities.get(field).cloned().ok_or_else(|| failure(format!("{field}: no compiler-checked field adapter is available")))?,
                                };
                                write!(rust, "{EXPRESSION}::record::size_of_member_type::<{marker}, _>(if false {{ Some(unsafe {{ ").expect("String output");
                                tasks.extend([text(" }) } else { None })"), value(*base)]);
                                continue;
                            }
                            let known = self.places[index];
                            write!(
                                rust,
                                "{EXPRESSION}::{}(if false {{ Some(unsafe {{ ",
                                if known { "size_of_place_type" } else { "size_of_value_type" }
                            )
                            .expect("String output");
                            tasks.extend([
                                text(" }) } else { None })"),
                                Task::Node(
                                    index,
                                    if known { Context::ReadPlace } else { Context::Value },
                                ),
                            ]);
                        }
                        continue;
                    }
                    if matches!(context, Context::Value) {
                        write!(rust, "{EXPRESSION}::profile_value::<{floats}, _>(")
                            .expect("String output");
                        tasks.push(text(")"));
                    }
                    match &node.kind {
                        ExpressionKind::Group { operand } => {
                            rust.push('(');
                            tasks.extend([text(")"), Task::Node(*operand, context)]);
                        }
                        ExpressionKind::Parameter { index: parameter } => {
                            arguments::render_operand(
                                analysis,
                                *parameter,
                                match context {
                                    Context::Value => ArgumentContext::Value,
                                    Context::Place => ArgumentContext::Place,
                                    Context::ReadPlace => ArgumentContext::ReadPlace,
                                    Context::Size | Context::Discard => unreachable!(
                                        "unevaluated/discarded operands dispatch before value rendering"
                                    ),
                                },
                                floats,
                                rust,
                            );
                        }
                        ExpressionKind::Identifier { name } if constants[index].is_none() => {
                            if let Some(local) = self.locals.get(name) {
                                let lowered = local
                                    .lowered
                                    .as_ref()
                                    .map_err(|message| failure(message.clone()))?;
                                if matches!(context, Context::Value) {
                                    write!(rust, "/* SAFETY: analysis proves this local initialized before each evaluated read; its aligned storage is owned here and addressed without creating references. */ #[allow(unused_unsafe)] unsafe {{ {EXPRESSION}::load(").expect("String output");
                                }
                                write!(rust, "{EXPRESSION}::{}::<{}>(::core::ptr::addr_of_mut!(__pgrx_c_local{}).cast::<{}>())", if local.ty.is_const { "const_place" } else { "place" }, lowered.marker, local.index, lowered.storage).expect("String output");
                                if matches!(context, Context::Value) {
                                    rust.push_str(") }");
                                }
                            } else if let Some(declaration) =
                                frontend.declarations().variables.get(name)
                            {
                                let binding = bindings.variables.get(name).ok_or_else(|| {
                                    failure(format!("{name}: no public global binding was emitted"))
                                })?;
                                let lowered = lowering
                                    .resolve_with_storage(declaration, &binding.ty)
                                    .map_err(failure)?;
                                let path = rust_path(&binding.path).map_err(failure)?;
                                if matches!(context, Context::Value) {
                                    write!(rust, "{EXPRESSION}::load(").expect("String output");
                                }
                                write!(
                                    rust,
                                    "{EXPRESSION}::{}::<{}>(::core::ptr::{}!({path}))",
                                    if declaration.is_const || !binding.mutable {
                                        "const_place"
                                    } else {
                                        "place"
                                    },
                                    lowered.marker,
                                    if declaration.is_const || !binding.mutable {
                                        "addr_of"
                                    } else {
                                        "addr_of_mut"
                                    }
                                )
                                .expect("String output");
                                if matches!(context, Context::Value) {
                                    rust.push(')');
                                }
                            } else {
                                let binding = bindings.function_addresses.get(name).ok_or_else(|| skip(analysis, SkipReasonCode::Call, format!("{name}: no verified original C function-address adapter was emitted"), Some(node.tokens)))?;
                                let path = rust_path(&binding.path).map_err(failure)?;
                                write!(rust, "{path}()").expect("String output");
                            }
                        }
                        ExpressionKind::Empty => rust.push_str("()"),
                        ExpressionKind::IntegerLiteral { .. }
                        | ExpressionKind::Identifier { .. } => {
                            if matches!(context, Context::Place | Context::ReadPlace) {
                                return Err(failure(
                                    "a C literal or constant is not an object place".into(),
                                ));
                            }
                            render_expression(analysis, index, bindings, constants, rust)?;
                        }
                        ExpressionKind::Cast { type_name, operand } => {
                            let ty = crate::analysis::resolve_type_info(
                                type_name,
                                frontend.declarations(),
                                &frontend.profile().target,
                            )
                            .ok_or_else(|| {
                                failure(format!("{type_name}: unresolved C cast type"))
                            })?;
                            if ty.category == TypeCategory::Void {
                                rust.push_str("{ ");
                                tasks.extend([text(" }"), Task::Node(*operand, Context::Discard)]);
                            } else {
                                let null =
                                    proved_integer_zero(frontend, analysis, constants, *operand);
                                let integer_null = null
                                    && matches!(ty.category, TypeCategory::Integer(_))
                                    && !expression.integer_zero_constants.contains(&index);
                                let void_null = null && is_unqualified_void_pointer(frontend, &ty);
                                let (ty, alias) = match lowering.cast_alias(type_name, &ty) {
                                    Some(Ok((alias, lowered))) => (lowered, Some(alias)),
                                    Some(Err(reason)) => {
                                        super::write_fallback(rust, type_name, &reason);
                                        (lowering.resolve(&ty).map_err(failure)?, None)
                                    }
                                    None => (lowering.resolve(&ty).map_err(failure)?, None),
                                };
                                if integer_null {
                                    write!(rust, "{EXPRESSION}::null_constant(")
                                        .expect("String output");
                                } else if void_null {
                                    write!(rust, "{EXPRESSION}::null::null_void(")
                                        .expect("String output");
                                }
                                if let Some(alias) = alias {
                                    write!(
                                        rust,
                                        "{EXPRESSION}::cast_as::<{alias}, {}, _>(",
                                        ty.marker
                                    )
                                    .expect("String output");
                                } else {
                                    write!(rust, "{EXPRESSION}::cast::<{}, _>(", ty.marker)
                                        .expect("String output");
                                }
                                tasks.extend([
                                    text(if integer_null || void_null { "))" } else { ")" }),
                                    value(*operand),
                                ]);
                            }
                        }
                        ExpressionKind::Unary { operator, operand } => {
                            let helper = match operator {
                                UnaryOperator::Plus => "positive",
                                UnaryOperator::Negate => "neg",
                                UnaryOperator::BitwiseNot => "bitnot",
                                UnaryOperator::LogicalNot => "logical_not",
                            };
                            write!(rust, "{EXPRESSION}::{helper}").expect("String output");
                            if *operator == UnaryOperator::Negate {
                                write!(rust, "::<{SUPPORT}::{policy}, _>").expect("String output");
                            }
                            rust.push('(');
                            tasks.extend([text(")"), value(*operand)]);
                        }
                        ExpressionKind::Binary { operator, left, right } => {
                            if matches!(
                                operator,
                                BinaryOperator::LogicalAnd | BinaryOperator::LogicalOr
                            ) {
                                write!(rust, "{SUPPORT}::CValue::<{SUPPORT}::CInt>::new(if {EXPRESSION}::truth(").expect("String output");
                                tasks.extend([
                                    text(") { 1 } else { 0 })"),
                                    value(*right),
                                    Task::Text(format!(
                                        ") {} {EXPRESSION}::truth(",
                                        if *operator == BinaryOperator::LogicalAnd {
                                            "&&"
                                        } else {
                                            "||"
                                        }
                                    )),
                                    value(*left),
                                ]);
                            } else {
                                write!(rust, "{EXPRESSION}::{}", binary_helper(*operator))
                                    .expect("String output");
                                if matches!(
                                    operator,
                                    BinaryOperator::Add
                                        | BinaryOperator::Subtract
                                        | BinaryOperator::Multiply
                                        | BinaryOperator::ShiftLeft
                                ) {
                                    write!(rust, "::<{SUPPORT}::{policy}, _, _>")
                                        .expect("String output");
                                }
                                rust.push('(');
                                tasks.extend([text(")"), value(*right), text(", "), value(*left)]);
                            }
                        }
                        ExpressionKind::Conditional { condition, then_value, else_value } => {
                            write!(rust, "{EXPRESSION}::select(if {EXPRESSION}::truth(")
                                .expect("String output");
                            tasks.extend([
                                text(") })"),
                                value(*else_value),
                                Task::Text(format!(") }} else {{ {SUPPORT}::Either::Right(")),
                                value(*then_value),
                                Task::Text(format!(") {{ {SUPPORT}::Either::Left(")),
                                value(*condition),
                            ]);
                        }
                        ExpressionKind::Comma { left, right } => {
                            rust.push_str("{ ");
                            tasks.extend([
                                text(" }"),
                                value(*right),
                                text("; "),
                                Task::Node(*left, Context::Discard),
                            ]);
                        }
                        ExpressionKind::Dereference { operand } => {
                            write!(
                                rust,
                                "{EXPRESSION}::{}(",
                                if matches!(context, Context::Value) {
                                    "dereference"
                                } else {
                                    "pointee"
                                }
                            )
                            .expect("String output");
                            tasks.extend([text(")"), value(*operand)]);
                        }
                        ExpressionKind::AddressOf { operand } => {
                            let mut ungrouped = *operand;
                            while let ExpressionKind::Group { operand } =
                                expression.syntax.nodes[ungrouped].kind
                            {
                                ungrouped = operand;
                            }
                            if let ExpressionKind::Dereference { operand } =
                                expression.syntax.nodes[ungrouped].kind
                            {
                                write!(rust, "{EXPRESSION}::dereference_address(")
                                    .expect("String output");
                                tasks.extend([text(")"), value(operand)]);
                            } else {
                                write!(rust, "{EXPRESSION}::address(").expect("String output");
                                tasks.extend([text(")"), place(*operand)]);
                            }
                        }
                        ExpressionKind::Member { base, field, field_parameter, indirect } => {
                            let marker = match field_parameter {
                                Some(parameter) => format!("$crate::__pgrx_c_field_marker!(${})", arguments::name(analysis, *parameter)),
                                None => bindings.field_capabilities.get(field).cloned().ok_or_else(|| skip(analysis, SkipReasonCode::PointerOperation, format!("{field}: no compiler-checked field adapter is available"), Some(node.tokens)))?,
                            };
                            let direct_place = self.places[*base];
                            if !*indirect && !direct_place && matches!(context, Context::Value) {
                                write!(rust, "{EXPRESSION}::member::<{marker}, _>(")
                                    .expect("String output");
                                tasks.extend([text(")"), value(*base)]);
                            } else {
                                if matches!(context, Context::Value) {
                                    write!(rust, "{EXPRESSION}::load(").expect("String output");
                                    tasks.push(text(")"));
                                }
                                write!(rust, "{EXPRESSION}::project::<{marker}, _, _>(")
                                    .expect("String output");
                                tasks.push(text(")"));
                                if *indirect {
                                    write!(rust, "{EXPRESSION}::pointee(").expect("String output");
                                    tasks.extend([text(")"), value(*base)]);
                                } else {
                                    tasks.push(Task::Node(
                                        *base,
                                        if matches!(context, Context::Place) {
                                            Context::Place
                                        } else {
                                            Context::ReadPlace
                                        },
                                    ));
                                }
                            }
                        }
                        ExpressionKind::Index { base, index: offset } => {
                            if matches!(context, Context::Value) {
                                write!(rust, "{EXPRESSION}::load(").expect("String output");
                                tasks.push(text(")"));
                            }
                            write!(rust, "{EXPRESSION}::index(").expect("String output");
                            tasks.extend([text(")"), value(*offset), text(", "), value(*base)]);
                        }
                        ExpressionKind::Assignment {
                            operator,
                            place: destination,
                            value: source,
                        } => {
                            if self.objects[*destination].as_ref().is_some_and(volatile_record) {
                                return Err(failure("volatile whole-record assignment requires a compiler-verified aggregate access primitive".into()));
                            }
                            if let Some(operator) = operator {
                                write!(rust, "{EXPRESSION}::modify(").expect("String output");
                                let helper = binary_helper(*operator);
                                let generics = if matches!(
                                    operator,
                                    BinaryOperator::Add
                                        | BinaryOperator::Subtract
                                        | BinaryOperator::Multiply
                                        | BinaryOperator::ShiftLeft
                                ) {
                                    format!("::<{SUPPORT}::{policy}, _, _>")
                                } else {
                                    String::new()
                                };
                                tasks.extend([Task::Text(format!(", |__pgrx_old, __pgrx_rhs| {EXPRESSION}::{helper}{generics}(__pgrx_old, __pgrx_rhs))")), value(*source), text(", "), place(*destination)]);
                            } else {
                                write!(rust, "{EXPRESSION}::assign(").expect("String output");
                                tasks.extend([
                                    text(")"),
                                    value(*source),
                                    text(", "),
                                    place(*destination),
                                ]);
                            }
                        }
                        ExpressionKind::Update { operand, increment, postfix } => {
                            write!(
                                rust,
                                "{EXPRESSION}::{}(",
                                if *postfix { "post_modify" } else { "modify" }
                            )
                            .expect("String output");
                            tasks.extend([Task::Text(format!(", {SUPPORT}::CValue::<{SUPPORT}::CInt>::new(1i32), |__pgrx_old, __pgrx_rhs| {EXPRESSION}::{}::<{SUPPORT}::{policy}, _, _>(__pgrx_old, __pgrx_rhs))", if *increment { "add" } else { "sub" })), place(*operand)]);
                        }
                        ExpressionKind::Call { callee, arguments } => {
                            let mut ungrouped = *callee;
                            while let ExpressionKind::Group { operand } =
                                expression.syntax.nodes[ungrouped].kind
                            {
                                ungrouped = operand;
                            }
                            if let ExpressionKind::Identifier { name } =
                                &expression.syntax.nodes[ungrouped].kind
                                && let Some(builtin) = frontend.declarations().builtins.get(name)
                            {
                                match builtin.kind {
                                    crate::BuiltinKind::ByteSwap { .. } => {
                                        let [argument] = arguments.as_slice() else {
                                            return Err(failure(format!(
                                                "{name}: byte swap requires one operand"
                                            )));
                                        };
                                        let parameters =
                                            builtin.signature.parameters.as_ref().expect(
                                                "compiler proof establishes a fixed prototype",
                                            );
                                        let parameter =
                                            lowering.resolve(&parameters[0]).map_err(failure)?;
                                        let result = lowering
                                            .resolve(&builtin.signature.result)
                                            .map_err(failure)?;
                                        write!(rust, "<{} as {EXPRESSION}::CType>::from_storage(<{} as {EXPRESSION}::CType>::into_storage({EXPRESSION}::implicit::<{}, _>(", result.marker, parameter.marker, parameter.marker).expect("String output");
                                        tasks.extend([text(")).swap_bytes())"), value(*argument)]);
                                    }
                                    crate::BuiltinKind::Expect => {
                                        let [first, expected] = arguments.as_slice() else {
                                            return Err(failure(format!(
                                                "{name}: branch expectation requires two operands"
                                            )));
                                        };
                                        let parameters =
                                            builtin.signature.parameters.as_ref().expect(
                                                "compiler proof establishes a fixed prototype",
                                            );
                                        let first_type =
                                            lowering.resolve(&parameters[0]).map_err(failure)?;
                                        let expected_type =
                                            lowering.resolve(&parameters[1]).map_err(failure)?;
                                        // Clang evaluates both converted operands, including the
                                        // expected value, before returning the first. The hint has
                                        // no value effect, but its operand can have side effects.
                                        write!(rust, "{{ let __pgrx_c_expect_result = {EXPRESSION}::implicit::<{}, _>(", first_type.marker).expect("String output");
                                        tasks.extend([
                                            text("); __pgrx_c_expect_result }"),
                                            value(*expected),
                                            Task::Text(format!(
                                                "); let _ = {EXPRESSION}::implicit::<{}, _>(",
                                                expected_type.marker
                                            )),
                                            value(*first),
                                        ]);
                                    }
                                }
                                continue;
                            }
                            let direct = match &expression.syntax.nodes[ungrouped].kind {
                                ExpressionKind::Identifier { name }
                                    if frontend
                                        .declarations()
                                        .function_signatures
                                        .contains_key(name) =>
                                {
                                    Some(name)
                                }
                                _ => None,
                            };
                            let Some(name) = direct else {
                                write!(rust, "{EXPRESSION}::invoke(").expect("String output");
                                tasks.push(text("))"));
                                for argument in arguments.iter().rev() {
                                    tasks.extend([text(", "), value(*argument)]);
                                }
                                tasks.extend([text(", ("), value(*callee)]);
                                continue;
                            };
                            let declared =
                                &frontend.declarations().function_signatures[name].signature;
                            let binding = bindings.functions.get(name).ok_or_else(|| skip(analysis, SkipReasonCode::Call, format!("{name}: {}", bindings.function_unavailable.get(name).map(String::as_str).unwrap_or("no callable binding or generated C inline shim was emitted")), Some(node.tokens)))?;
                            if binding.variadic
                                || binding.parameters.len() != arguments.len()
                                || !matches!(binding.abi.as_str(), "C" | "C-unwind")
                            {
                                return Err(failure(format!(
                                    "{name}: binding does not match a fixed C ABI prototype"
                                )));
                            }
                            let parameters = declared
                                .parameters
                                .as_ref()
                                .expect("analysis establishes prototype");
                            let result = if declared.result.category == TypeCategory::Void {
                                if binding.result != crate::RustBindingType::Unit {
                                    return Err(failure(format!(
                                        "{name}: C void return differs from bindgen return"
                                    )));
                                }
                                None
                            } else {
                                Some(
                                    lowering
                                        .resolve_with_storage(&declared.result, &binding.result)
                                        .map_err(failure)?,
                                )
                            };
                            let path = rust_path(&binding.path).map_err(failure)?;
                            if let Some(result) = &result {
                                write!(
                                    rust,
                                    "<{} as {EXPRESSION}::CType>::from_storage(",
                                    result.marker
                                )
                                .expect("String output");
                                tasks.push(text(")"));
                            }
                            write!(rust, "{path}(").expect("String output");
                            tasks.push(text(")"));
                            for ((argument, c_parameter), rust_parameter) in
                                arguments.iter().zip(parameters).zip(&binding.parameters).rev()
                            {
                                let lowered = lowering
                                    .resolve_with_storage(c_parameter, rust_parameter)
                                    .map_err(failure)?;
                                tasks.push(text(", "));
                                tasks.push(text("))"));
                                tasks.push(value(*argument));
                                tasks.push(Task::Text(format!("<{} as {EXPRESSION}::CType>::into_storage({EXPRESSION}::implicit::<{}, _>(", lowered.marker, lowered.marker)));
                            }
                        }
                        ExpressionKind::SizeOfType { type_name }
                        | ExpressionKind::AlignOfType { type_name } => {
                            let ty = crate::analysis::resolve_type_info(
                                type_name,
                                frontend.declarations(),
                                &frontend.profile().target,
                            )
                            .ok_or_else(|| {
                                failure(format!("{type_name}: unresolved type operand"))
                            })?;
                            if ty.category == TypeCategory::Void
                                || ty.category == TypeCategory::Function
                            {
                                return Err(failure(
                                    "sizeof/alignment requires a complete object type".into(),
                                ));
                            }
                            let lowered = lowering.resolve(&ty).map_err(failure)?;
                            write!(
                                rust,
                                "{EXPRESSION}::{}::<{}>()",
                                if matches!(node.kind, ExpressionKind::SizeOfType { .. }) {
                                    "size_of"
                                } else {
                                    "align_of"
                                },
                                lowered.marker
                            )
                            .expect("String output");
                        }
                        ExpressionKind::OffsetOf { record, fields } => {
                            let (marker, mut canonical) = match record {
                                OffsetRecord::Parameter { index } => (
                                    format!(
                                        "<${} as {EXPRESSION}::NativeType>::Marker",
                                        arguments::name(analysis, *index)
                                    ),
                                    None,
                                ),
                                OffsetRecord::Named { name } => {
                                    let ty = crate::analysis::resolve_type_info(
                                        name,
                                        frontend.declarations(),
                                        &frontend.profile().target,
                                    )
                                    .ok_or_else(|| {
                                        failure(format!("{name}: unresolved offsetof type"))
                                    })?;
                                    let binding = lowering.record_binding(&ty).map_err(failure)?;
                                    (
                                        format!(
                                            "{EXPRESSION}::CRecord<{}>",
                                            rust_path(&binding.path).map_err(failure)?
                                        ),
                                        Some(ty.canonical_spelling),
                                    )
                                }
                            };
                            let mut path = Vec::with_capacity(fields.len());
                            for (position, field) in fields.iter().enumerate() {
                                match field {
                                    OffsetComponent::Named { name } => {
                                        if let Some(record) = &canonical {
                                            let key = format!("{record}::{name}");
                                            let member = bindings.offset_capabilities.get(&key).ok_or_else(|| failure(bindings.offset_unavailable.get(&key).cloned().unwrap_or_else(|| format!("{key}: no compiler-verified offset capability"))))?;
                                            if member.is_none() && position + 1 != fields.len() {
                                                return Err(failure(format!(
                                                    "{key}: offsetof member continuation requires a record"
                                                )));
                                            }
                                            canonical = member.clone();
                                        }
                                        path.push(super::field_identifier(name).ok_or_else(|| failure(format!("{name}: offsetof member has no supported identifier")))?.to_owned());
                                    }
                                    OffsetComponent::Parameter { index } => {
                                        canonical = None;
                                        path.push(format!(
                                            "${}",
                                            arguments::name(analysis, *index)
                                        ));
                                    }
                                }
                            }
                            write!(rust, "{EXPRESSION}::offset_of::<{marker}, $crate::__pgrx_c_field_marker!(@path; {})>()", path.join(" . ")).expect("String output");
                        }
                        ExpressionKind::SizeOfExpression { operand } => {
                            tasks.push(Task::Node(*operand, Context::Size));
                        }
                        ExpressionKind::TypeParameterCast {
                            parameter,
                            pointers,
                            is_const,
                            operand,
                        } => {
                            let mut marker = format!(
                                "<${} as {EXPRESSION}::NativeType>::Marker",
                                arguments::name(analysis, *parameter)
                            );
                            for depth in 0..*pointers {
                                marker = format!(
                                    "{EXPRESSION}::CPointer<{marker}, {EXPRESSION}::{}>",
                                    if depth == 0 && *is_const { "ReadOnly" } else { "ReadWrite" }
                                );
                            }
                            write!(rust, "{EXPRESSION}::cast::<{marker}, _>(")
                                .expect("String output");
                            tasks.extend([text(")"), value(*operand)]);
                        }
                        ExpressionKind::SizeOfTypeParameter { parameter, pointers, is_const }
                        | ExpressionKind::AlignOfTypeParameter { parameter, pointers, is_const } => {
                            let mut marker = format!(
                                "<${} as {EXPRESSION}::NativeType>::Marker",
                                arguments::name(analysis, *parameter)
                            );
                            for depth in 0..*pointers {
                                marker = format!(
                                    "{EXPRESSION}::CPointer<{marker}, {EXPRESSION}::{}>",
                                    if depth == 0 && *is_const { "ReadOnly" } else { "ReadWrite" }
                                );
                            }
                            write!(
                                rust,
                                "{EXPRESSION}::{}::<{marker}>()",
                                if matches!(node.kind, ExpressionKind::SizeOfTypeParameter { .. }) {
                                    "size_of"
                                } else {
                                    "align_of"
                                }
                            )
                            .expect("String output");
                        }
                    }
                    if !matches!(context, Context::Value)
                        && !matches!(
                            node.kind,
                            ExpressionKind::Group { .. }
                                | ExpressionKind::Parameter { .. }
                                | ExpressionKind::Identifier { .. }
                                | ExpressionKind::Member { .. }
                                | ExpressionKind::Dereference { .. }
                                | ExpressionKind::Index { .. }
                        )
                    {
                        return Err(failure(
                            "C expression is not a place in this operation".into(),
                        ));
                    }
                }
            }
            if rust.len() > MAX_EMISSION_BYTES {
                return Err(skip(
                    analysis,
                    SkipReasonCode::BudgetExceeded,
                    "generated typed expression exceeds the output budget",
                    None,
                ));
            }
        }
        Ok(())
    }
}

fn volatile_record(ty: &TypeInfo) -> bool {
    ty.category == TypeCategory::Record && ty.is_volatile
}

/// Track only declarations established by the compiler, plus C's structural
/// place/qualifier rules. Unknown caller-supplied type families stay unknown.
/// Children precede parents, so explicit volatile aggregate accesses can be
/// rejected once without recursively rescanning each expression subtree.
fn declared_objects(
    frontend: &FrontendOutput,
    analysis: &MacroAnalysis,
    lowering: &Lowering<'_>,
    locals: &BTreeMap<String, Local>,
) -> Vec<Option<TypeInfo>> {
    let declarations = frontend.declarations();
    let mut objects: Vec<Option<TypeInfo>> = Vec::new();
    let expression = analysis.expression.as_ref().expect("candidate expression");
    for (index, node) in expression.syntax.nodes.iter().enumerate() {
        let ty = match &node.kind {
            ExpressionKind::Group { operand } => objects[*operand].clone(),
            ExpressionKind::Cast { type_name, .. } => crate::analysis::resolve_type_info(
                type_name,
                declarations,
                &frontend.profile().target,
            ),
            ExpressionKind::Identifier { name } => locals
                .get(name)
                .map(|local| local.ty.clone())
                .or_else(|| declarations.variables.get(name).cloned()),
            ExpressionKind::Dereference { operand } => objects[*operand]
                .as_ref()
                .and_then(|pointer| lowering.pointer_pointee(pointer).ok()),
            ExpressionKind::AddressOf { operand } => {
                objects[*operand].as_ref().map(|object| TypeInfo {
                    spelling: format!(
                        "{}{}{} *",
                        if object.is_const { "const " } else { "" },
                        if object.is_volatile { "volatile " } else { "" },
                        object.spelling
                    ),
                    canonical_spelling: format!(
                        "{}{}{} *",
                        if object.is_const { "const " } else { "" },
                        if object.is_volatile { "volatile " } else { "" },
                        object.canonical_spelling
                    ),
                    category: TypeCategory::Pointer,
                    size: Some(u64::from(
                        frontend.profile().target.pointer_bits
                            / frontend.profile().target.char_bits,
                    )),
                    alignment: None,
                    is_const: false,
                    is_volatile: false,
                })
            }
            ExpressionKind::Index { base, .. } => objects[*base].as_ref().and_then(|base| {
                if base.category == TypeCategory::Pointer {
                    lowering.pointer_pointee(base).ok()
                } else if let Some(crate::TypeShape {
                    kind: TypeShapeKind::Array { element, .. },
                    ..
                }) = declarations.type_shapes.get(&base.canonical_spelling)
                {
                    let mut element = element.clone();
                    element.is_const |= base.is_const;
                    element.is_volatile |= base.is_volatile;
                    Some(element)
                } else {
                    None
                }
            }),
            ExpressionKind::Member { base, field, field_parameter: None, indirect } => {
                objects[*base].as_ref().and_then(|base| {
                    let parent =
                        if *indirect { lowering.pointer_pointee(base).ok()? } else { base.clone() };
                    let mut field =
                        declared_field(declarations, &parent.canonical_spelling, field, 0)?;
                    field.is_volatile |= parent.is_volatile;
                    field.is_const |= parent.is_const;
                    Some(field)
                })
            }
            ExpressionKind::Assignment { place, .. } => objects[*place].as_ref().map(|object| {
                let mut result = object.clone();
                result.is_const = false;
                result.is_volatile = false;
                result
            }),
            ExpressionKind::Conditional { then_value, else_value, .. } => objects[*then_value]
                .as_ref()
                .zip(objects[*else_value].as_ref())
                .and_then(|(left, right)| declared_common_object(lowering, left, right)),
            ExpressionKind::Comma { right, .. } => objects[*right].as_ref().map(|object| {
                let mut result = object.clone();
                result.is_const = false;
                result.is_volatile = false;
                result
            }),
            _ => match &expression.types[index] {
                crate::analysis::TypeExpression::External { ty } => Some(ty.clone()),
                _ => None,
            },
        };
        objects.push(ty);
    }
    objects
}

fn declared_common_object(
    lowering: &Lowering<'_>,
    left: &TypeInfo,
    right: &TypeInfo,
) -> Option<TypeInfo> {
    if left == right {
        let mut result = left.clone();
        result.is_const = false;
        result.is_volatile = false;
        return Some(result);
    }
    if left.category != TypeCategory::Pointer || right.category != TypeCategory::Pointer {
        return None;
    }
    let left_pointee = lowering.pointer_pointee(left).ok()?;
    let right_pointee = lowering.pointer_pointee(right).ok()?;
    if left_pointee.category != TypeCategory::Record
        || right_pointee.category != TypeCategory::Record
    {
        return None;
    }
    let unqualified = |ty: &TypeInfo| {
        ty.canonical_spelling
            .split_whitespace()
            .filter(|word| !matches!(*word, "const" | "volatile"))
            .collect::<Vec<_>>()
            .join(" ")
    };
    let canonical = unqualified(&left_pointee);
    if canonical != unqualified(&right_pointee) {
        return None;
    }
    // C's conditional operator combines the qualifiers of compatible pointees.
    // This proof is deliberately limited to compiler-established record identity.
    let mut result = left.clone();
    result.canonical_spelling = format!(
        "{}{}{} *",
        if left_pointee.is_const || right_pointee.is_const { "const " } else { "" },
        if left_pointee.is_volatile || right_pointee.is_volatile { "volatile " } else { "" },
        canonical
    );
    result.spelling = result.canonical_spelling.clone();
    result.is_const = false;
    result.is_volatile = false;
    Some(result)
}

fn declared_field(
    declarations: &crate::DeclarationCatalog,
    canonical: &str,
    name: &str,
    depth: usize,
) -> Option<TypeInfo> {
    if depth > 64 {
        return None;
    }
    let record = declarations.records.get(canonical)?;
    if let Some(field) = record.fields.iter().find(|field| field.name.as_deref() == Some(name)) {
        return Some(field.ty.clone());
    }
    let mut candidates =
        record.fields.iter().filter(|field| field.name.is_none()).filter_map(|field| {
            let mut found =
                declared_field(declarations, &field.ty.canonical_spelling, name, depth + 1)?;
            found.is_volatile |= field.ty.is_volatile;
            found.is_const |= field.ty.is_const;
            Some(found)
        });
    let first = candidates.next()?;
    candidates.next().is_none().then_some(first)
}

/// A bounded proof of integer-constant-expression zero, before any conversion.
/// A zero runtime value, a pointer cast or a comma expression cannot establish
/// the source identity that permits C's null-pointer-constant conversions.
fn proved_integer_zero(
    frontend: &FrontendOutput,
    analysis: &MacroAnalysis,
    constants: &[Option<&ResolvedConstant>],
    mut root: NodeId,
) -> bool {
    let syntax = &analysis.expression.as_ref().expect("candidate expression").syntax;
    loop {
        if analysis
            .expression
            .as_ref()
            .expect("candidate expression")
            .integer_zero_constants
            .contains(&root)
        {
            return true;
        }
        match &syntax.nodes[root].kind {
            ExpressionKind::Group { operand } => root = *operand,
            ExpressionKind::Cast { type_name, operand }
                if crate::analysis::resolve_type_info(
                    type_name,
                    frontend.declarations(),
                    &frontend.profile().target,
                )
                .is_some_and(|ty| matches!(ty.category, TypeCategory::Integer(_))) =>
            {
                root = *operand
            }
            ExpressionKind::IntegerLiteral { literal } => return literal.value == 0,
            ExpressionKind::Identifier { .. } => {
                return constants[root].is_some_and(|constant| {
                    matches!(
                        constant.value,
                        crate::IntegerValue::Signed(0) | crate::IntegerValue::Unsigned(0)
                    )
                });
            }
            _ => return false,
        }
    }
}

fn is_unqualified_void_pointer(frontend: &FrontendOutput, ty: &crate::TypeInfo) -> bool {
    if ty.category != TypeCategory::Pointer {
        return false;
    }
    let pointee = match frontend.declarations().type_shapes.get(&ty.canonical_spelling) {
        Some(crate::TypeShape { kind: crate::TypeShapeKind::Pointer { pointee }, .. }) => {
            pointee.clone()
        }
        _ => {
            let Some((name, _)) = ty.canonical_spelling.rsplit_once('*') else {
                return false;
            };
            let Some(pointee) = crate::analysis::resolve_type_info(
                name.trim(),
                frontend.declarations(),
                &frontend.profile().target,
            ) else {
                return false;
            };
            pointee
        }
    };
    pointee.category == TypeCategory::Void && !pointee.is_const && !pointee.is_volatile
}
