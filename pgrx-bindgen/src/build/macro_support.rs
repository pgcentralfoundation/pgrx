//LICENSE Portions Copyright 2026 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.

//! Build the native primitives required by generated Rust macro adapters.
//!
//! The frontend has already selected and verified the C compiler invocation.
//! This module reuses that exact profile to compile generated access helpers
//! against the original header, archives the fresh object, and tells Cargo how to
//! link it. Native helpers cover operations whose C layout or calling convention
//! cannot be inferred from Rust storage alone. When the C shim is enabled, both
//! kinds of adapters share one translation unit: PostgreSQL implementation
//! headers can define external functions, and including them in two linked
//! objects would give those functions duplicate definitions.

use eyre::{WrapErr, eyre};
use pgrx_c_macros::CompilationProfile;
use std::path::Path;

/// Select the native artifact for the active Cargo feature while still allowing
/// release generation to emit Rust bindings and macros for every supported version.
#[derive(Clone, Copy)]
pub(super) struct NativeBuild<'a> {
    /// Directory shared with bindgen's version-specific static function wrappers.
    pub(super) out_dir: &'a Path,
    /// Whether this PostgreSQL version is the one selected for the Rust build.
    pub(super) active: bool,
    /// Original C shim source to compile alongside native macro helpers when enabled.
    pub(super) cshim: Option<&'a Path>,
}

/// Compile and archive generated C access primitives with the inspected invocation profile,
/// without publishing Cargo linkage until all macro proofs succeed. The result says
/// whether this artifact also contains the C shim, so the caller can avoid
/// compiling a second object containing the same PostgreSQL header definitions.
pub(super) fn compile_macro_support(
    major: u16,
    profile: &CompilationProfile,
    source: &str,
    build: &NativeBuild<'_>,
) -> eyre::Result<bool> {
    let out_dir = build.out_dir;
    let stem = format!("pgrx_c_macros_pg{major}");
    let c_path = out_dir.join(format!("{stem}.c"));
    let msvc = profile.target.uses_msvc_abi();
    let object = out_dir.join(format!("{stem}.{}", if msvc { "obj" } else { "o" }));
    let archive = out_dir.join(if msvc { format!("{stem}.lib") } else { format!("lib{stem}.a") });
    let wrapper = out_dir.join(format!("{}.c", super::cshim_static_wrapper_name(major)));
    let translation_unit = native_source(
        wrapper.is_file().then_some(wrapper.as_path()),
        &profile.header,
        source,
        build.cshim,
    )?;
    super::write_content_stable(&c_path, translation_unit.as_bytes())?;
    pgrx_c_macros::compile_native_support(profile, &c_path, &object, &archive)
        .wrap_err("could not compile and archive generated C access support")?;
    Ok(build.cshim.is_some())
}

/// Publish the previously verified native archive only once the Rust macro tree
/// and audit report are ready, so a refused optional build cannot leak linkage.
pub(super) fn link_macro_support(major: u16, out_dir: &Path) {
    println!("cargo:rustc-link-search=native={}", out_dir.display());
    println!("cargo:rustc-link-lib=static=pgrx_c_macros_pg{major}");
}

/// Include the inspected header exactly once, either directly or through
/// bindgen's static wrapper included by the C shim. Some implementation headers
/// intentionally lack include guards, so including the header a second time is
/// invalid even within a single translation unit.
fn native_source(
    wrapper: Option<&Path>,
    header: &Path,
    source: &str,
    cshim: Option<&Path>,
) -> eyre::Result<String> {
    let prefix = if let Some(cshim) = cshim {
        let wrapper = wrapper.ok_or_else(|| eyre!("C shim needs its generated static wrapper"))?;
        format!(
            "#define PGRX_CSHIM_STATIC \"{}\"\n#include \"{}\"\n",
            native_include_path(wrapper)?,
            native_include_path(cshim)?,
        )
    } else if let Some(wrapper) = wrapper {
        // The generated wrapper includes the original header. Its inline
        // bindings belong to the ordinary pg-sys API, independently of cshim.
        format!("#include \"{}\"\n", native_include_path(wrapper)?)
    } else {
        format!("#include \"{}\"\n", native_include_path(header)?)
    };
    Ok(format!("{prefix}{source}"))
}

