//LICENSE Portions Copyright 2019-2021 ZomboDB, LLC.
//LICENSE
//LICENSE Portions Copyright 2021-2023 Technology Concepts & Design, Inc.
//LICENSE
//LICENSE Portions Copyright 2023-2023 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE All rights reserved.
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.
fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    if let Some(minor_version) = rust_minor_version() {
        println!("cargo:rustc-env=MINOR_RUST_VERSION={minor_version}");
    }
}

fn rust_minor_version() -> Option<u32> {
    let rustc = std::env::var_os("RUSTC").unwrap_or_else(|| "rustc".into());
    let output = std::process::Command::new(rustc).arg("--version").output().ok()?;
    let version = std::str::from_utf8(&output.stdout).ok()?;
    let mut iter = version.split('.');
    if iter.next() != Some("rustc 1") {
        None
    } else {
        iter.next()?.parse().ok()
    }
}
