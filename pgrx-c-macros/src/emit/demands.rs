//LICENSE Portions Copyright 2026 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.

//! Capability requests established by C declarations and expression constraints.
//!
//! Caller operands remain type families. Structural constraints narrow those
//! families without observing any invocation or prescribing PostgreSQL callers.
//!
//! Demand planning determines which native adapters a batch can actually use. It
//! propagates operator and prototype constraints through the analyzed expression
//! arena, indexes promoted fields, and caches qualified compatibility queries.
//! Missing facts retain possible capabilities; they never justify excluding a valid
//! caller type or replace the checks performed by lowering.

use super::{BindingCatalog, callbacks, enumerations, fields, typed, types::Lowering};
use crate::analysis::{AnalysisStatus, MacroAnalysis};
use crate::syntax::{
    BinaryOperator, ExpressionKind, NodeId, OffsetComponent, OffsetRecord, UnaryOperator,
};
use crate::{
    AnalysisSession, DeclarationCatalog, FrontendOutput, TargetFacts, TypeCategory, TypeInfo,
    TypeShapeKind,
};
use std::cell::RefCell;
use std::collections::{BTreeMap, BTreeSet, VecDeque};
use std::rc::Rc;

/// Batch requirements consumed by native adapter generation rather than inferred from sample invocations.
#[derive(Default, PartialEq, Eq)]
pub(super) struct Requests {
    /// Member projections and offsetof paths selected from compiler record identities.
    pub fields: fields::FieldRequests,
    /// Declarations needed as calls, addresses, or callback storage witnesses.
    pub functions: BTreeSet<String>,
    /// Functions whose native call body is required, distinct from identity-only use.
    pub called_functions: BTreeSet<String>,
    /// Functions whose original C address must be exposed rather than a callable thunk.
    pub addresses: BTreeSet<String>,
    /// Concrete C types whose native marker and storage support must remain available.
    pub types: BTreeMap<String, TypeInfo>,
    /// Exact and open-family callback operations retained after structural constraints.
    pub callbacks: callbacks::CallbackRequests,
    /// Enum identities and raw native operands that require checked representation bridges.
    pub enums: enumerations::EnumRequests,
}

/// Derive the capability union for admitted roots using declarations and conservative type constraints.
///
/// Unknown caller operands keep the relevant family open; known operator and
/// prototype facts narrow it without prescribing one concrete invocation type.
pub(super) fn plan(
    session: &AnalysisSession<'_>,
    names: &[impl AsRef<str>],
    bindings: &BindingCatalog,
) -> Requests {
    let frontend = session.frontend();
    let lowering = Lowering::new(frontend.declarations(), bindings, &frontend.profile().target);
    let mut requests = Requests::default();
    let records = RecordIndex::new(frontend.declarations());
    let compatibility = Compatibility::new(frontend.declarations(), &frontend.profile().target);
    let mut functions_by_arity = BTreeMap::<usize, Vec<_>>::new();
    let mut integer_types = BTreeMap::new();
    for (canonical, shape) in &frontend.declarations().type_shapes {
        if let TypeShapeKind::Function { signature } = &shape.kind
            && let Some(parameters) = &signature.parameters
        {
            functions_by_arity.entry(parameters.len()).or_default().push((canonical, signature));
        }
        if let TypeCategory::Integer(kind) = shape.ty.category {
            integer_types.entry(kind).or_insert(&shape.ty);
        }
    }
    let mut open_records = false;
    for name in names {
        let analysis = session.analyze(name.as_ref());
        if !matches!(analysis.status, AnalysisStatus::Candidate)
            || super::binding_mismatch(session, &analysis, bindings).is_some()
        {
            continue;
        }
        let Some(expression) = &analysis.expression else { continue };
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
            .collect();
        let objects = typed::declared_objects(frontend, &analysis, &lowering, &locals);
        let constraints = Constraints::new(frontend, &analysis, &objects, &records, &compatibility);
        let mut callees = BTreeSet::new();
        for node in &expression.syntax.nodes {
            if let ExpressionKind::Call { callee, .. } = node.kind {
                callees.insert(constraints.representative[callee]);
            }
        }
        for (index, node) in expression.syntax.nodes.iter().enumerate() {
            match &node.kind {
                ExpressionKind::Member { base, field, field_parameter, indirect } => {
                    let field_name = field_parameter.is_none().then_some(field.as_str());
                    let owner = objects[*base].as_ref().and_then(|ty| {
                        if *indirect { lowering.pointer_pointee(ty).ok() } else { Some(ty.clone()) }
                    });
                    if let Some(owner) = owner.filter(|ty| ty.category == TypeCategory::Record) {
                        if let Some(field) = field_name {
                            requests.fields.field(Some(&owner.canonical_spelling), Some(field));
                        } else if let Some(fields) = records.fields.get(&owner.canonical_spelling) {
                            for (field, ty) in fields {
                                if constraints.accepts(index, ty) {
                                    requests
                                        .fields
                                        .field(Some(&owner.canonical_spelling), Some(field));
                                }
                            }
                        }
                    } else {
                        constraints.request_fields(
                            index,
                            *base,
                            *indirect,
                            field_name,
                            constraints.sources(*base, &expression.syntax, &objects, 0).as_deref(),
                            &mut requests.fields,
                        );
                    }
                }
                ExpressionKind::OffsetOf { record, fields } => {
                    let mut owner = match record {
                        OffsetRecord::Named { name } => crate::analysis::resolve_type_info(
                            name,
                            frontend.declarations(),
                            &frontend.profile().target,
                        ),
                        OffsetRecord::Parameter { .. } => None,
                    };
                    if let Some(ty) = &owner {
                        requests.types.insert(ty.canonical_spelling.clone(), ty.clone());
                    }
                    for field in fields {
                        match field {
                            OffsetComponent::Named { name } => {
                                requests.fields.offset(
                                    owner.as_ref().map(|ty| ty.canonical_spelling.as_str()),
                                    Some(name),
                                );
                                owner = owner.as_ref().and_then(|ty| {
                                    typed::declared_field(
                                        frontend.declarations(),
                                        &ty.canonical_spelling,
                                        name,
                                        0,
                                    )
                                });
                            }
                            OffsetComponent::Parameter { .. } => {
                                requests.fields.offset(
                                    owner.as_ref().map(|ty| ty.canonical_spelling.as_str()),
                                    None,
                                );
                                // A caller field designator can contain a whole dotted path;
                                // the owner-scoped offset request closes its by-value records.
                                break;
                            }
                        }
                    }
                }
                ExpressionKind::Cast { type_name, .. }
                | ExpressionKind::SizeOfType { type_name }
                | ExpressionKind::AlignOfType { type_name } => {
                    if let Some(ty) = crate::analysis::resolve_type_info(
                        type_name,
                        frontend.declarations(),
                        &frontend.profile().target,
                    ) {
                        requests.types.insert(ty.canonical_spelling.clone(), ty);
                    }
                }
                ExpressionKind::SizeOfTypeParameter { .. }
                | ExpressionKind::AlignOfTypeParameter { .. }
                | ExpressionKind::TypeParameterCast { .. } => {
                    // The caller's type is not a nominal declaration in this macro.
                    // Every compiler-owned native type remains an eligible operand.
                    requests.enums.open_scalar = true;
                    for shape in frontend
                        .declarations()
                        .type_shapes
                        .values()
                        .filter(|shape| shape.ty.category == TypeCategory::Record)
                    {
                        requests
                            .types
                            .insert(shape.ty.canonical_spelling.clone(), shape.ty.clone());
                    }
                }
                ExpressionKind::Identifier { name } => {
                    if let Some(ty) = frontend.declarations().variables.get(name) {
                        requests.types.insert(ty.canonical_spelling.clone(), ty.clone());
                    }
                    if let Some(function) = frontend.declarations().function_signatures.get(name) {
                        requests.functions.insert(name.clone());
                        if callees.contains(&constraints.representative[index]) {
                            // Calls under sizeof remain present in Rust's unevaluated
                            // typechecking branch and still require callable support.
                            requests.called_functions.insert(name.clone());
                            requests.types.insert(
                                function.signature.result.canonical_spelling.clone(),
                                function.signature.result.clone(),
                            );
                            for ty in function.signature.parameters.iter().flatten() {
                                requests.types.insert(ty.canonical_spelling.clone(), ty.clone());
                            }
                        } else {
                            requests.addresses.insert(name.clone());
                            if let Some(ty) = frontend.declarations().functions.get(name) {
                                requests
                                    .callbacks
                                    .identity_types
                                    .insert(ty.canonical_spelling.clone());
                            }
                        }
                    }
                }
                ExpressionKind::Call { callee, arguments } => {
                    let direct =
                        match &expression.syntax.nodes[constraints.representative[*callee]].kind {
                            ExpressionKind::Identifier { name } => {
                                frontend.declarations().function_signatures.contains_key(name)
                                    || frontend.declarations().builtins.contains_key(name)
                            }
                            _ => false,
                        };
                    if !direct {
                        if let Some(ty) = &objects[*callee] {
                            requests.callbacks.call_types.insert(ty.canonical_spelling.clone());
                        } else if let Some(types) =
                            constraints.sources(*callee, &expression.syntax, &objects, 0)
                        {
                            requests
                                .callbacks
                                .call_types
                                .extend(types.values().map(|ty| ty.canonical_spelling.clone()));
                        } else {
                            let arguments = arguments
                                .iter()
                                .map(|argument| {
                                    let sources = constraints
                                        .sources(*argument, &expression.syntax, &objects, 0)
                                        .or_else(|| {
                                            let crate::analysis::TypeExpression::Concrete { ty } =
                                                &expression.types[*argument]
                                            else {
                                                return None;
                                            };
                                            let ty = *integer_types.get(&ty.kind)?;
                                            Some(Rc::new(
                                                [(QualifiedType::new(ty), ty.clone())].into(),
                                            ))
                                        });
                                    (sources, possible_null_constant(&expression.syntax, *argument))
                                })
                                .collect::<Vec<_>>();
                            let excluded = functions_by_arity
                                .get(&arguments.len())
                                .filter(|_| {
                                    arguments.iter().any(|(sources, _)| sources.is_some())
                                        || !constraints.requirements
                                            [constraints.representative[index]]
                                            .is_empty()
                                })
                                .into_iter()
                                .flatten()
                                .filter(|(_, signature)| {
                                    !constraints.accepts(index, &signature.result)
                                        || signature
                                            .parameters
                                            .iter()
                                            .flatten()
                                            .zip(&arguments)
                                            .any(|(target, (sources, null_possible))| {
                                                sources.as_ref().is_some_and(|sources| {
                                                    sources.values().all(|source| {
                                                        compatibility.implicit(
                                                            source,
                                                            target,
                                                            *null_possible,
                                                        ) == Some(false)
                                                    })
                                                })
                                            })
                                })
                                .map(|(canonical, _)| (*canonical).clone())
                                .collect::<BTreeSet<_>>();
                            // Admitted signatures are the union over call sites;
                            // therefore exclusions intersect, including an empty
                            // set when an unconstrained site needs the whole family.
                            requests
                                .callbacks
                                .open_calls
                                .entry(arguments.len())
                                .and_modify(|previous| {
                                    previous.retain(|canonical| excluded.contains(canonical));
                                })
                                .or_insert(excluded);
                        }
                    }
                }
                _ => {}
            }
        }
        // Scalar and pointer operands require native bridges even when their
        // nominal type is absent from the macro's replacement text. Evaluate
        // the same structural constraints used for field demands; do not retain
        // unrelated records solely because another record shares a field name.
        let mut seen_parameters = BTreeSet::new();
        for (index, node) in expression.syntax.nodes.iter().enumerate() {
            if !matches!(node.kind, ExpressionKind::Parameter { .. })
                || !seen_parameters.insert(constraints.representative[index])
            {
                continue;
            }
            if !open_records {
                if constraints.open_pointer(index) {
                    open_records = true;
                    for shape in frontend
                        .declarations()
                        .type_shapes
                        .values()
                        .filter(|shape| shape.ty.category == TypeCategory::Record)
                    {
                        requests
                            .types
                            .insert(shape.ty.canonical_spelling.clone(), shape.ty.clone());
                    }
                } else {
                    for canonical in records.fields.keys() {
                        let Some(shape) = frontend.declarations().type_shapes.get(canonical) else {
                            continue;
                        };
                        if constraints.accepts(index, &pointer_to(&shape.ty)) {
                            requests
                                .types
                                .insert(shape.ty.canonical_spelling.clone(), shape.ty.clone());
                        }
                    }
                }
            }
            // Open scalar witnesses retain native enum and function identities
            // only where their supported binding representation admits them.
            if !requests.enums.open_scalar {
                for shape in frontend
                    .declarations()
                    .type_shapes
                    .values()
                    .filter(|shape| shape.ty.category == TypeCategory::Enum)
                {
                    if matches!(lowering.enum_binding(&shape.ty), Ok(Some(_)))
                        && constraints.accepts(index, &pointer_to(&shape.ty))
                    {
                        requests.enums.types.insert(super::types::enum_key(&shape.ty));
                    }
                }
            }
            if constraints.accepts_category(index, TypeCategory::Enum) {
                requests.enums.open_scalar = true;
            }
            if !requests.callbacks.open_identity {
                for shape in frontend
                    .declarations()
                    .type_shapes
                    .values()
                    .filter(|shape| shape.ty.category == TypeCategory::Function)
                {
                    if constraints.accepts(index, &pointer_to(&shape.ty)) {
                        requests
                            .callbacks
                            .identity_types
                            .insert(shape.ty.canonical_spelling.clone());
                        requests
                            .callbacks
                            .native_input_types
                            .insert(shape.ty.canonical_spelling.clone());
                    }
                }
            }
            if constraints.accepts_category(index, TypeCategory::Function) {
                // This category proof admits every native function pointer;
                // the preceding scan has therefore requested the whole family
                // for both tagged identities and raw native inputs.
                requests.callbacks.open_identity = true;
            }
        }
    }
    for ty in requests.fields.required_types(frontend.declarations()) {
        requests.types.insert(ty.canonical_spelling.clone(), ty);
    }
    type_dependencies(
        frontend.declarations(),
        &lowering,
        &requests.types.values().cloned().collect::<Vec<_>>(),
        &mut requests.enums.types,
        &mut requests.callbacks.identity_types,
    );
    requests
}

