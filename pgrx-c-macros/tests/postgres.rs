//! Check selected-installation ownership and wrapper resolution.
//!
//! Synthetic pg_config roots, include trees, and symlinks distinguish PostgreSQL
//! macros from external context definitions. Raw history remains available for
//! expansion, while public selection follows physical server-header ownership.

#![cfg(unix)]

/// Use the production scanner, analysis, and emission contracts so these checks exercise the
/// actual C macro pipeline.
use pgrx_c_macros::{
    DiagnosticSeverity, MacroDefinition, MacroScanner, PostgresConfig, PostgresError,
};
/// Resolve configured PostgreSQL installations through the same metadata used by ordinary pgrx
/// builds.
use pgrx_pg_config::PgConfig;
/// Create isolated filesystem aliases used to verify physical provenance and input identity.
use std::os::unix::fs::{PermissionsExt, symlink};
/// Keep fixture and generated-output locations explicit so consumer builds remain independent
/// of the working directory.
use std::path::{Path, PathBuf};
/// Serialize shared Clang runtime ownership for scanner-backed tests in this process.
use std::sync::Mutex;
/// Bound compiler processes and choose isolated temporary names without reusing prior oracle
/// artifacts.
use std::time::{SystemTime, UNIX_EPOCH};

// The clang wrapper permits one live Clang instance in a process.
/// Serialize libclang-backed inspection within this test process because its safe runtime
/// permits one active owner.
static SCANNER_LOCK: Mutex<()> = Mutex::new(());

/// Own synthetic PostgreSQL configuration and header roots, keeping CLI and ownership tests
/// independent of the developer's setup.
struct TestConfig {
    /// Owned PGRX_HOME containing only the fixture's configuration.
    home: PathBuf,
    /// Owned server-header root used for version and physical ownership resolution.
    include_dir: PathBuf,
}

/// Build isolated installation metadata and commands for configuration and ownership
/// assertions.
impl TestConfig {
    /// Create an isolated configuration root and header trees for physical ownership tests.
    fn new() -> Self {
        let nonce = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();
        let home = std::env::temp_dir()
            .join(format!("pgrx-c-macros-postgres {} '{nonce}", std::process::id()));
        std::fs::create_dir(&home).expect("test directory must be created");
        let config = Self { include_dir: home.join("server"), home };
        std::fs::create_dir(&config.include_dir).unwrap();
        config
    }

    /// Create one fixture header under an owned path and ensure its parent directories exist.
    fn write(&self, relative: &str, source: &str) -> PathBuf {
        let path = self.home.join(relative);
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();
        std::fs::write(&path, source).unwrap();
        path
    }

    /// Build the selected PostgreSQL configuration against the fixture's normal server include
    /// root.
    fn postgres(&self) -> PostgresConfig {
        self.postgres_with_include_dir(&self.include_dir)
    }

    /// Build fixture PostgreSQL metadata with an explicitly selected server root to test
    /// aliases and ownership boundaries.
    fn postgres_with_include_dir(&self, include_dir: &Path) -> PostgresConfig {
        let quoted_include = format!("'{}'", include_dir.to_str().unwrap().replace('\'', "'\\''"));
        let executable = self.write(
            "pg_config",
            &format!(
                "#!/bin/sh\n\
                 case \"$1\" in\n\
                 --version) printf '%s\\n' 'PostgreSQL 18.0' ;;\n\
                 --includedir-server) printf '%s\\n' {quoted_include} ;;\n\
                 --cppflags) printf '%s\\n' '-DOWNERSHIP_COMMAND_LINE(value)=((value)+4)' ;;\n\
                 *) exit 1 ;;\n\
                 esac\n"
            ),
        );
        std::fs::set_permissions(&executable, std::fs::Permissions::from_mode(0o700)).unwrap();
        PostgresConfig::from_pg_config(PgConfig::new_with_defaults(executable))
            .expect("self-contained pg_config must resolve without changing the environment")
    }
}

/// Release only temporary artifacts owned by this fixture, including on failed compiler or
/// assertion paths.
impl Drop for TestConfig {
    /// Remove only this fixture's owned temporary storage after the test or oracle completes.
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.home);
    }
}

/// Project raw ownership classifications to names so scans can be compared without discarding
/// definition history.
fn ownership_names(definitions: &[MacroDefinition]) -> Vec<&str> {
    definitions
        .iter()
        .map(|definition| definition.name.as_str())
        .filter(|name| name.starts_with("OWNERSHIP_"))
        .collect()
}

