//LICENSE Portions Copyright 2026 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.

#![cfg(unix)]

use pgrx_c_macros::{ActiveProvenance, AnalysisSession, MacroScanner, inspect};
use std::path::PathBuf;
use std::sync::Mutex;

static SCANNER_LOCK: Mutex<()> = Mutex::new(());

struct Directory(PathBuf);

impl Directory {
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

impl Drop for Directory {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

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
