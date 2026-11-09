//LICENSE Portions Copyright 2026 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE All rights reserved.
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.

//! The workspace's packages, as `cargo metadata` describes them.

use cargo_metadata::camino::{Utf8Path, Utf8PathBuf};
use cargo_metadata::{Metadata, MetadataCommand, Package, PackageId, TargetKind};
use eyre::WrapErr;

pub(crate) struct Workspace {
    metadata: Metadata,
}

impl Workspace {
    pub(crate) fn load() -> eyre::Result<Self> {
        let metadata = MetadataCommand::new()
            .no_deps()
            .other_options(vec!["--offline".to_owned()])
            .exec()
            .wrap_err("`cargo metadata` failed")?;
        Ok(Self { metadata })
    }

    pub(crate) fn root(&self) -> &Utf8Path {
        &self.metadata.workspace_root
    }

    pub(crate) fn packages(&self) -> &[Package] {
        &self.metadata.packages
    }

    pub(crate) fn package(&self, name: &str) -> Option<&Package> {
        self.packages().iter().find(|package| package.name.as_str() == name)
    }

    pub(crate) fn by_id(&self, id: &PackageId) -> Option<&Package> {
        self.packages().iter().find(|package| &package.id == id)
    }

    /// A target directory beside the default one, for builds that must not share it.
    pub(crate) fn target_dir(&self, name: &str) -> Utf8PathBuf {
        self.root().join(name)
    }
}

/// Whether a package builds against one PostgreSQL version, which a `pgNN` feature selects.
pub(crate) fn needs_postgres(package: &Package) -> bool {
    package.features.keys().any(|feature| {
        feature.strip_prefix("pg").is_some_and(|version| {
            !version.is_empty() && version.bytes().all(|byte| byte.is_ascii_digit())
        })
    })
}

/// Whether a package has a library target that doctests can link.
pub(crate) fn has_rust_library(package: &Package) -> bool {
    package.targets.iter().any(|target| {
        target.kind.iter().any(|kind| matches!(kind, TargetKind::Lib | TargetKind::RLib))
    })
}

/// The directory that holds a package's manifest.
pub(crate) fn package_dir(package: &Package) -> &Utf8Path {
    package.manifest_path.parent().expect("a manifest path names a file in a directory")
}
