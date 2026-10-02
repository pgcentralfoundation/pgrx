//LICENSE Portions Copyright 2026 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.

//! Function addresses come from the original C declaration, including statics.

use super::types::Lowering;
use crate::{
    BindingCatalog, DeclarationCatalog, DeclarationLinkage, FunctionAddressBinding, TargetFacts,
};
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Write;

pub(super) struct AddressAdapters {
    pub rust: String,
    pub c_source: String,
    pub bindings: BTreeMap<String, FunctionAddressBinding>,
    pub unsupported: BTreeMap<String, String>,
}

pub(super) fn generate(
    declarations: &DeclarationCatalog,
    bindings: &BindingCatalog,
    names: &BTreeSet<String>,
    target: &TargetFacts,
    profile_identity: &str,
) -> Result<AddressAdapters, String> {
    let lowering = Lowering::new(declarations, bindings, target);
    let mut output = AddressAdapters {
        rust: String::new(),
        c_source: String::new(),
        bindings: BTreeMap::new(),
        unsupported: BTreeMap::new(),
    };
    for name in names {
        let result = (|| {
            let function = declarations
                .function_signatures
                .get(name)
                .ok_or("missing compiler function declaration")?;
            if function.linkage == Some(DeclarationLinkage::Internal)
                && !function.definition_available
            {
                return Err("internal function address has no compiler-proven definition".into());
            }
            if !matches!(
                function.linkage,
                Some(
                    DeclarationLinkage::Internal
                        | DeclarationLinkage::External
                        | DeclarationLinkage::UniqueExternal
                )
            ) {
                return Err("function address has no supported linkage".into());
            }
            let mut chars = name.chars();
            if !chars.next().is_some_and(|ch| ch == '_' || ch.is_ascii_alphabetic())
                || !chars.all(|ch| ch == '_' || ch.is_ascii_alphanumeric())
            {
                return Err("function address has no single C identifier".into());
            }
            let ty = declarations.functions.get(name).ok_or("missing compiler function type")?;
            let signature = bindings
                .callback_capabilities
                .get(&ty.canonical_spelling)
                .ok_or("function address has no verified callback ABI")?;
            let storage = lowering.storage_type(&signature.storage, 0)?.replace("$crate", "crate");
            let marker = signature.marker.replace("$crate", "crate");
            let mut hash = Sha256::new();
            hash.update(profile_identity);
            hash.update(name);
            hash.update(serde_json::to_vec(function).map_err(|error| error.to_string())?);
            hash.update(serde_json::to_vec(&signature.storage).map_err(|error| error.to_string())?);
            let suffix =
                hash.finalize()[..16].iter().map(|byte| format!("{byte:02x}")).collect::<String>();
            let symbol = format!("__pgrx_function_address_{suffix}");
            let raw = format!("raw_address_{suffix}");
            let getter = format!("Address_{suffix}");
            // Parentheses keep a same-named function-like macro from expanding.
            // This also returns the original static function, rather than its
            // generated callable thunk or pgrx's Rust guard wrapper.
            let c = format!(
                "typedef __typeof__(&({name})) {symbol}_type;\n_Static_assert(sizeof({symbol}_type) == {}, \"function pointer size\");\n_Static_assert(_Alignof({symbol}_type) == {}, \"function pointer alignment\");\n{symbol}_type {symbol}(void) {{ return &({name}); }}\n",
                target.function_pointer.size, target.function_pointer.alignment
            );
            let mut rust = String::new();
            writeln!(rust, "unsafe extern \"C\" {{ #[link_name = {symbol:?}] fn {raw}() -> {storage}; }}\n#[inline]\npub fn {getter}() -> crate::__pgrx_c_macros::expression::FunctionValue<{marker}> {{\n// SAFETY: This generated C getter only takes a function address. It accesses no object, makes no callback, and cannot raise PostgreSQL ERROR. Its exact nullable function-pointer ABI was checked against the compiler declaration and binding storage.\ncrate::__pgrx_c_macros::expression::FunctionValue::new(unsafe {{ {raw}() }})\n}}").expect("String output");
            Ok::<_, String>((
                rust,
                c,
                FunctionAddressBinding {
                    marker: signature.marker.clone(),
                    path: vec!["__pgrx_c_generated".into(), getter],
                },
            ))
        })();
        match result {
            Ok((rust, c, binding)) => {
                output.rust.push_str(&rust);
                output.c_source.push_str(&c);
                output.bindings.insert(name.clone(), binding);
            }
            Err(reason) => {
                output.unsupported.insert(name.clone(), reason);
            }
        }
        if output.rust.len() + output.c_source.len() > 16 * 1024 * 1024 {
            return Err("function address adapters exceed the 16 MiB source budget".into());
        }
    }
    Ok(output)
}
