//LICENSE Portions Copyright 2026 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.

//! Establish inline definitions that declaration-only parsing intentionally hides.

//! Declaration-only parsing omits function bodies to keep inspection affordable. Static inline
//! adapters still need proof that an original definition exists, so this module performs one
//! bounded reparse with bodies enabled. Only definitions whose linkage, storage class, and
//! complete signature match the first catalog are admitted to native support generation.

use super::{FrontendError, type_info};
use crate::{DeclarationLinkage, FrontendOutput, MacroScanner};
use clang::{EntityKind, EntityVisitResult, Index, Linkage, StorageClass, TypeKind};
use std::collections::BTreeSet;

/// Bound declaration or expression candidates before constructing a compiler proof batch.
const MAX_CANDIDATES: usize = 16_384;

/// Reparse the same input once with function bodies enabled. Definition identity
/// and the full prototype must agree with the declaration-only catalog; seeing
/// braces in a token range is not sufficient evidence of a function definition.
pub(super) fn prove(
    scanner: &MacroScanner,
    frontend: &FrontendOutput,
) -> Result<BTreeSet<String>, FrontendError> {
    let candidates = frontend
        .declarations()
        .function_signatures
        .iter()
        .filter(|(_, info)| {
            info.is_static
                && info.is_inline
                && info.linkage == Some(DeclarationLinkage::Internal)
                && !info.definition_available
        })
        .map(|(name, _)| name.as_str())
        .collect::<BTreeSet<_>>();
    if candidates.is_empty() {
        return Ok(BTreeSet::new());
    }
    if candidates.len() > MAX_CANDIDATES {
        return Err(FrontendError::Output(
            "inline definitions exceed the 16384-function budget".into(),
        ));
    }
    let profile = frontend.profile();
    let index = Index::new(&scanner.clang, false, false);
    let arguments = ["-x".to_owned(), "c".to_owned()]
        .into_iter()
        .chain(profile.arguments.iter().cloned())
        .collect::<Vec<_>>();
    let mut parser = index.parser(&profile.header);
    parser.arguments(&arguments).skip_function_bodies(false);
    let unit = parser
        .parse()
        .map_err(|source| crate::Error::Parse { header: profile.header.clone(), source })?;
    let errors = unit
        .get_diagnostics()
        .into_iter()
        .filter(|diagnostic| {
            matches!(
                diagnostic.get_severity(),
                clang::diagnostic::Severity::Error | clang::diagnostic::Severity::Fatal
            )
        })
        .map(|diagnostic| diagnostic.to_string())
        .collect::<Vec<_>>();
    if !errors.is_empty() {
        return Err(FrontendError::Environment(format!(
            "full inline definition parsing failed: {}",
            errors.join("\n")
        )));
    }
    let mut definitions = BTreeSet::new();
    unit.get_entity().visit_children(|entity, _| {
        if entity.get_kind() != EntityKind::FunctionDecl {
            return EntityVisitResult::Continue;
        }
        let Some(name) = entity.get_name().filter(|name| candidates.contains(name.as_str())) else {
            return EntityVisitResult::Continue;
        };
        if !entity.is_inline_function()
            || entity.get_storage_class() != Some(StorageClass::Static)
            || entity.get_linkage() != Some(Linkage::Internal)
            || !(entity.is_definition() || entity.get_definition().is_some())
        {
            return EntityVisitResult::Continue;
        }
        let Some(ty) = entity.get_type() else { return EntityVisitResult::Continue };
        let expected = &frontend.declarations().function_signatures[&name].signature;
        let parameters = if ty.get_canonical_type().get_kind() == TypeKind::FunctionPrototype {
            ty.get_argument_types()
                .map(|parameters| parameters.into_iter().map(type_info).collect::<Vec<_>>())
        } else {
            None
        };
        if ty.get_result_type().map(type_info).as_ref() == Some(&expected.result)
            && parameters == expected.parameters
            && ty.is_variadic() == expected.variadic
            && ty.get_calling_convention().map(|convention| format!("{convention:?}"))
                == expected.calling_convention
        {
            definitions.insert(name);
        }
        EntityVisitResult::Continue
    });
    Ok(definitions)
}
