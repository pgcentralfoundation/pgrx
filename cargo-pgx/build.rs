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
