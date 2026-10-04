//LICENSE Portions Copyright 2026 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.

//! Check physical header identity and fresh native build artifacts.
//!
//! Temporary files and symlinks exercise include lookup, dependency freshness,
//! source/output alias rejection, and failed tools. Existing objects or archives
//! must never substitute for missing fresh output, and ownership follows real
//! files rather than spoofed logical locations.

#![cfg(unix)]

/// Use the production scanner, analysis, and emission contracts so these checks exercise the
/// actual C macro pipeline.
use pgrx_c_macros::{
    ActiveProvenance, AnalysisSession, FrontendError, MacroScanner, compile_native_support, inspect,
};
/// Keep fixture and generated-output locations explicit so consumer builds remain independent
/// of the working directory.
use std::path::PathBuf;
/// Serialize shared Clang runtime ownership for scanner-backed tests in this process.
use std::sync::Mutex;

/// Serialize libclang-backed inspection within this test process because its safe runtime
/// permits one active owner.
static SCANNER_LOCK: Mutex<()> = Mutex::new(());

/// Own the temporary header tree for this test and remove only its fixture files afterward.
struct Directory(
    /// Owned fixture path used for isolated inputs and cleanup.
    PathBuf,
);

/// Construct owned header trees for freshness and include-identity tests.
impl Directory {
    /// Create owned, uniquely named fixture storage so this test's headers and compiler outputs
    /// cannot collide with another invocation.
    fn new() -> Self {
        for attempt in 0..64 {
            let path = std::env::temp_dir()
                .join(format!("pgrx-frontend-paths-{}-{attempt}", std::process::id()));
            match std::fs::create_dir(&path) {
                Ok(()) => return Self(path),
                Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => continue,
                Err(error) => panic!("could not create temporary fixture: {error}"),
            }
        }
        panic!("could not reserve a temporary fixture directory");
    }
}