/// Construct pointer facts for a finite source family while preserving the object’s pointee qualifiers.
fn pointer_to(ty: &TypeInfo) -> TypeInfo {
    TypeInfo {
        spelling: format!("{} *", ty.spelling),
        canonical_spelling: format!("{} *", ty.canonical_spelling),
        category: TypeCategory::Pointer,
        size: None,
        alignment: None,
        is_const: false,
        is_volatile: false,
    }
}

/// Conservatively retain syntaxes that could prove a C null pointer constant during lowering.
///
/// This predicate is a pruning bound, not the final constant-expression proof.
fn possible_null_constant(expression: &crate::syntax::Expression, mut node: NodeId) -> bool {
    while let ExpressionKind::Group { operand } = expression.nodes[node].kind {
        node = operand;
    }
    match &expression.nodes[node].kind {
        ExpressionKind::IntegerLiteral { literal } => literal.value == 0,
        // Unknown caller tokens remain potential integer constant expressions.
        ExpressionKind::Member { .. }
        | ExpressionKind::Index { .. }
        | ExpressionKind::Dereference { .. }
        | ExpressionKind::Update { .. }
        | ExpressionKind::Assignment { .. }
        | ExpressionKind::AddressOf { .. }
        | ExpressionKind::Call { .. }
        | ExpressionKind::Comma { .. } => false,
        _ => true,
    }
}

/// Category bit for C integers and arithmetic enum values.
const INTEGER: u8 = 1;
/// Category bit for modeled C floating values.
const FLOAT: u8 = 2;
/// Category bit for pointers and function values after applicable decay.
const POINTER: u8 = 4;
/// Category bit for by-value record objects.
const RECORD: u8 = 8;
/// Combined integer and floating categories admitted by arithmetic operations.
const ARITHMETIC: u8 = INTEGER | FLOAT;
/// Arithmetic and pointer categories admitted by truth tests and scalar consumers.
const SCALAR: u8 = ARITHMETIC | POINTER;

/// A consumer restriction propagated toward operands without assigning caller-supplied types.
#[derive(Clone)]
enum Requirement {
    /// Restrict a source family to categories permitted by a C operator.
    Kind(
        /// Allowed category bits propagated from a C consumer.
        u8,
    ),
    /// Retain sources capable of implicit conversion to a compiler-owned target type.
    ImplicitTo {
        /// Compiler-proved destination type for implicit conversion.
        target: TypeInfo,
        /// Whether source syntax may establish a C null pointer constant.
        null_possible: bool,
    },
    /// Require a direct or indirect member whose result satisfies downstream constraints.
    Member {
        /// Concrete member name, or None for a caller-supplied designator family.
        name: Option<String>,
        /// Member result node whose consumer constraints must also hold.
        output: NodeId,
        /// Whether owner candidates must first dereference a pointer.
        indirect: bool,
    },
    /// Require a dereferenceable element satisfying the result’s consumer restrictions.
    Element {
        /// Dereference result node whose accepted source family constrains the pointee.
        output: NodeId,
    },
    /// Retain both C subscript orientations while relating the pointer, integer, and result.
    Subscript {
        /// Opposite subscript operand, retaining both C pointer/integer orientations.
        other: NodeId,
        /// Element result node whose requirements constrain admitted pointer families.
        output: NodeId,
    },
    /// Require a callable prototype of the selected arity with a compatible result family.
    Call {
        /// Argument count required by the analyzed call site.
        arity: usize,
        /// Call result node whose downstream consumers constrain possible prototypes.
        output: NodeId,
    },
}

/// Per-expression consumer constraints and cached finite source families.
///
/// The arena orders children before consumers, so acceptance queries can follow
/// result requirements without cycling through recursive compiler record types.
struct Constraints<'a> {
    /// Immutable compiler shapes used to prove object, pointer, and prototype relationships.
    declarations: &'a DeclarationCatalog,
    /// Arena representatives that share constraints across grouping and equivalent nodes.
    representative: Vec<NodeId>,
    /// Consumer restrictions accumulated for each representative node.
    requirements: Vec<Vec<Requirement>>,
    /// Batch-wide promoted-field index reused by member-owner queries.
    records: &'a RecordIndex,
    /// Batch-wide qualified C compatibility cache used only to exclude proven mismatches.
    compatibility: &'a Compatibility<'a>,
    /// Memoized acceptance of a qualified source type at an expression representative.
    accepted: RefCell<BTreeMap<(NodeId, QualifiedType), bool>>,
    /// Cached finite source families; None preserves an unresolved caller family.
    sources: RefCell<BTreeMap<NodeId, SourceFamily>>,
}

