//LICENSE Portions Copyright 2026 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.

//! Copy declaration type edges while the translation unit owns their Clang handles.

use super::type_info;
use crate::{
    ArrayKind, DeclarationCatalog, DeclarationLinkage, FieldInfo, FunctionInfo, FunctionSignature,
    RecordInfo, RecordKind, TypeShape, TypeShapeKind,
};
use clang::{
    Entity, EntityKind, EntityVisitResult, Linkage, StorageClass, TranslationUnit, Type, TypeKind,
};
use std::collections::BTreeSet;

pub(super) fn collect(unit: &TranslationUnit<'_>, catalog: &mut DeclarationCatalog) {
    let mut collector = Collector { catalog, visited: BTreeSet::new(), pending: Vec::new() };
    unit.get_entity().visit_children(|entity, _| {
        match entity.get_kind() {
            EntityKind::TypedefDecl => {
                if let Some(ty) = entity.get_typedef_underlying_type() {
                    collector.collect_type(ty);
                }
            }
            EntityKind::StructDecl | EntityKind::UnionDecl | EntityKind::EnumDecl => {
                if let Some(ty) = entity.get_type() {
                    collector.collect_type(ty);
                }
                return EntityVisitResult::Recurse;
            }
            EntityKind::VarDecl | EntityKind::FunctionDecl => {
                if let Some(ty) = entity.get_type() {
                    collector.collect_type(ty);
                    if entity.get_kind() == EntityKind::FunctionDecl {
                        collector.collect_function(entity, ty);
                    }
                }
            }
            _ => {}
        }
        EntityVisitResult::Continue
    });
}

struct Collector<'a, 'tu> {
    catalog: &'a mut DeclarationCatalog,
    visited: BTreeSet<String>,
    pending: Vec<Type<'tu>>,
}