/// Use the frontend's checked header-name spelling so inspection and native
/// compilation select the same file on Unix, Windows drive paths, and UNC paths.
fn native_include_path(path: &Path) -> eyre::Result<String> {
    Ok(pgrx_c_macros::c_header_path(path)?)
}

/// The C shim owns the header include in a combined artifact. These tests
/// keep that ownership explicit, including version selection and invalid
/// include paths, before native compilation can change any output files.
#[cfg(test)]
mod tests {

    use super::native_source;
    #[cfg(unix)]
    use super::{NativeBuild, compile_macro_support};
    use std::path::Path;
    #[cfg(unix)]
    use std::path::PathBuf;

    /// Own a uniquely reserved native fixture directory, keeping test objects
    /// and executables away from generated binding outputs.
    #[cfg(unix)]
    struct Directory(
        /// Temporary tree containing only this test's sources and link artifacts.
        PathBuf,
    );

    /// Reserve and clean up only files owned by this native compilation test.
    #[cfg(unix)]
    impl Directory {
        /// Reserve a fresh temporary path without reusing another process's
        /// headers or previously compiled artifacts.
        fn new() -> Self {
            for attempt in 0..64 {
                let path = std::env::temp_dir()
                    .join(format!("pgrx-native-cshim-{}-{attempt}", std::process::id()));
                match std::fs::create_dir(&path) {
                    Ok(()) => return Self(path),
                    Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => continue,
                    Err(error) => panic!("could not reserve native fixture: {error}"),
                }
            }
            panic!("could not reserve a unique native fixture");
        }
    }

    /// Remove the native fixture after assertions finish or unwind.
    #[cfg(unix)]
    impl Drop for Directory {
        /// Delete only the temporary tree reserved by this fixture.
        fn drop(&mut self) {
            let _ = std::fs::remove_dir_all(&self.0);
        }
    }

    /// Prove the combined source selects the matching static wrapper and omits
    /// the second header include that would redefine unguarded C implementation code.
    #[test]
    fn combined_source_includes_header_only_through_the_versioned_cshim() {
        let source = native_source(
            Some(Path::new("/output/pgrx-cshim-static-pg15.c")),
            Path::new("/headers/pg15.h"),
            "int macro_helper(void) { return header_function(); }\n",
            Some(Path::new("/source/pgrx-cshim.c")),
        )
        .unwrap();
        assert_eq!(
            source,
            "#define PGRX_CSHIM_STATIC \"/output/pgrx-cshim-static-pg15.c\"\n#include \"/source/pgrx-cshim.c\"\nint macro_helper(void) { return header_function(); }\n"
        );
    }

    /// Prove builds without the C shim still compile helpers against the
    /// original inspected header rather than relying on an absent static wrapper.
    #[test]
    fn native_source_without_cshim_includes_the_inspected_header() {
        assert_eq!(
            native_source(None, Path::new("/headers/pg18.h"), "int helper;\n", None).unwrap(),
            "#include \"/headers/pg18.h\"\nint helper;\n"
        );
    }

    /// Reject include paths that could terminate or alter the generated C
    /// directive before either compiler output is published.
    #[test]
    fn invalid_include_paths_cannot_change_the_translation_unit() {
        for invalid in ["header\n.h", "header\r.h", "header\".h", "header\0.h"] {
            assert!(native_source(None, Path::new(invalid), "", None).is_err());
            assert!(
                native_source(
                    Some(Path::new("wrapper.c")),
                    Path::new("valid.h"),
                    "",
                    Some(Path::new(invalid))
                )
                .is_err()
            );
            assert!(
                native_source(
                    Some(Path::new(invalid)),
                    Path::new("valid.h"),
                    "",
                    Some(Path::new("shim.c"))
                )
                .is_err()
            );
        }
    }