/// Qualified concrete C source types for a family proved finite by structural facts.
type SourceTypes = BTreeMap<QualifiedType, TypeInfo>;
/// Shared immutable finite family, or None when compiler facts cannot close the family.
type SourceFamily = Option<Rc<SourceTypes>>;

/// Build and query conservative operand-family constraints while sharing batch-wide indexes.
impl<'a> Constraints<'a> {
    /// Build representatives and consumer constraints from the analyzed expression and known objects.
    fn new(
        frontend: &'a FrontendOutput,
        analysis: &MacroAnalysis,
        objects: &[Option<TypeInfo>],
        records: &'a RecordIndex,
        compatibility: &'a Compatibility<'a>,
    ) -> Self {
        let expression = analysis.expression.as_ref().expect("candidate expression");
        let mut representative = Vec::with_capacity(expression.syntax.nodes.len());
        let mut parameters = BTreeMap::new();
        let mut members = BTreeMap::new();
        for (index, node) in expression.syntax.nodes.iter().enumerate() {
            let root = match &node.kind {
                ExpressionKind::Group { operand } => representative[*operand],
                ExpressionKind::Parameter { index: parameter } => {
                    *parameters.entry(*parameter).or_insert(index)
                }
                ExpressionKind::Member { base, field, field_parameter, indirect } => {
                    let key = (representative[*base], field.clone(), *field_parameter, *indirect);
                    *members.entry(key).or_insert(index)
                }
                _ => index,
            };
            representative.push(root);
        }
        let mut result = Self {
            declarations: frontend.declarations(),
            requirements: vec![Vec::new(); representative.len()],
            representative,
            records,
            compatibility,
            accepted: RefCell::new(BTreeMap::new()),
            sources: RefCell::new(BTreeMap::new()),
        };
        for (index, node) in expression.syntax.nodes.iter().enumerate() {
            match &node.kind {
                ExpressionKind::Member { base, field, field_parameter, indirect } => result
                    .require(
                        *base,
                        Requirement::Member {
                            name: field_parameter.is_none().then(|| field.clone()),
                            output: index,
                            indirect: *indirect,
                        },
                    ),
                ExpressionKind::Index { base, index: offset } => {
                    result.require(*base, Requirement::Subscript { other: *offset, output: index });
                    result.require(*offset, Requirement::Subscript { other: *base, output: index });
                }
                ExpressionKind::Dereference { operand } => {
                    result.require(*operand, Requirement::Element { output: index })
                }
                ExpressionKind::Call { callee, arguments } => {
                    result.require(
                        *callee,
                        Requirement::Call { arity: arguments.len(), output: index },
                    );
                    let named = match &expression.syntax.nodes[result.representative[*callee]].kind
                    {
                        ExpressionKind::Identifier { name } => frontend
                            .declarations()
                            .function_signatures
                            .get(name)
                            .map(|function| &function.signature),
                        _ => None,
                    };
                    let declared = objects[*callee].as_ref().and_then(|ty| {
                        let function = if ty.category == TypeCategory::Function {
                            Some(ty.clone())
                        } else {
                            result.element(ty)
                        }?;
                        match &frontend
                            .declarations()
                            .type_shapes
                            .get(&function.canonical_spelling)?
                            .kind
                        {
                            TypeShapeKind::Function { signature } => Some(signature),
                            _ => None,
                        }
                    });
                    if let Some(parameters) =
                        named.or(declared).and_then(|signature| signature.parameters.as_ref())
                    {
                        for (argument, parameter) in arguments.iter().zip(parameters) {
                            result.implicit(*argument, parameter, &expression.syntax);
                        }
                    }
                }
                ExpressionKind::Assignment { operator: None, place, value } => {
                    if let Some(target) = &objects[*place] {
                        result.implicit(*value, target, &expression.syntax);
                    }
                }
                ExpressionKind::Unary { operator, operand } => result.kind(
                    *operand,
                    match operator {
                        UnaryOperator::Plus | UnaryOperator::Negate => ARITHMETIC,
                        UnaryOperator::BitwiseNot => INTEGER,
                        UnaryOperator::LogicalNot => SCALAR,
                    },
                ),
                ExpressionKind::Binary { operator, left, right } => {
                    let (left_mask, right_mask) = match operator {
                        BinaryOperator::Remainder
                        | BinaryOperator::ShiftLeft
                        | BinaryOperator::ShiftRight
                        | BinaryOperator::BitAnd
                        | BinaryOperator::BitXor
                        | BinaryOperator::BitOr => (INTEGER, INTEGER),
                        BinaryOperator::Multiply | BinaryOperator::Divide => {
                            (ARITHMETIC, ARITHMETIC)
                        }
                        BinaryOperator::Add => (
                            if objects[*right]
                                .as_ref()
                                .is_some_and(|ty| ty.category == TypeCategory::Pointer)
                            {
                                INTEGER
                            } else {
                                SCALAR
                            },
                            if objects[*left]
                                .as_ref()
                                .is_some_and(|ty| ty.category == TypeCategory::Pointer)
                            {
                                INTEGER
                            } else {
                                SCALAR
                            },
                        ),
                        _ => (SCALAR, SCALAR),
                    };
                    result.kind(*left, left_mask);
                    result.kind(*right, right_mask);
                }
                ExpressionKind::Cast { type_name, operand } => {
                    if crate::analysis::resolve_type_info(
                        type_name,
                        frontend.declarations(),
                        &frontend.profile().target,
                    )
                    .is_some_and(|ty| ty.category != TypeCategory::Void)
                    {
                        result.kind(*operand, SCALAR);
                    }
                }
                ExpressionKind::Conditional { condition, .. } => result.kind(*condition, SCALAR),
                ExpressionKind::Update { operand, .. } => result.kind(*operand, SCALAR),
                _ => {}
            }
        }
        for statement in expression.syntax.statement_body.iter().flat_map(|body| body.walk()) {
            if let crate::syntax::Statement::Declaration {
                type_name,
                initializer: Some(initializer),
                ..
            } = statement
                && let Some(target) = crate::analysis::resolve_type_info(
                    type_name,
                    frontend.declarations(),
                    &frontend.profile().target,
                )
            {
                result.implicit(*initializer, &target, &expression.syntax);
            }
        }
        result.propagate_categories(&expression.syntax, objects);
        result
    }

