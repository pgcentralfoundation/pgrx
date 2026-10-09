//LICENSE Portions Copyright 2026 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.

//! Verify input bytes without making downstream unoptimized build scripts repeatedly hash compiler
//! images through unoptimized Rust compression loops. Supported Linux/macOS generator hosts use
//! ring's assembly-backed SHA-256; other targets retain the portable SHA2 implementation. Both
//! produce the same content identities and re-read every physical file on every verification pass.
//! Aliases share observations only within a pass, never through metadata-only cross-pass caching.

use super::FrontendError;
use std::collections::BTreeMap;
use std::io::{self, Read};
use std::path::PathBuf;

/// Own one streaming SHA-256 computation using the supported host's assembly backend.
#[cfg(all(
    any(target_os = "linux", target_os = "macos"),
    any(target_arch = "x86_64", target_arch = "aarch64")
))]
struct FileDigest(
    /// Native digest state whose compression core remains fast in unoptimized downstream builds.
    ring::digest::Context,
);

/// Keep unsupported generator hosts independent of a native digest build toolchain.
#[cfg(not(all(
    any(target_os = "linux", target_os = "macos"),
    any(target_arch = "x86_64", target_arch = "aarch64")
)))]
struct FileDigest(
    /// Portable digest state for generator hosts without the selected assembly backend.
    sha2::Sha256,
);

/// Present the same streaming content contract through the assembly backend.
#[cfg(all(
    any(target_os = "linux", target_os = "macos"),
    any(target_arch = "x86_64", target_arch = "aarch64")
))]
impl FileDigest {
    /// Begin an independently computed digest for one physical input in this pass.
    fn new() -> Self {
        Self(ring::digest::Context::new(&ring::digest::SHA256))
    }
    /// Include every successfully read input byte in its content identity.
    fn update(&mut self, bytes: &[u8]) {
        self.0.update(bytes);
    }
    /// Preserve the report's existing SHA-256 hexadecimal fingerprint format.
    fn finish(self) -> String {
        hexadecimal(self.0.finish().as_ref())
    }
}

/// Preserve the same content contract on hosts without the selected assembly backend.
#[cfg(not(all(
    any(target_os = "linux", target_os = "macos"),
    any(target_arch = "x86_64", target_arch = "aarch64")
)))]
impl FileDigest {
    /// Begin a portable digest without requiring a native C compiler for this host.
    fn new() -> Self {
        use sha2::Digest;
        Self(sha2::Sha256::new())
    }
    /// Include every successfully read input byte in its content identity.
    fn update(&mut self, bytes: &[u8]) {
        use sha2::Digest;
        self.0.update(bytes);
    }
    /// Keep portable and native fingerprint representations identical.
    fn finish(self) -> String {
        use sha2::Digest;
        hexadecimal(self.0.finalize().as_ref())
    }
}

/// Encode a digest without relying on backend-specific formatting trait implementations.
fn hexadecimal(bytes: &[u8]) -> String {
    use std::fmt::Write as _;
    let mut fingerprint = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        write!(&mut fingerprint, "{byte:02x}").expect("String output");
    }
    fingerprint
}

/// Snapshot file bytes, following each requested spelling so symlink replacement is observed.
pub(crate) fn fingerprint_files<'a>(
    files: impl IntoIterator<Item = &'a PathBuf>,
) -> Result<BTreeMap<PathBuf, Option<String>>, FrontendError> {
    let mut fingerprints = BTreeMap::new();
    // The spelled executable and its canonical target often name the same large
    // binary. Share only this pass's content observation; every later verification
    // still re-reads bytes, including same-size changes with restored mtimes.
    let mut identities = BTreeMap::<PathBuf, String>::new();
    let mut buffer = [0u8; 65536];
    for path in files {
        let identity = match path.canonicalize() {
            Ok(identity) => identity,
            Err(error) if error.kind() == io::ErrorKind::NotFound => {
                fingerprints.insert(path.clone(), None);
                continue;
            }
            Err(source) => {
                return Err(FrontendError::CompilerIo { compiler: path.clone(), source });
            }
        };
        if let Some(fingerprint) = identities.get(&identity) {
            fingerprints.insert(path.clone(), Some(fingerprint.clone()));
            continue;
        }
        // Open the identity used as the cache key. A spelling retargeted between
        // canonicalization and open must not cache another target's bytes under
        // this identity; the surrounding checks verify spellings independently.
        let mut file = match std::fs::File::open(&identity) {
            Ok(file) => file,
            Err(error) if error.kind() == io::ErrorKind::NotFound => {
                fingerprints.insert(path.clone(), None);
                continue;
            }
            Err(source) => {
                return Err(FrontendError::CompilerIo { compiler: path.clone(), source });
            }
        };
        let mut digest = FileDigest::new();
        loop {
            let length = file
                .read(&mut buffer)
                .map_err(|source| FrontendError::CompilerIo { compiler: path.clone(), source })?;
            if length == 0 {
                break;
            }
            digest.update(&buffer[..length]);
        }
        let fingerprint = digest.finish();
        identities.insert(identity, fingerprint.clone());
        fingerprints.insert(path.clone(), Some(fingerprint));
    }
    Ok(fingerprints)
}

/// Capture spelled input targets separately from their bytes so identical-content symlink
/// replacements cannot reuse provenance or compiler-selection observations from an older target.
pub(crate) fn file_identities<'a>(
    files: impl IntoIterator<Item = &'a PathBuf>,
) -> Result<BTreeMap<PathBuf, Option<PathBuf>>, FrontendError> {
    files
        .into_iter()
        .map(|path| {
            let identity = match path.canonicalize() {
                Ok(identity) => Some(identity),
                Err(error) if error.kind() == io::ErrorKind::NotFound => None,
                Err(source) => {
                    return Err(FrontendError::CompilerIo { compiler: path.clone(), source });
                }
            };
            Ok((path.clone(), identity))
        })
        .collect()
}

/// Preserve report digest compatibility across backend choices and streaming boundaries.
#[cfg(test)]
mod tests {
    use super::*;
    use sha2::Digest;

    /// The fast backend must produce standard SHA-256 vectors and agree with the independently
    /// implemented portable backend even when input updates cross its block and file-buffer sizes.
    #[test]
    fn digest_backends_preserve_sha256_identity_across_streaming_boundaries() {
        let mut digest = FileDigest::new();
        digest.update(b"abc");
        assert_eq!(
            digest.finish(),
            "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"
        );
        for length in [0, 1, 63, 64, 65, 65536, 65537] {
            let bytes = (0..length).map(|index| (index % 251) as u8).collect::<Vec<_>>();
            let mut digest = FileDigest::new();
            for chunk in bytes.chunks(63) {
                digest.update(chunk);
            }
            assert_eq!(digest.finish(), hexadecimal(sha2::Sha256::digest(&bytes).as_ref()));
        }
    }
}