/// Checks that the inventory retains every raw definition in order in either primary inventory
/// or context.
#[test]
fn retains_every_raw_definition_in_order_in_either_primary_inventory_or_context() {
    let _lock = SCANNER_LOCK.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let scanner = MacroScanner::new().expect("libclang must be available for these tests");
    let config = TestConfig::new();
    config.write(
        "server/macros.h",
        "#define OWNERSHIP_OBJECT 10\n\
         #define OWNERSHIP_Z(value) ((value) + OWNERSHIP_OBJECT)\n\
         #define OWNERSHIP_REPEAT(value) ((value) + 1)\n\
         #undef OWNERSHIP_REPEAT\n\
         #define OWNERSHIP_A(value) (value)\n\
         #define OWNERSHIP_REPEAT(value) ((value) + 2)\n",
    );
    let wrapper = config.write(
        "wrapper.h",
        "#warning ownership test warning\n\
         #define OWNERSHIP_EXTERNAL_BEFORE(value) (value)\n\
         #include <macros.h>\n\
         #define OWNERSHIP_EXTERNAL_AFTER(value) (value)\n",
    );
    let postgres = config.postgres();
    let raw = scanner.scan(&wrapper, postgres.clang_args()).unwrap();
    assert_eq!(
        ownership_names(&raw.macros),
        [
            "OWNERSHIP_COMMAND_LINE",
            "OWNERSHIP_EXTERNAL_BEFORE",
            "OWNERSHIP_OBJECT",
            "OWNERSHIP_Z",
            "OWNERSHIP_REPEAT",
            "OWNERSHIP_A",
            "OWNERSHIP_REPEAT",
            "OWNERSHIP_EXTERNAL_AFTER",
        ]
    );
    let partitioned = postgres.scan(&scanner, Some(&wrapper), &[]).unwrap();
    assert_eq!(
        ownership_names(&partitioned.inventory.macros),
        ["OWNERSHIP_Z", "OWNERSHIP_REPEAT", "OWNERSHIP_A", "OWNERSHIP_REPEAT"]
    );
    assert_eq!(
        ownership_names(&partitioned.context),
        [
            "OWNERSHIP_COMMAND_LINE",
            "OWNERSHIP_EXTERNAL_BEFORE",
            "OWNERSHIP_OBJECT",
            "OWNERSHIP_EXTERNAL_AFTER",
        ]
    );
    let primary = |definition: &&MacroDefinition| {
        matches!(definition.name.as_str(), "OWNERSHIP_Z" | "OWNERSHIP_REPEAT" | "OWNERSHIP_A")
    };
    assert_eq!(
        partitioned.inventory.macros,
        raw.macros.iter().filter(primary).cloned().collect::<Vec<_>>()
    );
    assert_eq!(
        partitioned.context,
        raw.macros.iter().filter(|definition| !primary(definition)).cloned().collect::<Vec<_>>()
    );
    let command_line = partitioned
        .context
        .iter()
        .find(|definition| definition.name == "OWNERSHIP_COMMAND_LINE")
        .unwrap();
    assert!(command_line.provenance.is_none());
    assert!(!command_line.builtin);
    assert!(partitioned.context.iter().any(|definition| definition.builtin));
    assert!(partitioned.inventory.diagnostics.iter().any(|diagnostic| {
        diagnostic.severity == DiagnosticSeverity::Warning
            && diagnostic.message.contains("ownership test warning")
    }));
    assert_eq!(partitioned.inventory.diagnostics, raw.diagnostics);
}

/// Checks that custom wrappers and extra includes do not expand the selected server tree.
#[test]
fn custom_wrappers_and_extra_includes_do_not_expand_the_selected_server_tree() {
    let _lock = SCANNER_LOCK.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let scanner = MacroScanner::new().expect("libclang must be available for these tests");
    let config = TestConfig::new();
    config.write("server/primary.h", "#define OWNERSHIP_PRIMARY(value) (value)\n");
    config.write("server-extra/sibling.h", "#define OWNERSHIP_SIBLING(value) (value)\n");
    config.write("external.h", "#define OWNERSHIP_DOTDOT(value) (value)\n");
    config.write("extra/added.h", "#define OWNERSHIP_EXTRA_INCLUDE(value) (value)\n");
    std::fs::create_dir(config.include_dir.join("nested")).unwrap();
    let wrapper = config.write(
        "wrapper.h",
        "#define OWNERSHIP_WRAPPER(value) (value)\n\
         #include \"server/nested/../primary.h\"\n\
         #include \"server-extra/sibling.h\"\n\
         #include \"server/../external.h\"\n\
         #include <added.h>\n",
    );
    let extra_include = format!("-I{}", config.home.join("extra").display());
    let partitioned = config.postgres().scan(&scanner, Some(&wrapper), &[extra_include]).unwrap();
    assert_eq!(ownership_names(&partitioned.inventory.macros), ["OWNERSHIP_PRIMARY"]);
    assert_eq!(
        ownership_names(&partitioned.context),
        [
            "OWNERSHIP_COMMAND_LINE",
            "OWNERSHIP_WRAPPER",
            "OWNERSHIP_SIBLING",
            "OWNERSHIP_DOTDOT",
            "OWNERSHIP_EXTRA_INCLUDE",
        ]
    );
}

