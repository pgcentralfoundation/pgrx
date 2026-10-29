//LICENSE Portions Copyright 2026 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.

// Write synthetic tools that tests run in place of a compiler, archiver, formatter or pg_config.

use std::path::Path;
use std::process::Command;

/// Write `contents` to `path` from a child shell and make it executable by its owner.
///
/// Tests spawn processes on parallel threads. A child forked while this process has a file open
/// for writing keeps that descriptor until it execs, and running the file meanwhile fails with
/// ETXTBSY ("Text file busy") on Linux. The shell's descriptor never exists in this process, so
/// no concurrently forked child can inherit it.
pub fn write_script(path: &Path, contents: &str) {
    let status = Command::new("/bin/sh")
        .args(["-c", r#"printf '%s' "$2" > "$1" && chmod 700 "$1""#, "sh"])
        .arg(path)
        .arg(contents)
        .status()
        .expect("start /bin/sh to write a test script");
    assert!(status.success(), "could not write test script {}", path.display());
}
