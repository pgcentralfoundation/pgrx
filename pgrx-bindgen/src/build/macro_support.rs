//LICENSE Portions Copyright 2026 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.

//! Build the native primitives required by generated Rust macro adapters.
//!
//! The frontend has already selected and verified the C compiler invocation.
//! This module reuses that exact profile to compile generated access helpers
//! against the original header, archives the fresh object, and tells Cargo how to
//! link it. Native helpers cover operations whose C layout or calling convention
//! cannot be inferred from Rust storage alone.

/// Preserve contextual generation and filesystem failures through the binding-build error
/// contract.
use eyre::{WrapErr, eyre};
/// Use the emitter catalog and result types that connect C inspection to generated binding
/// publication.
use pgrx_c_macros::CompilationProfile;
/// Keep provenance roots and generated relative paths explicit across module rendering and
/// publication.
use std::path::Path;

/// Compile and archive generated C access primitives with the inspected invocation profile,
/// then emit Cargo linkage for the selected PostgreSQL version.
pub(super) fn compile_macro_support(
    major: u16,
    profile: &CompilationProfile,
    source: &str,
    out_dir: &Path,
) -> eyre::Result<()> {
    let header =
        profile.header.to_str().ok_or_else(|| eyre!("C access support header is not UTF-8"))?;
    if header.contains(['\n', '\r', '"', '\\']) {
        return Err(eyre!("C access support header cannot be represented in an include directive"));
    }
    let stem = format!("pgrx_c_macros_pg{major}");
    let c_path = out_dir.join(format!("{stem}.c"));
    let object = out_dir.join(format!("{stem}.o"));
    let archive = out_dir.join(format!("lib{stem}.a"));
    super::write_content_stable(&c_path, format!("#include \"{header}\"\n{source}").as_bytes())?;
    pgrx_c_macros::compile_native_support(profile, &c_path, &object, &archive)
        .wrap_err("could not compile and archive generated C access support")?;
    println!("cargo:rustc-link-search=native={}", out_dir.display());
    println!("cargo:rustc-link-lib=static={stem}");
    Ok(())
}