    /// Attach a consumer restriction to the canonical representative of an operand.
    fn require(&mut self, node: NodeId, requirement: Requirement) {
        let category = match &requirement {
            Requirement::Member { indirect, .. } => Some(if *indirect { POINTER } else { RECORD }),
            Requirement::Element { .. } | Requirement::Call { .. } => Some(POINTER),
            Requirement::Subscript { .. } => Some(INTEGER | POINTER),
            _ => None,
        };
        if let Some(mask) = category {
            self.requirements[self.representative[node]].push(Requirement::Kind(mask));
        }
        self.requirements[self.representative[node]].push(requirement);
    }
    /// Add an operator’s allowed category mask to the operand constraints.
    fn kind(&mut self, node: NodeId, mask: u8) {
        self.require(node, Requirement::Kind(mask));
    }
    /// Add a compiler-target conversion restriction while preserving possible null constants.
    fn implicit(
        &mut self,
        node: NodeId,
        target: &TypeInfo,
        expression: &crate::syntax::Expression,
    ) {
        let mask = match target.category {
            TypeCategory::Integer(crate::IntegerKind::Bool) => Some(SCALAR),
            TypeCategory::Integer(_) | TypeCategory::Enum | TypeCategory::Floating => {
                Some(ARITHMETIC)
            }
            TypeCategory::Pointer | TypeCategory::Function => Some(POINTER | INTEGER),
            TypeCategory::Record => Some(RECORD),
            TypeCategory::Void | TypeCategory::Other => None,
        };
        if let Some(mask) = mask {
            self.kind(node, mask);
        }
        let null_possible = possible_null_constant(expression, node);
        self.require(node, Requirement::ImplicitTo { target: target.clone(), null_possible });
    }
    /// Arc consistency over five C value categories. Each domain can lose at
    /// most five bits; only adjacent relations are revisited after a change.
    /// Rank, enum identity and pointer compatibility remain lowering facts.
    fn propagate_categories(
        &mut self,
        expression: &crate::syntax::Expression,
        objects: &[Option<TypeInfo>],
    ) {
        /// Universe of category bits used while propagating conservative operator constraints.
        const ALL: u8 = 31;
        let mut masks = vec![ALL; self.representative.len()];
        let mut relations = Vec::new();
        for (index, node) in expression.nodes.iter().enumerate() {
            let root = self.representative[index];
            for requirement in &self.requirements[root] {
                if let Requirement::Kind(mask) = requirement {
                    masks[root] &= mask;
                }
            }
            if let Some(ty) = &objects[index] {
                masks[root] &= self.mask(ty);
            }
            match &node.kind {
                ExpressionKind::IntegerLiteral { .. }
                | ExpressionKind::SizeOfType { .. }
                | ExpressionKind::SizeOfExpression { .. }
                | ExpressionKind::AlignOfType { .. }
                | ExpressionKind::SizeOfTypeParameter { .. }
                | ExpressionKind::AlignOfTypeParameter { .. }
                | ExpressionKind::OffsetOf { .. } => masks[root] &= INTEGER,
                ExpressionKind::AddressOf { .. } => masks[root] &= POINTER,
                ExpressionKind::Member { field, field_parameter: None, .. } => {
                    let family = self
                        .records
                        .by_name
                        .get(field)
                        .into_iter()
                        .flatten()
                        .filter_map(|owner| self.records.fields[owner].get(field))
                        .fold(0, |mask, ty| mask | self.mask(ty));
                    masks[root] &= family;
                }
                ExpressionKind::Binary { left, right, .. } => relations.push((
                    index,
                    vec![root, self.representative[*left], self.representative[*right]],
                )),
                ExpressionKind::Index { base, index: offset } => relations.push((
                    index,
                    vec![root, self.representative[*base], self.representative[*offset]],
                )),
                ExpressionKind::Conditional { then_value, else_value, .. } => relations.push((
                    index,
                    vec![root, self.representative[*then_value], self.representative[*else_value]],
                )),
                ExpressionKind::Comma { right, .. } => {
                    relations.push((index, vec![root, self.representative[*right]]))
                }
                ExpressionKind::Unary {
                    operator: UnaryOperator::Plus | UnaryOperator::Negate,
                    operand,
                }
                | ExpressionKind::Update { operand, .. } => {
                    relations.push((index, vec![root, self.representative[*operand]]))
                }
                _ => {}
            }
        }
        let mut adjacent = vec![Vec::new(); masks.len()];
        for (relation, (_, nodes)) in relations.iter().enumerate() {
            for node in nodes {
                adjacent[*node].push(relation);
            }
        }
        let mut queue = (0..relations.len()).collect::<VecDeque<_>>();
        let mut queued = vec![true; relations.len()];
        /// Individual category alternatives enumerated when solving binary result restrictions.
        const ATOMS: [u8; 5] = [INTEGER, FLOAT, POINTER, RECORD, 16];
        while let Some(relation) = queue.pop_front() {
            queued[relation] = false;
            let (index, nodes) = &relations[relation];
            let mut allowed = [0u8; 3];
            for left in ATOMS.into_iter().filter(|bit| masks[nodes[1]] & bit != 0) {
                for right in
                    ATOMS.into_iter().filter(|bit| nodes.len() == 2 || masks[nodes[2]] & bit != 0)
                {
                    let output = match &expression.nodes[*index].kind {
                        ExpressionKind::Binary { operator, .. } => {
                            binary_category(*operator, left, right)
                        }
                        ExpressionKind::Conditional { .. } => common_category(left, right),
                        ExpressionKind::Index { .. } => {
                            if matches!((left, right), (INTEGER, POINTER) | (POINTER, INTEGER)) {
                                masks[nodes[0]]
                            } else {
                                0
                            }
                        }
                        _ => left,
                    };
                    if output & masks[nodes[0]] != 0 {
                        allowed[0] |= output;
                        allowed[1] |= left;
                        allowed[2] |= right;
                    }
                }
            }
            for (position, node) in nodes.iter().enumerate() {
                let narrowed = masks[*node] & allowed[position];
                if narrowed != masks[*node] {
                    masks[*node] = narrowed;
                    for adjacent in &adjacent[*node] {
                        if !queued[*adjacent] {
                            queued[*adjacent] = true;
                            queue.push_back(*adjacent);
                        }
                    }
                }
            }
        }
        for (index, mask) in masks.into_iter().enumerate().filter(|(_, mask)| *mask != ALL) {
            self.requirements[index]
                .retain(|requirement| !matches!(requirement, Requirement::Kind(_)));
            self.requirements[index].push(Requirement::Kind(mask));
        }
    }

    /// Detect consumers that leave pointer identities unconstrained, requiring an open family.
    fn open_pointer(&self, node: NodeId) -> bool {
        self.requirements[self.representative[node]].iter().all(|requirement| match requirement {
            Requirement::Kind(mask) => mask & POINTER != 0,
            Requirement::ImplicitTo { target, .. } => {
                target.category == TypeCategory::Integer(crate::IntegerKind::Bool)
                    || matches!(target.category, TypeCategory::Other)
            }
            _ => false,
        })
    }
    /// Check category-level consumers where a complete concrete type is not available.
    fn accepts_category(&self, node: NodeId, category: TypeCategory) -> bool {
        self.requirements[self.representative[node]].iter().all(|requirement| match requirement {
            Requirement::Kind(mask) => mask & category_mask(category) != 0,
            Requirement::ImplicitTo { target, null_possible } => {
                match (category, target.category) {
                    (TypeCategory::Enum, TypeCategory::Pointer | TypeCategory::Function) => {
                        *null_possible
                    }
                    (
                        TypeCategory::Enum,
                        TypeCategory::Integer(_) | TypeCategory::Enum | TypeCategory::Floating,
                    ) => true,
                    (TypeCategory::Function, TypeCategory::Integer(crate::IntegerKind::Bool)) => {
                        true
                    }
                    (_, TypeCategory::Other) => true,
                    _ => false,
                }
            }
            Requirement::Subscript { other, .. } => {
                category_mask(category) == INTEGER && self.allows(*other, POINTER)
            }
            _ => false,
        })
    }

    /// Memoize whether a qualified concrete source satisfies every propagated consumer.
    ///
    /// Unknown compatibility facts retain the source; only established mismatches prune
    /// capabilities that a valid invocation might otherwise require.
    fn accepts(&self, node: NodeId, ty: &TypeInfo) -> bool {
        if ty.category == TypeCategory::Other && self.mask(ty) != POINTER {
            return true;
        }
        let node = self.representative[node];
        let key = (node, QualifiedType::new(ty));
        if let Some(accepted) = self.accepted.borrow().get(&key) {
            return *accepted;
        }
        // Consumer requirements point to later arena representatives. The
        // query graph is acyclic even when compiler record types are recursive.
        let accepted = self.requirements[node].iter().all(|requirement| match requirement {
            Requirement::Kind(mask) => mask & self.mask(ty) != 0,
            Requirement::ImplicitTo { target, null_possible } => {
                self.compatibility.implicit(ty, target, *null_possible).unwrap_or(true)
            }
            Requirement::Subscript { other, output } => {
                if self.mask(ty) == INTEGER {
                    self.allows(*other, POINTER)
                } else if self.mask(ty) == POINTER && self.allows(*other, INTEGER) {
                    self.index_element(ty).is_none_or(|element| self.accepts(*output, &element))
                } else {
                    false
                }
            }
            Requirement::Element { output } => {
                let element = if ty.category == TypeCategory::Function {
                    Some(ty.clone())
                } else {
                    self.element(ty)
                };
                element.is_none_or(|element| self.accepts(*output, &element))
            }
            Requirement::Call { arity, output } => {
                let function = if ty.category == TypeCategory::Function {
                    Some(ty.clone())
                } else {
                    self.element(ty)
                };
                function
                    .and_then(|function| {
                        self.declarations.type_shapes.get(&function.canonical_spelling)
                    })
                    .is_none_or(|shape| match &shape.kind {
                        TypeShapeKind::Function { signature } => {
                            signature
                                .parameters
                                .as_ref()
                                .is_none_or(|parameters| parameters.len() == *arity)
                                && self.accepts(*output, &signature.result)
                        }
                        TypeShapeKind::Unsupported => true,
                        _ => false,
                    })
            }
            Requirement::Member { name, output, indirect } => {
                let owner = if *indirect { self.element(ty) } else { Some(ty.clone()) };
                owner.is_none_or(|owner| {
                    if owner.category == TypeCategory::Other {
                        return true;
                    }
                    owner.category == TypeCategory::Record
                        && self.records.fields.get(&owner.canonical_spelling).is_none_or(|fields| {
                            RecordIndex::select(fields, name.as_deref())
                                .any(|(_, ty)| self.accepts(*output, ty))
                        })
                })
            }
        });
        self.accepted.borrow_mut().insert(key, accepted);
        accepted
    }

    /// Request fields from accepted owner families, falling back to wildcard demand when unresolved.
    fn request_fields(
        &self,
        output: NodeId,
        base: NodeId,
        indirect: bool,
        name: Option<&str>,
        upstream: Option<&SourceTypes>,
        requests: &mut fields::FieldRequests,
    ) {
        let owners = self
            .member_owners(indirect, name, upstream)
            .unwrap_or_else(|| self.broad_member_owners(indirect, name).flatten().collect());
        for (base_ty, owner) in owners {
            if !self.accepts(base, &base_ty) {
                continue;
            }
            for (field, ty) in
                RecordIndex::select(&self.records.fields[&owner.canonical_spelling], name)
            {
                if self.accepts(output, ty) {
                    requests.field(Some(&owner.canonical_spelling), Some(field));
                }
            }
        }
    }

