//LICENSE Portions Copyright 2026 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.

#![cfg(unix)]

use pgrx_c_macros::{ActiveProvenance, MacroScanner, inspect};
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
