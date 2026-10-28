//LICENSE Portions Copyright 2026 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.

//! Verify that PostgreSQL server headers describe the inspected C target.
//!
//! Freestanding fixtures record `pg_config.h` facts for several machines and include no system
//! headers, so each runs under an explicit `--target` on any host with libclang and a matching
//! Clang. The machine check and the layout witness each refuse configurations the other
//! accepts.

use pgrx_c_macros::{
    ConfigurationError, LayoutDisagreement, MacroScanner, PostgresError, inspect,
    verify_postgres_configuration,
};
use std::path::{Path, PathBuf};
use std::sync::Mutex;

/// Serialize libclang use: its runtime permits one live scanner per process.
static SCANNER_LOCK: Mutex<()> = Mutex::new(());

/// The fixture directory, holding `postgres.h` and one `pg_config.h` directory per machine.
fn fixtures() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/postgres_configuration")
}

/// Inspect the fixture `postgres.h` with `server` on the include path, then verify it.
fn verify(
    scanner: &MacroScanner,
    server: &Path,
    target: &str,
    major: u16,
) -> Result<(), ConfigurationError> {
    let arguments = [format!("--target={target}"), format!("-I{}", server.display())];
    let frontend = inspect(scanner, &fixtures().join("postgres.h"), &arguments, None)
        .unwrap_or_else(|error| panic!("inspect {} for {target}: {error}", server.display()));
    verify_postgres_configuration(scanner, &frontend, major, &[])
}

/// A server root holding a fixture `pg_config.h` with textual edits applied.
fn edited(machine: &str, edits: &[(&str, &str)]) -> tempfile::TempDir {
    let mut text = std::fs::read_to_string(fixtures().join(machine).join("pg_config.h")).unwrap();
    for (from, to) in edits {
        assert!(text.contains(from), "{machine}/pg_config.h lacks {from:?}");
        text = text.replace(from, to);
    }
    let server = tempfile::tempdir().unwrap();
    std::fs::write(server.path().join("pg_config.h"), text).unwrap();
    server
}

/// The disagreeing fact names of a layout refusal.
fn disagreeing_facts(error: &ConfigurationError) -> Vec<&str> {
    let ConfigurationError::Layout { disagreements, .. } = error else {
        panic!("expected a layout refusal: {error:?}");
    };
    disagreements.iter().map(|disagreement| disagreement.fact.as_str()).collect()
}

/// x86_64 headers describe x86_64 Linux. aarch64 shares their LP64 layout, so only the machine
/// check refuses it; x32 shares their machine name, so only the layout witness refuses it.
#[test]
fn x86_64_headers_are_refused_for_aarch64_and_x32() {
    let _lock = SCANNER_LOCK.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let scanner = MacroScanner::new().expect("libclang must be available");
    let server = fixtures().join("x86_64_linux");
    verify(&scanner, &server, "x86_64-unknown-linux-gnu", 18).unwrap();

    let error = verify(&scanner, &server, "aarch64-unknown-linux-gnu", 18).unwrap_err();
    assert!(
        matches!(&error, ConfigurationError::Machine { recorded, target }
            if recorded == "x86_64-pc-linux-gnu" && target == "aarch64-unknown-linux-gnu"),
        "{error:?}"
    );
    // The message carries the disagreement, so an error chain must not repeat it as a source.
    let error = PostgresError::TargetConfiguration(error);
    assert!(std::error::Error::source(&error).is_none());
    assert_eq!(
        error.to_string(),
        "PostgreSQL headers do not describe the inspected C target: pg_config.h was configured \
         for x86_64-pc-linux-gnu (PG_VERSION_STR), but Clang targets aarch64-unknown-linux-gnu"
    );

    let error = verify(&scanner, &server, "x86_64-unknown-linux-gnux32", 18).unwrap_err();
    assert_eq!(
        disagreeing_facts(&error),
        ["ALIGNOF_LONG", "SIZEOF_LONG", "SIZEOF_SIZE_T", "SIZEOF_VOID_P"],
        "{error}"
    );
}

/// configure and meson record member offsets, which are 4 for double and long long on i686
/// Linux although `_Alignof` reports 8. A configuration claiming 8 is refused.
#[test]
fn i686_alignment_facts_are_member_offsets() {
    let _lock = SCANNER_LOCK.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let scanner = MacroScanner::new().expect("libclang must be available");
    verify(&scanner, &fixtures().join("i686_linux"), "i686-unknown-linux-gnu", 17).unwrap();

    let server = edited(
        "i686_linux",
        &[("ALIGNOF_DOUBLE 4", "ALIGNOF_DOUBLE 8"), ("MAXIMUM_ALIGNOF 4", "MAXIMUM_ALIGNOF 8")],
    );
    let error = verify(&scanner, server.path(), "i686-unknown-linux-gnu", 17).unwrap_err();
    let ConfigurationError::Layout { disagreements, .. } = &error else { panic!("{error:?}") };
    assert_eq!(
        disagreements,
        &[
            LayoutDisagreement { fact: "ALIGNOF_DOUBLE".into(), headers: 8, target: 4 },
            LayoutDisagreement { fact: "MAXIMUM_ALIGNOF".into(), headers: 8, target: 4 },
        ]
    );

    // i686 has no __int128, so the probe itself fails, and its error names the cause.
    let server = edited(
        "i686_linux",
        &[(
            "#define PG_INT64_TYPE",
            "#define PG_INT128_TYPE __int128\n#define ALIGNOF_PG_INT128_TYPE 16\n#define PG_INT64_TYPE",
        )],
    );
    let error = verify(&scanner, server.path(), "i686-unknown-linux-gnu", 17).unwrap_err();
    assert!(matches!(error, ConfigurationError::Probe(_)), "{error:?}");
    assert!(error.to_string().contains("__int128"), "{error}");
}