/// Release only temporary artifacts owned by this fixture, including on failed compiler or
/// assertion paths.
impl Drop for Directory {
    /// Remove only this fixture's owned temporary storage after the test or oracle completes.
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

/// Checks that syntax only profile cannot rearchive a stale native object.
#[test]
fn syntax_only_profile_cannot_rearchive_a_stale_native_object() {
    let _lock = SCANNER_LOCK.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let directory = Directory::new();
    let header = directory.0.join("native.h");
    let source = directory.0.join("native.c");
    let object = directory.0.join("native.o");
    let archive = directory.0.join("native.a");
    std::fs::write(&header, "int native_version(void);\n").unwrap();
    std::fs::write(&source, "int native_version(void) { return 1; }\n").unwrap();
    let scanner = MacroScanner::new().unwrap();
    let frontend = inspect(&scanner, &header, &[], None).unwrap();
    let mut profile = frontend.profile().clone();
    compile_native_support(&profile, &source, &object, &archive).unwrap();
    let old_object = std::fs::read(&object).unwrap();
    let old_archive = std::fs::read(&archive).unwrap();
    std::fs::write(&source, "int native_version(void) { return 2; }\n").unwrap();
    profile.arguments.push("-fsyntax-only".into());
    assert!(matches!(
        compile_native_support(&profile, &source, &object, &archive),
        Err(FrontendError::Arguments(_))
    ));
    assert_eq!(std::fs::read(&object).unwrap(), old_object);
    assert_eq!(std::fs::read(&archive).unwrap(), old_archive);
}

/// Checks that native support requires fresh outputs and preserves existing artifacts on tool
/// failure.
#[test]
fn native_support_requires_fresh_outputs_and_preserves_existing_artifacts_on_tool_failure() {
    /// Make fixture compiler or pg_config wrappers executable so process failures can be tested
    /// directly.
    use std::os::unix::fs::PermissionsExt;
    let _lock = SCANNER_LOCK.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let directory = Directory::new();
    let header = directory.0.join("native.h");
    let source = directory.0.join("native.c");
    let object = directory.0.join("native.o");
    let archive = directory.0.join("native.a");
    std::fs::write(&header, "int native_version(void);\n").unwrap();
    std::fs::write(&source, "int native_version(void) { return 1; }\n").unwrap();
    let scanner = MacroScanner::new().unwrap();
    let frontend = inspect(&scanner, &header, &[], None).unwrap();
    let profile = frontend.profile();
    compile_native_support(profile, &source, &object, &archive).unwrap();
    let old_object = std::fs::read(&object).unwrap();
    let old_archive = std::fs::read(&archive).unwrap();
    std::fs::write(&source, "int native_version(void) { return 2; }\n").unwrap();
    for action in ["-###", "-fdriver-only", "--version"] {
        let mut changed = profile.clone();
        changed.arguments.push(action.into());
        let error = compile_native_support(&changed, &source, &object, &archive).unwrap_err();
        assert!(matches!(error, FrontendError::Output(_)), "{action}: {error}");
        assert_eq!(std::fs::read(&object).unwrap(), old_object);
        assert_eq!(std::fs::read(&archive).unwrap(), old_archive);
    }
    let compiler = directory.0.join("clang");
    std::os::unix::fs::symlink(&profile.compiler.executable, &compiler).unwrap();
    let archiver = directory.0.join("llvm-ar");
    let mut changed = profile.clone();
    changed.compiler.executable = compiler;
    for body in ["exit 0", ": > \"$2\"", "ln -s \"$3\" \"$2\""] {
        std::fs::write(&archiver, format!("#!/bin/sh\n{body}\n")).unwrap();
        std::fs::set_permissions(&archiver, std::fs::Permissions::from_mode(0o700)).unwrap();
        let error = compile_native_support(&changed, &source, &object, &archive).unwrap_err();
        assert!(matches!(error, FrontendError::Output(_)), "{body}: {error}");
        assert_eq!(std::fs::read(&object).unwrap(), old_object);
        assert_eq!(std::fs::read(&archive).unwrap(), old_archive);
    }
    let replacement = directory.0.join("replacement.o");
    compile_native_support(profile, &source, &replacement, &archive).unwrap();
    let adjacent = profile.compiler.executable.with_file_name("llvm-ar");
    let archiver = if adjacent.is_file() { adjacent } else { PathBuf::from("ar") };
    let members = std::process::Command::new(archiver).arg("t").arg(&archive).output().unwrap();
    assert!(members.status.success());
    let members = String::from_utf8(members.stdout).unwrap();
    // Darwin's archiver lists its symbol index alongside the actual objects.
    let objects = members.lines().filter(|name| *name != "__.SYMDEF SORTED").collect::<Vec<_>>();
    assert_eq!(objects, ["replacement.o"]);
    assert!(std::fs::read_dir(&directory.0).unwrap().all(|entry| {
        !entry.unwrap().file_name().to_string_lossy().starts_with(".pgrx-native-")
    }));
}

/// Checks that native support rejects source and output aliases before publication.
#[test]
fn native_support_rejects_source_and_output_aliases_before_publication() {
    let _lock = SCANNER_LOCK.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let directory = Directory::new();
    let header = directory.0.join("native.h");
    let source = directory.0.join("native.c");
    let object = directory.0.join("native.o");
    let archive = directory.0.join("native.a");
    let contents = "int native_version(void) { return 1; }\n";
    std::fs::write(&header, "int native_version(void);\n").unwrap();
    std::fs::write(&source, contents).unwrap();
    let scanner = MacroScanner::new().unwrap();
    let frontend = inspect(&scanner, &header, &[], None).unwrap();
    let alias = directory.0.join("alias.c");
    std::os::unix::fs::symlink(&source, &alias).unwrap();
    let root_alias = directory.0.join("root");
    std::os::unix::fs::symlink("/", &root_alias).unwrap();
    for (object, archive) in [
        (&source, &archive),
        (&object, &source),
        (&alias, &archive),
        (&object, &object),
        (&root_alias, &archive),
    ] {
        assert!(matches!(
            compile_native_support(frontend.profile(), &source, object, archive),
            Err(FrontendError::Arguments(_))
        ));
        assert_eq!(std::fs::read_to_string(&source).unwrap(), contents);
    }
    let object_directory = directory.0.join("objects");
    let archive_directory = directory.0.join("archives");
    std::fs::create_dir(&object_directory).unwrap();
    std::fs::create_dir(&archive_directory).unwrap();
    let object = object_directory.join("same-name");
    let archive = archive_directory.join("same-name");
    compile_native_support(frontend.profile(), &source, &object, &archive).unwrap();
    assert!(std::fs::metadata(&object).unwrap().len() > 0);
    assert!(std::fs::metadata(&archive).unwrap().len() > 0);
}

/// Checks that symlink wrapper keeps quoted include lookup at the supplied path.
#[test]
fn symlink_wrapper_keeps_quoted_include_lookup_at_the_supplied_path() {
    let _lock = SCANNER_LOCK.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let directory = Directory::new();
    let source = directory.0.join("source");
    let entry = directory.0.join("entry");
    std::fs::create_dir(&source).unwrap();
    std::fs::create_dir(&entry).unwrap();
    std::fs::write(
        source.join("wrapper.h"),
        "#include \"choice.h\"\n#define WRAPPER_FUNCTION(x) ((x) + SELECTED_CHOICE)\n",
    )
    .unwrap();
    std::fs::write(source.join("choice.h"), "#define SELECTED_CHOICE 41\n").unwrap();
    std::fs::write(entry.join("choice.h"), "#define SELECTED_CHOICE 7\n").unwrap();
    let wrapper = entry.join("wrapper.h");
    std::os::unix::fs::symlink(source.join("wrapper.h"), &wrapper).unwrap();
    let scanner = MacroScanner::new().unwrap();
    let inspection = inspect(&scanner, &wrapper, &[], None).unwrap();
    let selected = &inspection.environment().active["SELECTED_CHOICE"].definition;
    assert_eq!(selected.tokens.last().unwrap().spelling, "7");
    assert_eq!(inspection.profile().header, wrapper);
    assert!(inspection.profile().inputs.files.contains(&wrapper));
    assert!(inspection.profile().inputs.files.contains(&wrapper.canonicalize().unwrap()));
    assert!(inspection.profile().inputs.files.contains(&entry.join("choice.h")));
    assert!(!inspection.profile().inputs.files.contains(&source.join("choice.h")));
}

/// Checks that repeated inclusion of one definition is not ambiguous.
#[test]
fn repeated_inclusion_of_one_definition_is_not_ambiguous() {
    let _lock = SCANNER_LOCK.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let directory = Directory::new();
    let wrapper = directory.0.join("wrapper.h");
    std::fs::write(&wrapper, "#include \"repeated.h\"\n#include \"repeated.h\"\n").unwrap();
    std::fs::write(directory.0.join("repeated.h"), "#define REPEATED_FUNCTION(x) ((x) + 1)\n")
        .unwrap();
    let scanner = MacroScanner::new().unwrap();
    let inspection = inspect(&scanner, &wrapper, &[], None).unwrap();
    assert_eq!(
        inspection
            .inventory()
            .macros
            .iter()
            .filter(|definition| definition.name == "REPEATED_FUNCTION")
            .count(),
        2
    );
    assert!(matches!(
        inspection.environment().active["REPEATED_FUNCTION"].provenance,
        ActiveProvenance::Resolved
    ));
}

/// Checks that header availability tracks symlink identity and rejects changed inputs.
#[test]
fn header_availability_tracks_symlink_identity_and_rejects_changed_inputs() {
    let _lock = SCANNER_LOCK.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let directory = Directory::new();
    let first = directory.0.join("first.h");
    let second = directory.0.join("second.h");
    let alias = directory.0.join("available.h");
    let wrapper = directory.0.join("wrapper.h");
    let contents = "#define AVAILABILITY_FILE_INCLUDED 1\n";
    std::fs::write(&first, contents).unwrap();
    // Identical bytes ensure symlink replacement cannot be detected merely by
    // hashing its requested spelling; its physical identity must be recorded.
    std::fs::write(&second, contents).unwrap();
    std::os::unix::fs::symlink(&first, &alias).unwrap();
    std::fs::write(
        &wrapper,
        "#if __has_include(\"available.h\")\n#define AVAILABLE_FUNCTION(value) ((value) + 3)\n#endif\n",
    )
    .unwrap();
    let scanner = MacroScanner::new().unwrap();
    let frontend = inspect(&scanner, &wrapper, &[], None).unwrap();
    let identity = first.canonicalize().unwrap();
    let inputs = &frontend.profile().inputs;
    assert!(inputs.files.contains(&alias));
    assert!(inputs.files.contains(&identity));
    assert!(inputs.fingerprints[&alias].is_some());
    assert_eq!(inputs.fingerprints[&alias], inputs.fingerprints[&identity]);
    assert!(frontend.environment().active.contains_key("AVAILABLE_FUNCTION"));
    assert!(
        !frontend.environment().active.contains_key("AVAILABILITY_FILE_INCLUDED"),
        "availability lookup must not include the queried file"
    );
    assert!(
        frontend
            .inventory()
            .macros
            .iter()
            .all(|definition| definition.name != "AVAILABILITY_FILE_INCLUDED")
    );
    let names = ["AVAILABLE_FUNCTION"];
    AnalysisSession::prepare(&scanner, &frontend, &names)
        .expect("an unchanged availability-only symlink must permit original-header probes");

    std::fs::remove_file(&alias).unwrap();
    std::os::unix::fs::symlink(&second, &alias).unwrap();
    let replacement = AnalysisSession::prepare(&scanner, &frontend, &names)
        .err()
        .expect("a symlink target change must invalidate the inspected environment")
        .to_string();
    assert!(replacement.contains("new header dependency appeared"), "{replacement}");

    std::fs::remove_file(&alias).unwrap();
    std::os::unix::fs::symlink(&first, &alias).unwrap();
    std::fs::write(&first, "#define AVAILABILITY_FILE_INCLUDED 2\n").unwrap();
    let mutation = AnalysisSession::prepare(&scanner, &frontend, &names)
        .err()
        .expect("mutation of an availability-only dependency must invalidate inspection")
        .to_string();
    assert!(mutation.contains("changed after inspection"), "{mutation}");
}