    /// Finite upstream declarations constrain downstream owners. An unknown
    /// caller or missing pointee/record metadata keeps the indexed broad family.
    fn member_owners(
        &self,
        indirect: bool,
        name: Option<&str>,
        upstream: Option<&SourceTypes>,
    ) -> Option<Vec<(TypeInfo, TypeInfo)>> {
        if let Some(upstream) = upstream {
            let mut owners = Vec::new();
            for base in upstream.values() {
                let owner = if indirect {
                    match self.element(base) {
                        Some(owner) => owner,
                        None if base.category == TypeCategory::Pointer
                            || base.category == TypeCategory::Other =>
                        {
                            return None;
                        }
                        None => continue,
                    }
                } else {
                    base.clone()
                };
                if owner.category == TypeCategory::Record {
                    let fields = self.records.fields.get(&owner.canonical_spelling)?;
                    if name.is_none_or(|name| fields.contains_key(name)) {
                        owners.push((base.clone(), owner));
                    }
                } else if owner.category == TypeCategory::Other {
                    return None;
                }
            }
            return Some(owners);
        }
        self.broad_member_owners(indirect, name).collect()
    }

    /// Use the promoted-field index to retain possible owners when source inference is open.
    fn broad_member_owners(
        &self,
        indirect: bool,
        name: Option<&str>,
    ) -> impl Iterator<Item = Option<(TypeInfo, TypeInfo)>> {
        let candidates = match name {
            Some(name) => self.records.by_name.get(name).cloned().unwrap_or_default(),
            None => self.records.fields.keys().cloned().collect(),
        };
        candidates.into_iter().map(move |canonical| {
            let owner = self.declarations.type_shapes.get(&canonical)?.ty.clone();
            let base = if indirect { pointer_to(&owner) } else { owner.clone() };
            Some((base, owner))
        })
    }

    /// Declarations close result families even when the first caller record is
    /// open. Cached immutable families are shared by field and callback demands.
    fn sources(
        &self,
        node: NodeId,
        expression: &crate::syntax::Expression,
        objects: &[Option<TypeInfo>],
        depth: usize,
    ) -> SourceFamily {
        if depth > 64 {
            return None;
        }
        if let Some(cached) = self.sources.borrow().get(&node) {
            return cached.clone();
        }
        let sources = match expression.nodes[node].kind {
            ExpressionKind::Group { operand } => {
                self.sources(operand, expression, objects, depth + 1)
            }
            ExpressionKind::Comma { right, .. } if objects[node].is_none() => {
                self.sources(right, expression, objects, depth + 1).map(|sources| {
                    if sources.values().all(|ty| !ty.is_const && !ty.is_volatile) {
                        return sources;
                    }
                    Rc::new(
                        sources
                            .values()
                            .map(|ty| {
                                let mut ty = ty.clone();
                                ty.is_const = false;
                                ty.is_volatile = false;
                                (QualifiedType::new(&ty), ty)
                            })
                            .collect(),
                    )
                })
            }
            _ => self.infer_sources(node, expression, objects, depth).map(Rc::new),
        };
        self.sources.borrow_mut().insert(node, sources.clone());
        sources
    }

    /// Infer finite families through fields, indexing, dereferences, and call results.
    ///
    /// Any unresolved structural dependency leaves the family open instead of guessing
    /// a single caller type from the available declaration catalog.
    fn infer_sources(
        &self,
        node: NodeId,
        expression: &crate::syntax::Expression,
        objects: &[Option<TypeInfo>],
        depth: usize,
    ) -> Option<SourceTypes> {
        if let Some(ty) = &objects[node] {
            return Some([(QualifiedType::new(ty), ty.clone())].into());
        }
        match &expression.nodes[node].kind {
            ExpressionKind::Member { base, field, field_parameter, indirect } => {
                let name = field_parameter.is_none().then_some(field.as_str());
                let upstream = self.sources(*base, expression, objects, depth + 1);
                let mut result = BTreeMap::new();
                for (base_ty, owner) in self.member_owners(*indirect, name, upstream.as_deref())? {
                    if !self.accepts(*base, &base_ty) {
                        continue;
                    }
                    for (_, ty) in
                        RecordIndex::select(&self.records.fields[&owner.canonical_spelling], name)
                    {
                        let mut ty = ty.clone();
                        ty.is_const |= owner.is_const;
                        ty.is_volatile |= owner.is_volatile;
                        if self.accepts(node, &ty) {
                            result.insert(QualifiedType::new(&ty), ty);
                        }
                    }
                }
                Some(result)
            }
            ExpressionKind::Index { base, index: offset } => {
                let mut result = BTreeMap::new();
                let left = self.sources(*base, expression, objects, depth + 1);
                let right = self.sources(*offset, expression, objects, depth + 1);
                for (operand, other, pointers, integers) in
                    [(*base, *offset, &left, &right), (*offset, *base, &right, &left)]
                {
                    if !self.allows(operand, POINTER) || !self.allows(other, INTEGER) {
                        continue;
                    }
                    if integers.as_ref().is_some_and(|types| {
                        types.values().all(|ty| {
                            self.mask(ty) != INTEGER
                                && (ty.category != TypeCategory::Other || self.mask(ty) == POINTER)
                        })
                    }) {
                        continue;
                    }
                    for ty in pointers.as_ref()?.values() {
                        if self.mask(ty) == POINTER {
                            let element = self.index_element(ty)?;
                            if self.accepts(node, &element) {
                                result.insert(QualifiedType::new(&element), element);
                            }
                        } else if ty.category == TypeCategory::Other {
                            return None;
                        }
                    }
                }
                Some(result)
            }
            ExpressionKind::Dereference { operand } => {
                let mut result = BTreeMap::new();
                for ty in self.sources(*operand, expression, objects, depth + 1)?.values() {
                    let element = if ty.category == TypeCategory::Function {
                        ty.clone()
                    } else {
                        self.element(ty)?
                    };
                    result.insert(QualifiedType::new(&element), element);
                }
                Some(result)
            }
            ExpressionKind::Call { callee, .. } => {
                let mut result = BTreeMap::new();
                for ty in self.sources(*callee, expression, objects, depth + 1)?.values() {
                    let function = if ty.category == TypeCategory::Function {
                        ty.clone()
                    } else {
                        self.element(ty)?
                    };
                    let shape = self.declarations.type_shapes.get(&function.canonical_spelling)?;
                    let TypeShapeKind::Function { signature } = &shape.kind else { return None };
                    if self.accepts(node, &signature.result) {
                        result.insert(
                            QualifiedType::new(&signature.result),
                            signature.result.clone(),
                        );
                    }
                }
                Some(result)
            }
            _ => None,
        }
    }

    /// Classify concrete objects, treating compiler arrays as pointer-capable after decay.
    fn mask(&self, ty: &TypeInfo) -> u8 {
        if matches!(
            self.declarations.type_shapes.get(&ty.canonical_spelling).map(|shape| &shape.kind),
            Some(TypeShapeKind::Array { .. })
        ) {
            POINTER
        } else {
            category_mask(ty.category)
        }
    }
    /// Check only the propagated category restrictions for a candidate category bit.
    fn allows(&self, node: NodeId, category: u8) -> bool {
        self.requirements[self.representative[node]].iter().all(|requirement| match requirement {
            Requirement::Kind(mask) => mask & category != 0,
            _ => true,
        })
    }
    /// Recover a subscript element only when it is neither void nor a function.
    fn index_element(&self, ty: &TypeInfo) -> Option<TypeInfo> {
        self.element(ty).filter(|element| {
            !matches!(element.category, TypeCategory::Void | TypeCategory::Function)
        })
    }
    /// Recover compiler element facts through the shared qualifier-aware compatibility resolver.
    fn element(&self, ty: &TypeInfo) -> Option<TypeInfo> {
        self.compatibility.element(ty)
    }
}

/// Compatibility is a pruning proof, not a substitute for C lowering. Missing
/// compiler facts retain the candidate. Pairs are shared across the whole batch;
/// qualifiers are part of the key because inherited object qualifiers need not
/// change the compiler's original canonical spelling.
struct Compatibility<'a> {
    /// Compiler type shapes required for structural and nominal compatibility proofs.
    declarations: &'a DeclarationCatalog,
    /// Selected target integer and pointer facts used by fallback type resolution.
    target: &'a TargetFacts,
    /// Tri-state compatibility results keyed by both qualified types and top-level treatment.
    pairs: RefCell<BTreeMap<TypePair, Option<bool>>>,
}

/// Cache identity that retains inherited qualifiers even when canonical spelling stays unchanged.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
struct QualifiedType {
    /// Compiler canonical spelling used to recover structural and nominal identity.
    canonical: String,
    /// Effective const qualification, including qualifiers inherited from containing objects.
    is_const: bool,
    /// Effective volatile qualification preserved in compatibility and source-family caches.
    is_volatile: bool,
}

/// Construct ordered cache identities that include effective object qualifiers.
impl QualifiedType {
    /// Capture canonical identity and effective qualifiers as an ordered cache key.
    fn new(ty: &TypeInfo) -> Self {
        Self {
            canonical: ty.canonical_spelling.clone(),
            is_const: ty.is_const,
            is_volatile: ty.is_volatile,
        }
    }
}

/// Directional compatibility query whose qualifier treatment is part of its identity.
#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct TypePair {
    /// Qualified source object before implicit conversion.
    source: QualifiedType,
    /// Qualified destination or comparison type.
    target: QualifiedType,
    /// Whether C value conversion permits ignoring top-level qualifiers for this query.
    ignore_top: bool,
}

/// Memoize compiler type compatibility as a pruning proof with an explicit unknown state.
impl<'a> Compatibility<'a> {
    /// Create one reusable compatibility cache for all macros in the batch.
    fn new(declarations: &'a DeclarationCatalog, target: &'a TargetFacts) -> Self {
        Self { declarations, target, pairs: RefCell::new(BTreeMap::new()) }
    }

