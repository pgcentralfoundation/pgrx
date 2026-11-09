#!/usr/bin/env python3
"""Choose the CI work that a change needs.

A change to a workspace package must test that package and every package that depends on it.
Cargo's graph supplies most edges. The extra edges below are dependencies that Cargo cannot
see, such as the pg_test harness running `cargo-pgrx` to install each extension.

Files outside the workspace packages either select everything (Cargo configuration, CI
scripts, unknown files) or nothing (documentation). Pushes select everything.

Usage:
    plan.py --all                 select everything
    plan.py --base REV --head REV select what the diff between the revisions needs

The result is written to $GITHUB_OUTPUT when it is set, and printed either way.
"""

from __future__ import annotations

import argparse
import json
import os
import subprocess
import sys
from pathlib import PurePosixPath

# A package's built artifact uses these packages at run time, so changing the dependency
# changes what the dependent's tests exercise. The pg_test harness builds and installs every
# extension with `cargo pgrx install`.
RUNTIME_EDGES = {
    "pgrx-tests": ["cargo-pgrx"],
}

# A package's tests, but not its artifact, use these packages. cargo-pgrx's install regression
# test builds pgrx-unit-tests, and pgrx-bindgen's harness tests build crates that use
# pgrx-macros.
TEST_EDGES = {
    "cargo-pgrx": ["pgrx-unit-tests"],
    "pgrx-bindgen": ["pgrx-macros"],
}

# Paths outside the packages that select everything. Directories end with "/".
SELECT_ALL = [
    ".cargo/",
    ".github/workflows/",
    "Cargo.lock",
    "Cargo.toml",
    "rust-toolchain.toml",
    "rustfmt.toml",
    "tools/",
    "update-versions.sh",
]

# Paths that no test reads.
SELECT_NONE = [
    ".github/FUNDING.yml",
    ".github/docker/",
    ".gitignore",
    "LICENSE",
    "art/",
    "articles/",
    "docs/",
    "prepare-release.sh",
    "publish.sh",
    "run-example.sh",
    "rust-analyzer.toml",
    "skills/",
    "upgrade-deps.sh",
]

# Paths that only the lint job reads.
SELECT_LINT = [
    "deny.toml",
]

# Each optional job runs when one of these packages needs testing.
JOB_SUBJECTS = {
    # Miri checks pointer provenance in pgrx-pg-sys's Datum and pgrx's fcinfo and varlena.
    "miri": ["pgrx-pg-sys", "pgrx"],
    # The extras jobs exercise these packages' less common configurations.
    "cmacros_clang14": ["pgrx-c-macros"],
    "bindgen_harness": ["pgrx-bindgen"],
    "ui": ["pgrx-unit-tests"],
    "bench": ["benching", "pgrx-bench"],
    "install_regression": ["cargo-pgrx"],
    "runas": ["arrays"],
    "init": ["cargo-pgrx"],
    "variants": ["pgrx-unit-tests", "custom_types"],
    "schema_release": ["pgrx-unit-tests"],
    "beta": None,
    "package": None,
}


def git(*args: str) -> str:
    return subprocess.check_output(["git", *args], text=True)


def cargo_metadata() -> dict:
    output = subprocess.check_output(
        ["cargo", "metadata", "--no-deps", "--format-version", "1", "--offline"], text=True
    )
    return json.loads(output)


def matches(path: str, patterns: list[str]) -> bool:
    return any(path.startswith(p) if p.endswith("/") else path == p for p in patterns)


def markdown(path: str) -> bool:
    # No code includes a Markdown file. Packaging copies the manifests' `readme` files, so
    # only the packaging job reads them.
    return path.endswith(".md")