/// meson names both ppc64 byte orders `ppc64`, so the layout witness decides between them.
#[test]
fn meson_ppc64_byte_order_comes_from_words_bigendian() {
    let _lock = SCANNER_LOCK.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let scanner = MacroScanner::new().expect("libclang must be available");
    let server = fixtures().join("ppc64_linux");
    verify(&scanner, &server, "powerpc64le-unknown-linux-gnu", 18).unwrap();
    let error = verify(&scanner, &server, "powerpc64-unknown-linux-gnu", 18).unwrap_err();
    let ConfigurationError::Layout { disagreements, .. } = &error else { panic!("{error:?}") };
    assert_eq!(
        disagreements,
        &[LayoutDisagreement { fact: "WORDS_BIGENDIAN".into(), headers: 0, target: 1 }]
    );
}

/// meson's Windows headers record an LLP64 layout with 4-byte long.
#[test]
fn meson_windows_headers_describe_llp64_windows() {
    let _lock = SCANNER_LOCK.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let scanner = MacroScanner::new().expect("libclang must be available");
    let server = fixtures().join("x86_64_windows");
    verify(&scanner, &server, "x86_64-w64-windows-gnu", 18).unwrap();
    assert!(matches!(
        verify(&scanner, &server, "x86_64-unknown-linux-gnu", 18),
        Err(ConfigurationError::Machine { .. })
    ));
}

/// Headers without PG_VERSION_NUM are not a PostgreSQL configuration and are not checked. A
/// PostgreSQL configuration must belong to the selected major and define every required fact.
#[test]
fn configuration_requires_the_selected_major_and_required_facts() {
    let _lock = SCANNER_LOCK.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let scanner = MacroScanner::new().expect("libclang must be available");
    let synthetic = edited("x86_64_linux", &[("#define PG_VERSION_NUM 180006\n", "")]);
    verify(&scanner, synthetic.path(), "aarch64-unknown-linux-gnu", 18).unwrap();

    let pg17 = edited("x86_64_linux", &[("PG_MAJORVERSION_NUM 18", "PG_MAJORVERSION_NUM 17")]);
    assert!(matches!(
        verify(&scanner, pg17.path(), "x86_64-unknown-linux-gnu", 18),
        Err(ConfigurationError::Major { headers: 17, expected: 18 })
    ));

    let missing = edited("x86_64_linux", &[("#define ALIGNOF_DOUBLE 8\n", "")]);
    assert!(matches!(
        verify(&scanner, missing.path(), "x86_64-unknown-linux-gnu", 18),
        Err(ConfigurationError::MissingFact("ALIGNOF_DOUBLE"))
    ));
}

/// Installation inspection runs the verification, and an overriding server root owns the
/// macros inspected through it.
#[cfg(unix)]
#[test]
fn postgres_config_inspection_verifies_the_target_configuration() {
    use pgrx_c_macros::{PostgresConfig, postgres_object_macro_names};
    use std::os::unix::fs::PermissionsExt;

    let _lock = SCANNER_LOCK.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let scanner = MacroScanner::new().expect("libclang must be available");
    let directory = tempfile::tempdir().unwrap();
    let x86_64 = fixtures().join("x86_64_linux");
    let executable = directory.path().join("pg_config");
    std::fs::write(
        &executable,
        format!(
            "#!/bin/sh\n\
             case \"$1\" in\n\
             --version) echo 'PostgreSQL 18.6' ;;\n\
             --includedir-server) echo '{}' ;;\n\
             --cppflags|--cflags|--configure) echo '' ;;\n\
             *) exit 1 ;;\n\
             esac\n",
            x86_64.display()
        ),
    )
    .unwrap();
    std::fs::set_permissions(&executable, std::fs::Permissions::from_mode(0o700)).unwrap();
    let postgres =
        PostgresConfig::from_pg_config(pgrx_pg_config::PgConfig::new_with_defaults(executable))
            .unwrap();
    let header = fixtures().join("postgres.h");
    let target = |triple: &str| vec![format!("--target={triple}")];
    postgres.inspect(&scanner, Some(&header), &target("x86_64-unknown-linux-gnu"), None).unwrap();
    let error = postgres
        .inspect(&scanner, Some(&header), &target("aarch64-unknown-linux-gnu"), None)
        .unwrap_err();
    assert!(
        matches!(error, PostgresError::TargetConfiguration(ConfigurationError::Machine { .. })),
        "{error:?}"
    );

    let ppc64 = fixtures().join("ppc64_linux");
    let postgres = postgres.with_server_include_dir(ppc64.clone());
    assert_eq!(postgres.server_include_dir(), ppc64);
    let mut arguments = target("powerpc64le-unknown-linux-gnu");
    arguments.push(format!("-I{}", ppc64.display()));
    let frontend = postgres.inspect_with_arguments(&scanner, &header, &arguments, None).unwrap();
    let owned = postgres_object_macro_names(&frontend, postgres.server_include_dir()).unwrap();
    assert!(owned.iter().any(|name| name == "PG_VERSION_STR"), "{owned:?}");
}

