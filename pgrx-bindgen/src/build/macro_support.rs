//LICENSE Portions Copyright 2026 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.

//! Compile native access primitives with the already inspected C invocation profile.

use eyre::{WrapErr, eyre};
use pgrx_c_macros::CompilationProfile;
use std::path::Path;

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
