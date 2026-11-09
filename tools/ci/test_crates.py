#!/usr/bin/env python3
"""Test the workspace packages that don't need PostgreSQL.

nextest runs each test in its own process, so tests that serialize libclang within one
process run in parallel, and so do tests from different binaries. Doctests run afterwards
with the same package selection, which reuses the libraries nextest built.

Usage:
    test_crates.py PACKAGE...
"""

from __future__ import annotations

import subprocess
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).parent))
import parallel_tests as pt  # noqa: E402


def main() -> int:
    packages = sys.argv[1:]
    if not packages:
        return 0
    selection = [arg for name in packages for arg in ("-p", name)]
    status = 0
    commands = [["cargo", "nextest", "run", "--locked", "--no-fail-fast", "--no-tests=pass", *selection]]

    # `cargo test --doc` refuses a selection without any library, such as cargo-pgrx alone.
    meta = {package["name"]: package for package in pt.metadata()["packages"]}
    if any("lib" in target["kind"] for name in packages for target in meta[name]["targets"]):
        commands.append(["cargo", "test", "--locked", "--doc", *selection])

    for command in commands:
        pt.log(f"+ {' '.join(command)}")
        status |= subprocess.run(command).returncode
    return 1 if status else 0


if __name__ == "__main__":
    sys.exit(main())