    /// Preserve Windows directory separators as C include path separators,
    /// including extended drive and UNC paths, without treating them as escapes.
    #[test]
    #[cfg(windows)]
    fn windows_include_paths_preserve_filesystem_identity() {
        for (input, expected) in [
            (r"C:\headers\pg18.h", "C:/headers/pg18.h"),
            (r"\\?\C:\headers\pg18.h", "C:/headers/pg18.h"),
            (r"\\?\UNC\server\headers\pg18.h", "//server/headers/pg18.h"),
        ] {
            assert_eq!(super::native_include_path(Path::new(input)).unwrap(), expected);
        }
    }

    /// Link native macro helpers and C shim entry points together despite an
    /// unguarded header defining external functions and mutable globals, then
    /// prove both adapter families retain the same function and global identities.
    #[test]
    #[cfg(unix)]
    fn combined_native_and_cshim_share_header_functions_and_globals() {
        let _lock =
            super::super::MACRO_SCANNER.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let directory = Directory::new();
        let header = directory.0.join("unguarded.h");
        let cshim = directory.0.join("pgrx-cshim.c");
        let wrapper = directory.0.join("pgrx-cshim-static-pg15.c");
        std::fs::write(
            &header,
            "int header_global = 7;\nint header_external(void) { return header_global; }\nstatic inline int inline_read(void) { return header_external(); }\n",
        )
        .unwrap();
        std::fs::write(
            &wrapper,
            format!(
                "#include \"{}\"\nint cshim_read(void) {{ return inline_read(); }}\n",
                header.display()
            ),
        )
        .unwrap();
        std::fs::write(
            &cshim,
            "#include PGRX_CSHIM_STATIC\nint cshim_mutate(void) { return ++header_global; }\n",
        )
        .unwrap();
        let scanner = pgrx_c_macros::MacroScanner::new().unwrap();
        let frontend = pgrx_c_macros::inspect(&scanner, &header, &[], None).unwrap();
        for cshim in [None, Some(cshim.as_path())] {
            assert!(
            compile_macro_support(
                15,
                frontend.profile(),
                "int macro_read(void) { return inline_read(); }\nint *macro_global(void) { return &header_global; }\nint (*macro_function(void))(void) { return header_external; }\n",
                &NativeBuild { out_dir: &directory.0, active: true, cshim },
            )
            .unwrap() == cshim.is_some()
        );
            let main = directory.0.join("main.c");
            let shim_declarations = if cshim.is_some() {
                "int cshim_mutate(void);"
            } else {
                "int cshim_mutate(void) { return ++header_global; }"
            };
            std::fs::write(
                &main,
                format!(
                    "extern int header_global;\n{shim_declarations}\n{}",
                    r#"
extern int header_global;
int header_external(void);
int cshim_read(void);
int cshim_mutate(void);
int macro_read(void);
int *macro_global(void);
int (*macro_function(void))(void);
int main(void) {
    if (cshim_read() != 7 || macro_read() != 7) return 1;
    if (macro_global() != &header_global) return 2;
    if (macro_function() != header_external) return 3;
    if (cshim_mutate() != 8 || macro_read() != 8) return 4;
    return 0;
}
"#
                ),
            )
            .unwrap();
            let executable = directory.0.join("native-test");
            // The host linker locates its system runtime independently of the
            // inspected Clang used to generate the portable native archive.
            let output = std::process::Command::new("cc")
                .arg(&main)
                .arg(directory.0.join("libpgrx_c_macros_pg15.a"))
                .arg("-o")
                .arg(&executable)
                .output()
                .unwrap();
            assert!(output.status.success(), "{}", String::from_utf8_lossy(&output.stderr));
            assert!(std::process::Command::new(&executable).status().unwrap().success());
        }
    }
}