    /// Recover pointees or array elements without dropping inherited const or volatile qualifiers.
    fn element(&self, ty: &TypeInfo) -> Option<TypeInfo> {
        match self.declarations.type_shapes.get(&ty.canonical_spelling).map(|shape| &shape.kind) {
            Some(TypeShapeKind::Pointer { pointee }) => Some(pointee.clone()),
            Some(TypeShapeKind::Array { element, .. }) => {
                let mut element = element.clone();
                element.is_const |= ty.is_const;
                element.is_volatile |= ty.is_volatile;
                Some(element)
            }
            _ if ty.category == TypeCategory::Pointer => {
                ty.canonical_spelling.rsplit_once('*').and_then(|(name, _)| {
                    crate::analysis::resolve_type_info(name.trim(), self.declarations, self.target)
                })
            }
            _ => None,
        }
    }

    /// Return a proved implicit-conversion verdict, retaining None when compiler facts are incomplete.
    ///
    /// Pointer compatibility considers null-constant possibility and pointee qualifiers
    /// without using the native Rust representation as a C type identity.
    fn implicit(&self, source: &TypeInfo, target: &TypeInfo, null_possible: bool) -> Option<bool> {
        let array = matches!(
            self.declarations.type_shapes.get(&source.canonical_spelling).map(|shape| &shape.kind),
            Some(TypeShapeKind::Array { .. })
        );
        if source.category == TypeCategory::Other && !array {
            return None;
        }
        match target.category {
            TypeCategory::Integer(crate::IntegerKind::Bool) => Some(
                array
                    || matches!(
                        source.category,
                        TypeCategory::Integer(_)
                            | TypeCategory::Enum
                            | TypeCategory::Floating
                            | TypeCategory::Pointer
                            | TypeCategory::Function
                    ),
            ),
            TypeCategory::Integer(_) | TypeCategory::Enum | TypeCategory::Floating => {
                Some(matches!(
                    source.category,
                    TypeCategory::Integer(_) | TypeCategory::Enum | TypeCategory::Floating
                ))
            }
            TypeCategory::Record => {
                if source.category != TypeCategory::Record {
                    Some(false)
                } else {
                    self.types(source, target, true, 0)
                }
            }
            TypeCategory::Pointer | TypeCategory::Function => {
                if matches!(source.category, TypeCategory::Integer(_) | TypeCategory::Enum) {
                    return Some(null_possible);
                }
                if !array
                    && !matches!(source.category, TypeCategory::Pointer | TypeCategory::Function)
                {
                    return Some(false);
                }
                let source = if source.category == TypeCategory::Function {
                    source.clone()
                } else {
                    self.element(source)?
                };
                let target = if target.category == TypeCategory::Function {
                    target.clone()
                } else {
                    self.element(target)?
                };
                if source.is_const && !target.is_const || source.is_volatile && !target.is_volatile
                {
                    return Some(false);
                }
                if let (Some(source), Some(target)) = (
                    self.declarations.type_shapes.get(&source.canonical_spelling),
                    self.declarations.type_shapes.get(&target.canonical_spelling),
                ) && source.is_restrict
                    && !target.is_restrict
                {
                    return Some(false);
                }
                // The first pointee may gain qualifiers. Nested pointer and
                // array identities still need exactly compatible qualifications.
                if source.category == TypeCategory::Void || target.category == TypeCategory::Void {
                    let other =
                        if source.category == TypeCategory::Void { &target } else { &source };
                    return match other.category {
                        TypeCategory::Function => Some(false),
                        TypeCategory::Other => None,
                        _ => Some(true),
                    };
                }
                self.types(&source, &target, true, 0)
            }
            TypeCategory::Void => Some(source.category == TypeCategory::Void),
            TypeCategory::Other => None,
        }
    }

    /// Cache a directional structural C compatibility query with explicit top-qualifier treatment.
    fn types(
        &self,
        source: &TypeInfo,
        target: &TypeInfo,
        ignore_top: bool,
        depth: usize,
    ) -> Option<bool> {
        if depth > 64 {
            return None;
        }
        let key = TypePair {
            source: QualifiedType::new(source),
            target: QualifiedType::new(target),
            ignore_top,
        };
        if let Some(compatible) = self.pairs.borrow().get(&key) {
            return *compatible;
        }
        let compatible = self.types_at(source, target, ignore_top, depth);
        self.pairs.borrow_mut().insert(key, compatible);
        compatible
    }

    /// Compare compiler types recursively within the depth bound, preserving unknown verdicts.
    fn types_at(
        &self,
        source: &TypeInfo,
        target: &TypeInfo,
        ignore_top: bool,
        depth: usize,
    ) -> Option<bool> {
        let left = self.declarations.type_shapes.get(&source.canonical_spelling);
        let right = self.declarations.type_shapes.get(&target.canonical_spelling);
        if !ignore_top {
            if source.is_const != target.is_const || source.is_volatile != target.is_volatile {
                return Some(false);
            }
            if let (Some(left), Some(right)) = (left, right)
                && left.is_restrict != right.is_restrict
            {
                return Some(false);
            }
        }
        if source.canonical_spelling == target.canonical_spelling {
            return Some(true);
        }
        match (source.category, target.category) {
            (TypeCategory::Integer(left), TypeCategory::Integer(right)) => {
                return Some(left == right);
            }
            (TypeCategory::Enum, TypeCategory::Enum) => {
                return Some(super::types::enum_key(source) == super::types::enum_key(target));
            }
            (TypeCategory::Enum, TypeCategory::Integer(_)) => {
                if let TypeShapeKind::Enum { underlying: Some(underlying) } = &left?.kind {
                    return self.types(underlying, target, true, depth + 1);
                }
                return None;
            }
            (TypeCategory::Integer(_), TypeCategory::Enum) => {
                if let TypeShapeKind::Enum { underlying: Some(underlying) } = &right?.kind {
                    return self.types(source, underlying, true, depth + 1);
                }
                return None;
            }
            (TypeCategory::Floating, TypeCategory::Floating) => {
                return match (source.size, target.size) {
                    (Some(left), Some(right)) if left != right => Some(false),
                    _ => None,
                };
            }
            (TypeCategory::Void, TypeCategory::Void) => return Some(true),
            (TypeCategory::Pointer, TypeCategory::Pointer) => {
                return self.types(
                    &self.element(source)?,
                    &self.element(target)?,
                    false,
                    depth + 1,
                );
            }
            _ => {}
        }
        match (&left?.kind, &right?.kind) {
            (
                TypeShapeKind::Record { identity: left },
                TypeShapeKind::Record { identity: right },
            ) => Some(left == right),
            (
                TypeShapeKind::Array { length: left, .. },
                TypeShapeKind::Array { length: right, .. },
            ) => {
                if left.zip(*right).is_some_and(|(left, right)| left != right) {
                    return Some(false);
                }
                self.types(&self.element(source)?, &self.element(target)?, ignore_top, depth + 1)
            }
            (
                TypeShapeKind::Function { signature: left },
                TypeShapeKind::Function { signature: right },
            ) => {
                if left.variadic != right.variadic
                    || matches!((&left.calling_convention, &right.calling_convention), (Some(left), Some(right)) if left != right)
                {
                    return Some(false);
                }
                let mut unknown =
                    left.calling_convention.is_none() || right.calling_convention.is_none();
                match self.types(&left.result, &right.result, true, depth + 1) {
                    Some(false) => return Some(false),
                    None => unknown = true,
                    _ => {}
                }
                let (Some(left), Some(right)) = (&left.parameters, &right.parameters) else {
                    return None;
                };
                if left.len() != right.len() {
                    return Some(false);
                }
                for (left, right) in left.iter().zip(right) {
                    match self.types(left, right, true, depth + 1) {
                        Some(false) => return Some(false),
                        None => unknown = true,
                        _ => {}
                    }
                }
                (!unknown).then_some(true)
            }
            (TypeShapeKind::Unsupported, _) | (_, TypeShapeKind::Unsupported) => None,
            _ => Some(false),
        }
    }
}

/// Classify C categories into conservative pruning bits rather than Rust storage types.
fn category_mask(category: TypeCategory) -> u8 {
    match category {
        TypeCategory::Integer(_) | TypeCategory::Enum => INTEGER,
        TypeCategory::Floating => FLOAT,
        TypeCategory::Pointer | TypeCategory::Function => POINTER,
        TypeCategory::Record => RECORD,
        TypeCategory::Void | TypeCategory::Other => 16,
    }
}