impl<'tu> Collector<'_, 'tu> {
    fn collect_type(&mut self, ty: Type<'tu>) {
        self.pending.push(ty);
        self.drain_types();
    }

    fn drain_types(&mut self) {
        // A worklist bounds stack use even for long chains of record/pointer types.
        while let Some(ty) = self.pending.pop() {
            self.collect_shape(ty);
        }
    }

    fn collect_shape(&mut self, ty: Type<'tu>) {
        let canonical = ty.get_canonical_type();
        let spelling = canonical.get_display_name();
        // Mark before traversing edges: records routinely contain pointers to themselves.
        if !self.visited.insert(spelling.clone()) {
            return;
        }
        let kind = match canonical.get_kind() {
            TypeKind::Pointer => match canonical.get_pointee_type() {
                Some(pointee) => {
                    self.pending.push(pointee);
                    TypeShapeKind::Pointer { pointee: type_info(pointee) }
                }
                None => TypeShapeKind::Unsupported,
            },
            array_kind @ (TypeKind::ConstantArray
            | TypeKind::IncompleteArray
            | TypeKind::VariableArray
            | TypeKind::DependentSizedArray) => match canonical.get_element_type() {
                Some(element) => {
                    self.pending.push(element);
                    TypeShapeKind::Array {
                        element: type_info(element),
                        length: canonical.get_size().map(|length| length as u64),
                        array_kind: match array_kind {
                            TypeKind::ConstantArray => ArrayKind::Constant,
                            TypeKind::IncompleteArray => ArrayKind::Incomplete,
                            TypeKind::VariableArray => ArrayKind::Variable,
                            TypeKind::DependentSizedArray => ArrayKind::Dependent,
                            _ => unreachable!("array kinds are matched above"),
                        },
                    }
                }
                None => TypeShapeKind::Unsupported,
            },
            TypeKind::FunctionPrototype | TypeKind::FunctionNoPrototype => {
                match self.collect_signature(canonical) {
                    Some(signature) => TypeShapeKind::Function { signature },
                    None => TypeShapeKind::Unsupported,
                }
            }
            TypeKind::Record => self.collect_record(canonical, &spelling),
            TypeKind::Enum => {
                let underlying = canonical
                    .get_declaration()
                    .and_then(|declaration| declaration.get_enum_underlying_type());
                if let Some(underlying) = underlying {
                    self.pending.push(underlying);
                }
                TypeShapeKind::Enum { underlying: underlying.map(type_info) }
            }
            TypeKind::Void
            | TypeKind::Bool
            | TypeKind::CharS
            | TypeKind::CharU
            | TypeKind::SChar
            | TypeKind::UChar
            | TypeKind::Short
            | TypeKind::UShort
            | TypeKind::Int
            | TypeKind::UInt
            | TypeKind::Long
            | TypeKind::ULong
            | TypeKind::LongLong
            | TypeKind::ULongLong
            | TypeKind::Int128
            | TypeKind::UInt128
            | TypeKind::Float
            | TypeKind::Double
            | TypeKind::LongDouble => TypeShapeKind::Scalar,
            _ => TypeShapeKind::Unsupported,
        };
        self.catalog.type_shapes.insert(
            spelling,
            TypeShape {
                ty: type_info(canonical),
                is_restrict: canonical.is_restrict_qualified(),
                kind,
            },
        );
    }

    fn collect_signature(&mut self, ty: Type<'tu>) -> Option<FunctionSignature> {
        let result = ty.get_result_type()?;
        self.pending.push(result);
        let parameters = if ty.get_canonical_type().get_kind() == TypeKind::FunctionPrototype {
            ty.get_argument_types()
        } else {
            None
        };
        if let Some(parameters) = &parameters {
            for &parameter in parameters {
                self.pending.push(parameter);
            }
        }
        Some(FunctionSignature {
            result: type_info(result),
            parameters: parameters
                .map(|parameters| parameters.into_iter().map(type_info).collect()),
            variadic: ty.is_variadic(),
            calling_convention: ty
                .get_calling_convention()
                .map(|convention| format!("{convention:?}")),
        })
    }

    fn collect_record(&mut self, ty: Type<'tu>, spelling: &str) -> TypeShapeKind {
        let Some(declaration) = ty.get_declaration() else { return TypeShapeKind::Unsupported };
        let declaration = declaration.get_definition().unwrap_or(declaration);
        let kind = match declaration.get_kind() {
            EntityKind::StructDecl => RecordKind::Struct,
            EntityKind::UnionDecl => RecordKind::Union,
            _ => return TypeShapeKind::Unsupported,
        };
        let identity =
            declaration.get_usr().map(|usr| usr.0).unwrap_or_else(|| spelling.to_owned());
        let mut fields = Vec::new();
        for field in declaration.get_children() {
            if field.get_kind() == EntityKind::FieldDecl {
                let Some(field_type) = field.get_type() else { continue };
                self.pending.push(field_type);
                fields.push(FieldInfo {
                    name: field.get_name().filter(|name| !name.is_empty()),
                    ty: type_info(field_type),
                    offset_bits: field.get_offset_of_field().ok().map(|offset| offset as u64),
                    bit_width: field
                        .get_bit_field_width()
                        .and_then(|width| u32::try_from(width).ok()),
                    is_anonymous: field.is_anonymous(),
                });
            } else if matches!(field.get_kind(), EntityKind::StructDecl | EntityKind::UnionDecl)
                && field.is_anonymous()
            {
                // libclang hides the implicit FieldDecl of a C11 anonymous member.
                // Recover its placement from compiler offsets of promoted member names.
                let Some(field_type) = field.get_type() else { continue };
                self.pending.push(field_type);
                fields.push(FieldInfo {
                    name: None,
                    ty: type_info(field_type),
                    offset_bits: anonymous_offset(ty, field_type),
                    bit_width: None,
                    is_anonymous: true,
                });
            }
        }
        self.catalog.records.insert(
            spelling.to_owned(),
            RecordInfo {
                identity: identity.clone(),
                name: declaration.get_name().filter(|name| !name.is_empty()),
                kind,
                fields,
                size: ty.get_sizeof().ok().map(|size| size as u64),
                alignment: ty.get_alignof().ok().map(|alignment| alignment as u64),
                is_anonymous: declaration.is_anonymous(),
            },
        );
        TypeShapeKind::Record { identity }
    }

    fn collect_function(&mut self, entity: Entity<'tu>, ty: Type<'tu>) {
        let (Some(name), Some(signature)) = (entity.get_name(), self.collect_signature(ty)) else {
            return;
        };
        let info = FunctionInfo {
            signature,
            linkage: entity.get_linkage().map(|linkage| match linkage {
                Linkage::Automatic => DeclarationLinkage::Automatic,
                Linkage::Internal => DeclarationLinkage::Internal,
                Linkage::External => DeclarationLinkage::External,
                Linkage::UniqueExternal => DeclarationLinkage::UniqueExternal,
            }),
            is_static: entity.get_storage_class() == Some(StorageClass::Static),
            is_inline: entity.is_inline_function(),
            definition_available: entity.is_definition() || entity.get_definition().is_some(),
        };
        self.catalog.function_signatures.insert(name, info);
        self.drain_types();
    }
}

fn anonymous_offset(owner: Type<'_>, member: Type<'_>) -> Option<u64> {
    member.get_fields()?.into_iter().find_map(|field| {
        let name = field.get_name().filter(|name| !name.is_empty())?;
        let outer = owner.get_offsetof(&name).ok()?;
        let inner = member.get_offsetof(&name).ok()?;
        outer.checked_sub(inner).map(|offset| offset as u64)
    })
}
