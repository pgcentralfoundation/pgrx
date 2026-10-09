//LICENSE Portions Copyright 2026 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.

//! Establish inline definitions that declaration-only parsing intentionally hides.

//! Declaration-only parsing omits function bodies to keep inspection affordable. Static inline
//! adapters still need proof that an original definition exists, so this module performs one
//! bounded reparse with bodies enabled. Only definitions whose linkage, storage class, and
//! complete signature match the first catalog are admitted to native support generation.

use super::{FrontendError, type_info};
use crate::{
    DeclarationLinkage, FrontendOutput, InlineFunctionDefinition, MacroScanner, SourceSpan,
};
use clang::{Entity, EntityKind, EntityVisitResult, Index, Linkage, StorageClass, TypeKind};
use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

/// Bound declaration or expression candidates before constructing a compiler proof batch.
const MAX_CANDIDATES: usize = 16_384;
/// Keep original definition documentation bounded independently of the Clang translation unit.
const MAX_DEFINITION_BYTES: usize = 64 * 1024;
/// Bound all retained inline definition text; unavailable text becomes an explicit emission refusal.
const MAX_SOURCE_BYTES: usize = 16 * 1024 * 1024;

/// Reparse the same input once with function bodies enabled. Definition identity
/// and the full prototype must agree with the declaration-only catalog; seeing
/// braces in a token range is not sufficient evidence of a function definition.
pub(super) fn prove(
    scanner: &MacroScanner,
    frontend: &FrontendOutput,
) -> Result<BTreeMap<String, Option<InlineFunctionDefinition>>, FrontendError> {
    let candidates = frontend
        .declarations()
        .function_signatures
        .iter()
        .filter(|(_, info)| {
            info.is_static && info.is_inline && info.linkage == Some(DeclarationLinkage::Internal)
        })
        .map(|(name, _)| name.as_str())
        .collect::<BTreeSet<_>>();
    if candidates.is_empty() {
        return Ok(BTreeMap::new());
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
    let mut definitions = BTreeMap::new();
    let mut files = BTreeMap::new();
    let mut remaining = MAX_SOURCE_BYTES;
    let directory = profile.header.parent().unwrap_or_else(|| Path::new("."));
    unit.get_entity().visit_children(|entity, _| {
        if entity.get_kind() != EntityKind::FunctionDecl {
            return EntityVisitResult::Continue;
        }
        let Some(name) = entity.get_name().filter(|name| candidates.contains(name.as_str())) else {
            return EntityVisitResult::Continue;
        };
        // An entry exists only after a matching complete definition proof. Its
        // forward declarations share that same definition: charging/copying it
        // again could exhaust the budget and replace valid retained text with None.
        if definitions.contains_key(&name) {
            return EntityVisitResult::Continue;
        }
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
            let definition = entity.get_definition().unwrap_or(entity);
            definitions.insert(name, source(definition, directory, &mut files, &mut remaining));
        }
        EntityVisitResult::Continue
    });
    Ok(definitions)
}

/// Copy physical original source once per header and retain only bounded definition text.
fn source(
    definition: Entity<'_>,
    directory: &Path,
    files: &mut BTreeMap<PathBuf, Option<String>>,
    remaining: &mut usize,
) -> Option<InlineFunctionDefinition> {
    let range = definition.get_range()?;
    let provenance = SourceSpan::from_range(range, directory).ok()??;
    let start = range.get_start().get_spelling_location();
    let end = range.get_end().get_spelling_location();
    let file = start.file?;
    let size = usize::try_from(end.offset.checked_sub(start.offset)?).ok()?;
    let source = if size <= MAX_DEFINITION_BYTES && size <= *remaining {
        let contents = files.entry(provenance.file.clone()).or_insert_with(|| file.get_contents());
        let text = contents.as_ref().and_then(|contents| {
            contents.get(usize::try_from(start.offset).ok()?..usize::try_from(end.offset).ok()?)
        });
        text.map(|text| {
            *remaining -= size;
            text.to_owned()
        })
    } else {
        None
    };
    Some(InlineFunctionDefinition {
        provenance,
        parameters: definition
            .get_arguments()
            .unwrap_or_default()
            .into_iter()
            .map(|parameter| parameter.get_name().filter(|name| !name.is_empty()))
            .collect(),
        source,
    })
}