/// Index promoted fields once for the entire batch; constraints perform only
/// lookups and visits to possible owners, rather than rescanning every record.
struct RecordIndex {
    /// Canonical records mapped to named and anonymously promoted field types.
    fields: BTreeMap<String, BTreeMap<String, TypeInfo>>,
    /// Reverse member-name index used to select possible owners without scanning every record.
    by_name: BTreeMap<String, Vec<String>>,
}
/// Build promoted-member indexes and perform bounded named or wildcard family selection.
impl RecordIndex {
    /// Select a named member in logarithmic lookup time, or traverse the full wildcard family.
    fn select<'a>(
        fields: &'a BTreeMap<String, TypeInfo>,
        name: Option<&str>,
    ) -> std::collections::btree_map::Range<'a, String, TypeInfo> {
        use std::ops::Bound::{Included, Unbounded};
        let bounds = name.map_or((Unbounded, Unbounded), |name| (Included(name), Included(name)));
        fields.range::<str, _>(bounds)
    }

    /// Index promoted fields once so subsequent demand queries reuse compiler-owned owner facts.
    fn new(declarations: &DeclarationCatalog) -> Self {
        /// Collect anonymous member promotions within the bounded record-depth limit.
        fn collect(
            declarations: &DeclarationCatalog,
            canonical: &str,
            depth: usize,
            fields: &mut BTreeMap<String, TypeInfo>,
        ) {
            if depth > 64 {
                return;
            }
            let Some(record) = declarations.records.get(canonical) else { return };
            for field in &record.fields {
                if let Some(name) = &field.name {
                    fields.insert(name.clone(), field.ty.clone());
                } else if field.is_anonymous {
                    collect(declarations, &field.ty.canonical_spelling, depth + 1, fields);
                }
            }
        }
        let mut result = Self { fields: BTreeMap::new(), by_name: BTreeMap::new() };
        for canonical in declarations.records.keys() {
            let mut fields = BTreeMap::new();
            collect(declarations, canonical, 0, &mut fields);
            for name in fields.keys() {
                result.by_name.entry(name.clone()).or_default().push(canonical.clone());
            }
            result.fields.insert(canonical.clone(), fields);
        }
        result
    }
}

/// Close semantic marker dependencies through pointer and array identities.
/// Record fields are followed only when a projection requested them.
pub(super) fn type_dependencies(
    declarations: &DeclarationCatalog,
    lowering: &Lowering<'_>,
    roots: &[TypeInfo],
    enums: &mut BTreeSet<String>,
    callbacks: &mut BTreeSet<String>,
) {
    let mut pending = roots.to_vec();
    let mut seen = BTreeSet::new();
    while let Some(ty) = pending.pop() {
        if !seen.insert(ty.canonical_spelling.clone()) {
            continue;
        }
        if ty.category == TypeCategory::Enum {
            enums.insert(super::types::enum_key(&ty));
        }
        match declarations.type_shapes.get(&ty.canonical_spelling).map(|shape| &shape.kind) {
            Some(TypeShapeKind::Array { element, .. }) => pending.push(element.clone()),
            Some(TypeShapeKind::Function { .. }) => {
                callbacks.insert(ty.canonical_spelling.clone());
            }
            _ if ty.category == TypeCategory::Pointer => {
                if let Ok(pointee) = lowering.pointer_pointee(&ty) {
                    if pointee.category == TypeCategory::Function {
                        callbacks.insert(ty.canonical_spelling.clone());
                    }
                    pending.push(pointee);
                }
            }
            _ => {}
        }
    }
}

/// Compute possible conditional or arithmetic result categories without assigning a concrete type.
fn common_category(left: u8, right: u8) -> u8 {
    if left & ARITHMETIC != 0 && right & ARITHMETIC != 0 {
        if left == FLOAT || right == FLOAT { FLOAT } else { INTEGER }
    } else if (left == POINTER && right & (POINTER | INTEGER) != 0)
        || (right == POINTER && left == INTEGER)
    {
        POINTER
    } else if left == right {
        left
    } else {
        0
    }
}
/// Model C operator category combinations used to propagate conservative operand restrictions.
fn binary_category(operator: BinaryOperator, left: u8, right: u8) -> u8 {
    use BinaryOperator::*;
    match operator {
        LogicalAnd | LogicalOr | Less | LessEqual | Greater | GreaterEqual | Equal | NotEqual => {
            if left & SCALAR != 0 && right & SCALAR != 0 {
                INTEGER
            } else {
                0
            }
        }
        Remainder | ShiftLeft | ShiftRight | BitAnd | BitXor | BitOr => {
            if left == INTEGER && right == INTEGER { INTEGER } else { 0 }
        }
        Multiply | Divide => {
            if left & ARITHMETIC != 0 && right & ARITHMETIC != 0 {
                common_category(left, right)
            } else {
                0
            }
        }
        Add | Subtract if left & ARITHMETIC != 0 && right & ARITHMETIC != 0 => {
            common_category(left, right)
        }
        Add if (left == POINTER && right == INTEGER) || (left == INTEGER && right == POINTER) => {
            POINTER
        }
        Subtract if left == POINTER && right == INTEGER => POINTER,
        Subtract if left == POINTER && right == POINTER => INTEGER,
        _ => 0,
    }
}

/// Regressions proving conservative type-family selection preserves qualifiers and unknown compiler facts.
#[cfg(test)]
mod tests {
    use super::*;
    use crate::{ArrayKind, IntegerKind, TypeShape};

    /// Check indexed named selection keeps wildcard behavior and returns no fabricated missing member.
    #[test]
    fn named_field_selection_preserves_wildcard_and_missing_families() {
        let fields = ["first", "selected", "third"]
            .map(|name| (name.into(), ty("int", TypeCategory::Integer(IntegerKind::Int))))
            .into();
        let names = |name| {
            RecordIndex::select(&fields, name).map(|(name, _)| name.as_str()).collect::<Vec<_>>()
        };
        assert_eq!(names(Some("selected")), ["selected"]);
        assert!(names(Some("missing")).is_empty());
        assert_eq!(names(None), ["first", "selected", "third"]);
    }

    /// Construct a minimal compiler type witness for conservative demand-planning fixtures.
    fn ty(name: &str, category: TypeCategory) -> TypeInfo {
        TypeInfo {
            spelling: name.into(),
            canonical_spelling: name.into(),
            category,
            size: None,
            alignment: None,
            is_const: false,
            is_volatile: false,
        }
    }

    /// Attach a compiler-owned structural shape to a synthetic type witness.
    fn shape(declarations: &mut DeclarationCatalog, ty: &TypeInfo, kind: TypeShapeKind) {
        declarations.type_shapes.insert(
            ty.canonical_spelling.clone(),
            TypeShape { ty: ty.clone(), is_restrict: false, kind },
        );
    }

    /// Provide integer rank and pointer facts needed by demand-planning fixture resolution.
    fn target() -> TargetFacts {
        TargetFacts {
            triple: "fixture".into(),
            pointer_bits: 64,
            function_pointer: crate::PointerLayout { size: 8, alignment: 8 },
            size_type: IntegerKind::UnsignedLong,
            ptrdiff_type: crate::IntegerKind::Long,
            preferred_alignments: Default::default(),
            arm_float_abi: None,
            ppc64_elf_abi: None,
            offsetof_supported: true,
            char_bits: 8,
            char_is_signed: true,
            ascii_execution_charset: true,
            byte_order: crate::ByteOrder::Little,
            c_standard: Some(201710),
            integers: [(
                IntegerKind::Int,
                crate::IntegerType { kind: IntegerKind::Int, bits: 32, signed: true, rank: 3 },
            )]
            .into(),
            floating_point: Default::default(),
        }
    }

    /// Check typedef-spelled pointers retain nominal pointee identity and nested qualifiers during inference.
    #[test]
    fn typedef_pointer_sources_preserve_identity_and_pointee_qualifiers() {
        let target = target();
        let leaf = ty("struct Leaf", TypeCategory::Record);
        let other = ty("struct Other", TypeCategory::Record);
        let pointer = ty("struct Leaf *", TypeCategory::Pointer);
        let other_pointer = ty("struct Other *", TypeCategory::Pointer);
        let mut frozen = leaf.clone();
        frozen.is_const = true;
        let mut observed = leaf.clone();
        observed.is_volatile = true;
        let mut void = ty("void", TypeCategory::Void);
        void.is_const = true;
        let const_void = ty("const void *", TypeCategory::Pointer);
        let nested = ty("struct Leaf **", TypeCategory::Pointer);
        let mut declarations = DeclarationCatalog::default();
        declarations.types.insert("Leaf".into(), leaf.clone());
        declarations.types.insert("FrozenLeaf".into(), frozen.clone());
        declarations.types.insert("LeafPointer".into(), pointer.clone());
        shape(&mut declarations, &leaf, TypeShapeKind::Record { identity: "leaf".into() });
        shape(&mut declarations, &other, TypeShapeKind::Record { identity: "other".into() });
        shape(&mut declarations, &pointer, TypeShapeKind::Pointer { pointee: leaf.clone() });
        shape(&mut declarations, &other_pointer, TypeShapeKind::Pointer { pointee: other });
        shape(&mut declarations, &const_void, TypeShapeKind::Pointer { pointee: void });
        shape(&mut declarations, &nested, TypeShapeKind::Pointer { pointee: pointer.clone() });
        let compatibility = Compatibility::new(&declarations, &target);
        let alias = ty("Leaf *", TypeCategory::Pointer);
        let const_alias = ty("const Leaf *", TypeCategory::Pointer);
        let frozen_alias = ty("FrozenLeaf *", TypeCategory::Pointer);
        assert_eq!(compatibility.element(&alias), Some(leaf));
        assert_eq!(compatibility.element(&const_alias), Some(frozen.clone()));
        assert_eq!(compatibility.element(&frozen_alias), Some(frozen));
        assert_eq!(
            compatibility.element(&ty("volatile Leaf *", TypeCategory::Pointer)),
            Some(observed)
        );
        assert_eq!(compatibility.implicit(&alias, &pointer, false), Some(true));
        assert_eq!(compatibility.implicit(&alias, &other_pointer, false), Some(false));
        assert_eq!(compatibility.implicit(&const_alias, &pointer, false), Some(false));
        assert_eq!(compatibility.implicit(&const_alias, &const_void, false), Some(true));
        assert_eq!(
            compatibility.implicit(&ty("LeafPointer *", TypeCategory::Pointer), &nested, false),
            Some(true)
        );
        assert_eq!(
            compatibility.implicit(&ty("const Leaf **", TypeCategory::Pointer), &nested, false),
            Some(false)
        );
        assert_eq!(compatibility.element(&ty("MissingAlias *", TypeCategory::Pointer)), None);
    }