/// A universal macOS installation, such as Postgres.app, records `-arch arm64 -arch x86_64` in
/// CFLAGS and names only the machine configure ran on. Inspection leaves the slices out of the
/// profile and accepts its own headers for either slice, but not for another machine, and not
/// once another server root owns the inspected macros.
#[cfg(unix)]
#[test]
fn universal_macos_installations_describe_each_compiled_slice() {
    use pgrx_c_macros::PostgresConfig;
    use std::os::unix::fs::PermissionsExt;

    let _lock = SCANNER_LOCK.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let scanner = MacroScanner::new().expect("libclang must be available");
    let server = edited(
        "x86_64_linux",
        &[(
            "(Ubuntu 18.6-1.pgdg24.04+2) on x86_64-pc-linux-gnu, compiled by gcc (Ubuntu 13.3.0-6ubuntu2~24.04.1) 13.3.0",
            "(Postgres.app) on aarch64-apple-darwin23.6.0, compiled by Apple clang version 15.0.0",
        )],
    );
    let directory = tempfile::tempdir().unwrap();
    let pg_config = |cflags: &str| {
        let executable = directory.path().join(format!("pg_config{}", cflags.len()));
        std::fs::write(
            &executable,
            format!(
                "#!/bin/sh\n\
                 case \"$1\" in\n\
                 --version) echo 'PostgreSQL 18.6' ;;\n\
                 --includedir-server) echo '{}' ;;\n\
                 --cflags) echo '{cflags}' ;;\n\
                 --cppflags|--configure) echo '' ;;\n\
                 *) exit 1 ;;\n\
                 esac\n",
                server.path().display()
            ),
        )
        .unwrap();
        std::fs::set_permissions(&executable, std::fs::Permissions::from_mode(0o700)).unwrap();
        PostgresConfig::from_pg_config(pgrx_pg_config::PgConfig::new_with_defaults(executable))
            .unwrap()
    };
    let header = fixtures().join("postgres.h");
    let target = |triple: &str| vec![format!("--target={triple}")];
    let universal = pg_config("-fwrapv -Os -mmacosx-version-min=10.15 -arch arm64 -arch x86_64");
    for triple in ["x86_64-apple-darwin", "arm64-apple-darwin"] {
        let frontend = universal
            .inspect(&scanner, Some(&header), &target(triple), None)
            .unwrap_or_else(|error| panic!("{triple}: {error}"));
        let arguments = &frontend.profile().arguments;
        assert!(!arguments.iter().any(|argument| argument == "-arch"), "{arguments:?}");
        assert!(arguments.iter().any(|argument| argument == "-fwrapv"), "{arguments:?}");
    }
    let error = universal
        .inspect(&scanner, Some(&header), &target("aarch64-unknown-linux-gnu"), None)
        .unwrap_err();
    assert!(
        matches!(error, PostgresError::TargetConfiguration(ConfigurationError::Machine { .. })),
        "{error:?}"
    );

    let x86_64_only = pg_config("-fwrapv -arch x86_64");
    let error = x86_64_only
        .inspect(&scanner, Some(&header), &target("arm64-apple-darwin"), None)
        .unwrap_err();
    assert!(
        matches!(
            &error,
            PostgresError::TargetConfiguration(ConfigurationError::Architectures { compiled, .. })
                if compiled == &["x86_64"]
        ),
        "{error:?}"
    );

    // The recorded slices describe the installation's headers, not an overriding root's.
    let copy = tempfile::tempdir().unwrap();
    std::fs::copy(server.path().join("pg_config.h"), copy.path().join("pg_config.h")).unwrap();
    let mut arguments = target("x86_64-apple-darwin");
    arguments.push(format!("-I{}", copy.path().display()));
    let error = universal
        .with_server_include_dir(copy.path().into())
        .inspect_with_arguments(&scanner, &header, &arguments, None)
        .unwrap_err();
    assert!(
        matches!(error, PostgresError::TargetConfiguration(ConfigurationError::Machine { .. })),
        "{error:?}"
    );
}