class Workspace:
    def __init__(self, metadata: dict):
        root = PurePosixPath(metadata["workspace_root"])
        self.packages: dict[str, dict] = {}
        self.dirs: list[tuple[str, str]] = []
        for package in metadata["packages"]:
            name = package["name"]
            self.packages[name] = package
            directory = PurePosixPath(package["manifest_path"]).parent.relative_to(root)
            self.dirs.append((f"{directory}/", name))
        # Match the deepest package directory first, so tools/version-updater wins over tools/.
        self.dirs.sort(key=lambda entry: -len(entry[0]))

        names = set(self.packages)
        self.artifact_dependents: dict[str, set[str]] = {name: set() for name in names}
        self.test_dependents: dict[str, set[str]] = {name: set() for name in names}
        for name, package in self.packages.items():
            for dependency in package["dependencies"]:
                target = dependency["name"]
                if target not in names or dependency.get("path") is None:
                    continue
                if dependency["kind"] == "dev":
                    self.test_dependents[target].add(name)
                else:
                    self.artifact_dependents[target].add(name)
        for name, targets in RUNTIME_EDGES.items():
            for target in targets:
                self.artifact_dependents[target].add(name)
        for name, targets in TEST_EDGES.items():
            for target in targets:
                self.test_dependents[target].add(name)

    def owner(self, path: str) -> str | None:
        for directory, name in self.dirs:
            if path.startswith(directory):
                return name
        return None

    def affected(self, changed: set[str]) -> set[str]:
        """Return the packages whose tests a change to `changed` can alter."""
        artifacts = set()
        pending = list(changed)
        while pending:
            name = pending.pop()
            if name in artifacts:
                continue
            artifacts.add(name)
            pending.extend(self.artifact_dependents[name])
        # A dev-dependency or test-only edge reruns the dependent's tests, but the
        # dependent's own artifact is unchanged, so the walk stops there.
        tested = set(artifacts)
        for name in artifacts:
            tested |= self.test_dependents[name]
        return tested

    def needs_pg(self, name: str) -> bool:
        return "pg18" in self.packages[name]["features"]


def plan(workspace: Workspace, files: list[str] | None) -> dict:
    everything = files is None
    changed: set[str] = set()
    lint = everything
    readme = False
    reasons = []
    for path in files or []:
        owner = workspace.owner(path)
        if markdown(path):
            readme = True
        elif owner is not None:
            changed.add(owner)
            lint = True
        elif matches(path, SELECT_ALL):
            everything = True
            reasons.append(f"{path} selects everything")
        elif matches(path, SELECT_LINT):
            lint = True
        elif matches(path, SELECT_NONE):
            pass
        else:
            everything = True
            reasons.append(f"{path} is not mapped, so it selects everything")

    affected = set(workspace.packages) if everything else workspace.affected(changed)
    pg = sorted(name for name in affected if workspace.needs_pg(name))
    crates = sorted(name for name in affected if not workspace.needs_pg(name))
    jobs = {}
    for job, subjects in JOB_SUBJECTS.items():
        if subjects is None:
            jobs[job] = bool(affected)
        else:
            jobs[job] = any(subject in affected for subject in subjects)
    jobs["package"] = jobs["package"] or readme
    return {
        "everything": everything,
        "reasons": reasons,
        "changed": sorted(changed),
        "lint": lint or bool(affected),
        "pg": pg,
        "crates": crates,
        "jobs": jobs,
    }


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument("--all", action="store_true", help="select everything")
    parser.add_argument("--base", help="revision the change starts from")
    parser.add_argument("--head", help="revision the change ends at")
    args = parser.parse_args()

    if args.all:
        files = None
    elif args.base and args.head:
        files = git("diff", "--name-only", "--no-renames", args.base, args.head).split()
    else:
        parser.error("pass --all, or both --base and --head")

    result = plan(Workspace(cargo_metadata()), files)
    print(json.dumps(result, indent=2))

    output = os.environ.get("GITHUB_OUTPUT")
    if output:
        with open(output, "a", encoding="utf-8") as out:
            out.write(f"lint={json.dumps(result['lint'])}\n")
            out.write(f"pg={' '.join(result['pg'])}\n")
            out.write(f"crates={' '.join(result['crates'])}\n")
            for job, selected in result["jobs"].items():
                out.write(f"{job}={json.dumps(selected)}\n")
    return 0


if __name__ == "__main__":
    sys.exit(main())