/// Checks that line directives cannot disguise physical header ownership.
#[test]
fn line_directives_cannot_disguise_physical_header_ownership() {
    let _lock = SCANNER_LOCK.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let scanner = MacroScanner::new().expect("libclang must be available for these tests");
    let config = TestConfig::new();
    let primary = config.write(
        "server/primary.h",
        "#line 500 \"outside.h\"\n#define OWNERSHIP_PRIMARY(value) (value)\n",
    );
    let spoofed_filename = config
        .include_dir
        .join("forged.h")
        .to_str()
        .unwrap()
        .replace('\\', "\\\\")
        .replace('"', "\\\"");
    let external = config.write(
        "external.h",
        &format!("#line 700 \"{spoofed_filename}\"\n#define OWNERSHIP_EXTERNAL(value) (value)\n"),
    );
    let wrapper = config.write("wrapper.h", "#include <primary.h>\n#include \"external.h\"\n");
    let partitioned = config.postgres().scan(&scanner, Some(&wrapper), &[]).unwrap();
    assert_eq!(ownership_names(&partitioned.inventory.macros), ["OWNERSHIP_PRIMARY"]);
    let definition = &partitioned.inventory.macros[0];
    let provenance = definition.provenance.as_ref().unwrap();
    assert_eq!(provenance.file.canonicalize().unwrap(), primary.canonicalize().unwrap());
    assert_eq!((provenance.start_line, provenance.end_line), (2, 2));
    let definition = partitioned
        .context
        .iter()
        .find(|definition| definition.name == "OWNERSHIP_EXTERNAL")
        .unwrap();
    let provenance = definition.provenance.as_ref().unwrap();
    assert_eq!(provenance.file.canonicalize().unwrap(), external.canonicalize().unwrap());
    assert_eq!((provenance.start_line, provenance.end_line), (2, 2));
}

/// Checks that configuration resolves file symlinks in both directions and refreshes ownership
/// between scans.
#[test]
fn resolves_file_symlinks_in_both_directions_and_refreshes_ownership_between_scans() {
    let _lock = SCANNER_LOCK.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let scanner = MacroScanner::new().expect("libclang must be available for these tests");
    let config = TestConfig::new();
    let primary = config.write("server/primary.h", "#define OWNERSHIP_PRIMARY(value) (value)\n");
    let external = config.write("external.h", "#define OWNERSHIP_EXTERNAL(value) (value)\n");
    let outside_alias = config.home.join("primary-alias.h");
    let inside_alias = config.include_dir.join("external-alias.h");
    symlink(&primary, &outside_alias).unwrap();
    symlink(&external, &inside_alias).unwrap();
    let wrapper =
        config.write("wrapper.h", "#include \"primary-alias.h\"\n#include <external-alias.h>\n");
    let postgres = config.postgres();
    let partitioned = postgres.scan(&scanner, Some(&wrapper), &[]).unwrap();
    assert_eq!(ownership_names(&partitioned.inventory.macros), ["OWNERSHIP_PRIMARY"]);
    assert_eq!(
        ownership_names(&partitioned.context),
        ["OWNERSHIP_COMMAND_LINE", "OWNERSHIP_EXTERNAL"]
    );
    std::fs::remove_file(&outside_alias).unwrap();
    symlink(&external, &outside_alias).unwrap();
    let partitioned = postgres.scan(&scanner, Some(&wrapper), &[]).unwrap();
    assert!(partitioned.inventory.macros.is_empty());
    assert_eq!(
        ownership_names(&partitioned.context),
        ["OWNERSHIP_COMMAND_LINE", "OWNERSHIP_EXTERNAL", "OWNERSHIP_EXTERNAL"]
    );
}

/// Checks that configuration accepts a configured server root symlink.
#[test]
fn accepts_a_configured_server_root_symlink() {
    let _lock = SCANNER_LOCK.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let scanner = MacroScanner::new().expect("libclang must be available for these tests");
    let config = TestConfig::new();
    config.write("server/primary.h", "#define OWNERSHIP_PRIMARY(value) (value)\n");
    let alias = config.home.join("server-link");
    symlink(&config.include_dir, &alias).unwrap();
    let wrapper = config.write("wrapper.h", "#include <primary.h>\n");
    let partitioned =
        config.postgres_with_include_dir(&alias).scan(&scanner, Some(&wrapper), &[]).unwrap();
    assert_eq!(ownership_names(&partitioned.inventory.macros), ["OWNERSHIP_PRIMARY"]);
    assert_eq!(ownership_names(&partitioned.context), ["OWNERSHIP_COMMAND_LINE"]);
}

/// Checks that configuration reports a server root removed after configuration.
#[test]
fn reports_a_server_root_removed_after_configuration() {
    let _lock = SCANNER_LOCK.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let scanner = MacroScanner::new().expect("libclang must be available for these tests");
    let config = TestConfig::new();
    let wrapper = config.write("wrapper.h", "#define OWNERSHIP_WRAPPER(value) (value)\n");
    let postgres = config.postgres();
    std::fs::remove_dir(&config.include_dir).unwrap();
    assert!(matches!(
        postgres.scan(&scanner, Some(&wrapper), &[]),
        Err(PostgresError::HeaderPath { path, source })
            if path == config.include_dir && source.kind() == std::io::ErrorKind::NotFound
    ));
}