    /// Prove cache keys distinguish inherited array qualifiers when canonical spellings match.
    #[test]
    fn acceptance_cache_preserves_inherited_array_qualifiers() {
        let target = target();
        let integer = ty("int", TypeCategory::Integer(IntegerKind::Int));
        let pointer = ty("int *", TypeCategory::Pointer);
        let array = ty("int[1]", TypeCategory::Other);
        let mut declarations = DeclarationCatalog::default();
        shape(&mut declarations, &integer, TypeShapeKind::Scalar);
        shape(&mut declarations, &pointer, TypeShapeKind::Pointer { pointee: integer.clone() });
        shape(
            &mut declarations,
            &array,
            TypeShapeKind::Array {
                element: integer,
                length: Some(1),
                array_kind: ArrayKind::Constant,
            },
        );
        let records = RecordIndex::new(&declarations);
        let compatibility = Compatibility::new(&declarations, &target);
        let constraints = Constraints {
            declarations: &declarations,
            representative: vec![0],
            requirements: vec![vec![Requirement::ImplicitTo {
                target: pointer,
                null_possible: false,
            }]],
            records: &records,
            compatibility: &compatibility,
            accepted: RefCell::new(BTreeMap::new()),
            sources: RefCell::new(BTreeMap::new()),
        };
        assert!(constraints.accepts(0, &array));
        let mut qualified = array.clone();
        qualified.is_const = true;
        assert!(!constraints.accepts(0, &qualified));
        qualified.is_const = false;
        qualified.is_volatile = true;
        assert!(!constraints.accepts(0, &qualified));
        assert!(constraints.accepts(0, &array));
    }

    /// Check missing qualifier compatibility facts retain candidates rather than prove restrict loss acceptable.
    #[test]
    fn first_pointee_restrict_loss_requires_compiler_proof() {
        let target = target();
        let integer = ty("int", TypeCategory::Integer(IntegerKind::Int));
        let pointer = ty("int *", TypeCategory::Pointer);
        let restricted = ty("int *restrict", TypeCategory::Pointer);
        let outer = ty("int *restrict *", TypeCategory::Pointer);
        let plain_outer = ty("int **", TypeCategory::Pointer);
        let void = ty("void", TypeCategory::Void);
        let void_pointer = ty("void *", TypeCategory::Pointer);
        let mut declarations = DeclarationCatalog::default();
        shape(&mut declarations, &integer, TypeShapeKind::Scalar);
        shape(&mut declarations, &pointer, TypeShapeKind::Pointer { pointee: integer.clone() });
        shape(&mut declarations, &restricted, TypeShapeKind::Pointer { pointee: integer });
        declarations.type_shapes.get_mut(&restricted.canonical_spelling).unwrap().is_restrict =
            true;
        shape(&mut declarations, &outer, TypeShapeKind::Pointer { pointee: restricted.clone() });
        shape(&mut declarations, &plain_outer, TypeShapeKind::Pointer { pointee: pointer });
        shape(&mut declarations, &void, TypeShapeKind::Scalar);
        shape(&mut declarations, &void_pointer, TypeShapeKind::Pointer { pointee: void });
        let compatibility = Compatibility::new(&declarations, &target);
        assert_eq!(compatibility.implicit(&outer, &plain_outer, false), Some(false));
        assert_eq!(compatibility.implicit(&outer, &void_pointer, false), Some(false));
        assert_eq!(compatibility.implicit(&plain_outer, &outer, false), Some(true));
        drop(compatibility);
        declarations.type_shapes.remove(&restricted.canonical_spelling);
        let compatibility = Compatibility::new(&declarations, &target);
        assert_ne!(compatibility.implicit(&outer, &plain_outer, false), Some(false));
        assert_ne!(compatibility.implicit(&outer, &void_pointer, false), Some(false));
    }

    /// Check incomplete call-result facts cannot justify excluding a callback from open call demand.
    #[test]
    fn missing_result_metadata_cannot_exclude_a_callback() {
        let target = target();
        let declarations = DeclarationCatalog::default();
        let records = RecordIndex::new(&declarations);
        let compatibility = Compatibility::new(&declarations, &target);
        let missing_pointer = ty("struct Missing *", TypeCategory::Pointer);
        let missing_record = ty("struct Missing", TypeCategory::Record);
        let unknown = ty("unknown", TypeCategory::Other);
        for requirement in [
            Requirement::Element { output: 1 },
            Requirement::Call { arity: 1, output: 1 },
            Requirement::Member { name: Some("field".into()), output: 1, indirect: true },
        ] {
            let constraints = Constraints {
                declarations: &declarations,
                representative: vec![0, 1],
                requirements: vec![vec![requirement], vec![Requirement::Kind(INTEGER)]],
                records: &records,
                compatibility: &compatibility,
                accepted: RefCell::new(BTreeMap::new()),
                sources: RefCell::new(BTreeMap::new()),
            };
            assert!(constraints.accepts(0, &missing_pointer));
            assert!(constraints.accepts(0, &unknown));
        }
        let constraints = Constraints {
            declarations: &declarations,
            representative: vec![0, 1],
            requirements: vec![
                vec![Requirement::Member {
                    name: Some("field".into()),
                    output: 1,
                    indirect: false,
                }],
                vec![Requirement::Kind(INTEGER)],
            ],
            records: &records,
            compatibility: &compatibility,
            accepted: RefCell::new(BTreeMap::new()),
            sources: RefCell::new(BTreeMap::new()),
        };
        assert!(constraints.accepts(0, &missing_record));
        assert!(constraints.accepts(0, &unknown));
        assert_eq!(
            compatibility.implicit(
                &unknown,
                &ty("int", TypeCategory::Integer(IntegerKind::Int)),
                false
            ),
            None,
            "an unfamiliar compiler category is not an incompatibility proof"
        );
    }

    /// Check finite subscript inference requires element facts and preserves both pointer-plus-integer orientations.
    #[test]
    fn finite_subscript_sources_require_pointee_facts_and_keep_both_orientations() {
        let target = target();
        let integer = ty("int", TypeCategory::Integer(IntegerKind::Int));
        let missing = ty("struct Missing *", TypeCategory::Pointer);
        let unknown = ty("unknown", TypeCategory::Other);
        let array = ty("int[1]", TypeCategory::Other);
        let mut declarations = DeclarationCatalog::default();
        shape(&mut declarations, &integer, TypeShapeKind::Scalar);
        shape(
            &mut declarations,
            &array,
            TypeShapeKind::Array {
                element: integer.clone(),
                length: Some(1),
                array_kind: ArrayKind::Constant,
            },
        );
        let records = RecordIndex::new(&declarations);
        let compatibility = Compatibility::new(&declarations, &target);
        let expression = crate::syntax::Expression {
            nodes: [
                ExpressionKind::Parameter { index: 0 },
                ExpressionKind::Parameter { index: 1 },
                ExpressionKind::Index { base: 0, index: 1 },
            ]
            .into_iter()
            .map(|kind| crate::syntax::ExpressionNode {
                kind,
                tokens: crate::syntax::TokenRange { start: 0, end: 0 },
            })
            .collect(),
            root: 2,
            statement_body: None,
        };
        let infer = |left: Option<TypeInfo>, right: Option<TypeInfo>| {
            let constraints = Constraints {
                declarations: &declarations,
                representative: vec![0, 1, 2],
                requirements: vec![Vec::new(); 3],
                records: &records,
                compatibility: &compatibility,
                accepted: RefCell::new(BTreeMap::new()),
                sources: RefCell::new(BTreeMap::new()),
            };
            constraints.sources(2, &expression, &[left, right, None], 0)
        };
        assert!(
            infer(Some(missing), Some(integer.clone())).is_none(),
            "a pointer without pointee metadata cannot prove an empty family"
        );
        assert!(
            infer(Some(unknown.clone()), None).is_none(),
            "unknown facts cannot close either subscript orientation"
        );
        assert_eq!(
            infer(Some(unknown), Some(array.clone())).unwrap().len(),
            1,
            "unknown facts must retain the possible integer orientation beside a known array"
        );
        assert!(
            infer(Some(integer.clone()), None).is_none(),
            "an integer and unknown pointer retain an open pointee family"
        );
        for (left, right) in [(Some(array.clone()), None), (None, Some(array.clone()))] {
            let sources =
                infer(left, right).expect("a known array closes both subscript orientations");
            assert_eq!(sources.len(), 1);
            assert_eq!(sources.values().next().unwrap(), &integer);
        }
        let mut qualified = array;
        qualified.is_const = true;
        let sources = infer(Some(qualified), Some(integer)).unwrap();
        assert!(
            sources.values().next().unwrap().is_const,
            "array element qualification remains part of its finite source identity"
        );
    }
}
