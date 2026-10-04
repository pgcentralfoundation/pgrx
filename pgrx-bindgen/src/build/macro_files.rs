//LICENSE Portions Copyright 2026 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.

//! Organize emitted PostgreSQL macros into readable modules without changing scope.
//!
//! Header provenance determines the public module tree. Shared adapters and the
//! private native module are split only at complete Rust item boundaries, with
//! include files retaining their original lexical scope. This produces a complete
//! relative-path map for the build writer, which formats and publishes the tree
//! and removes obsolete leaves. Names are encoded when natural header names would
//! collide with indexes, directories, keywords, or reserved support files.

use eyre::{WrapErr, eyre};
use pgrx_c_macros::{EmissionStatus, MacroEmission};
use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Write;
use std::path::{Component, Path, PathBuf};
use syn::{Item, spanned::Spanned};

/// Reserved generated subtree holding same-scope support fragments, separated from public
/// header modules.
const SUPPORT_DIRECTORY: &str = "__pgrx_c_support";
/// Soft support-file size bound; complete items may exceed it rather than be split into invalid
/// or differently scoped Rust.
const FRAGMENT_BYTES: usize = 256 * 1024;

/// Paths are relative to the owning `cmacros/pgNN` directory. The caller writes
/// this complete set and removes obsolete generated files in that directory.
pub(super) struct MacroFiles {
    /// Complete relative Rust source map; stable writing and stale-file pruning share this
    /// ownership boundary.
    pub(super) sources: BTreeMap<PathBuf, String>,
}

/// Build the complete versioned output map used for both host and documentation publication.
impl MacroFiles {
    /// Keep availability queries usable when C inspection is unavailable, without publishing
    /// C definitions, adapters, or any guessed compiler facts.
    pub(super) fn empty() -> Self {
        Self {
            sources: BTreeMap::from([(
                PathBuf::from("mod.rs"),
                pgrx_c_macros::emit_unavailable_macro_support(),
            )]),
        }
    }

    /// Partition adapter support and group emitted macros by verified header provenance,
    /// rejecting external roots and duplicate public identifiers.
    pub(super) fn new(
        support: String,
        emissions: &[MacroEmission],
        header_root: &Path,
    ) -> eyre::Result<Self> {
        let mut files = BTreeMap::new();
        let shared = partition_support(&support, &mut files)?;
        let mut tree = Directory::default();
        let mut root = None;
        let mut resolved = BTreeMap::<PathBuf, PathBuf>::new();
        let mut public_names = BTreeSet::new();
        let mut ordered = emissions.iter().collect::<Vec<_>>();
        ordered.sort_unstable_by(|left, right| left.analysis.name.cmp(&right.analysis.name));
        for emission in ordered {
            let EmissionStatus::Emitted { rust, .. } = &emission.status else { continue };
            let provenance = emission.analysis.provenance.as_ref().ok_or_else(|| {
                eyre!("emitted macro {} has no defining header", emission.analysis.name)
            })?;
            if root.is_none() {
                root = Some(header_root.canonicalize().wrap_err_with(|| {
                    format!("could not resolve PostgreSQL header root {}", header_root.display())
                })?);
            }
            let root = root.as_ref().expect("initialized header root");
            if !resolved.contains_key(&provenance.file) {
                let canonical = provenance.file.canonicalize().wrap_err_with(|| {
                    format!("could not resolve macro header {}", provenance.file.display())
                })?;
                let relative = canonical.strip_prefix(root).wrap_err_with(|| {
                    format!(
                        "macro {} comes from outside the PostgreSQL header root",
                        emission.analysis.name
                    )
                })?;
                resolved.insert(provenance.file.clone(), relative.to_owned());
            }
            let source = resolved.get(&provenance.file).expect("resolved source path");
            let public = public_identifier(rust).wrap_err_with(|| {
                format!("could not identify generated macro {}", emission.analysis.name)
            })?;
            if !public_names.insert(public.clone()) {
                return Err(eyre!("generated macro identifier {public} occurs more than once"));
            }
            let header = tree.header_mut(source)?;
            header.source.push_str(rust);
            header.source.push('\n');
            header.exports.insert(public);
        }
        tree.write(&PathBuf::new(), Some(&shared), &mut files)?;
        Ok(Self { sources: files })
    }
}

/// Read the final exported macro's actual Rust identifier so reexports honor keyword escaping
/// rather than guessing from the C name.
fn public_identifier(source: &str) -> eyre::Result<String> {
    let file = syn::parse_file(source).wrap_err("generated macro source is invalid Rust")?;
    let item = file
        .items
        .iter()
        .rev()
        .find_map(|item| match item {
            Item::Macro(item) if item.mac.path.is_ident("macro_rules") => Some(item),
            _ => None,
        })
        .filter(|item| item.attrs.iter().any(|attribute| attribute.path().is_ident("macro_export")))
        .ok_or_else(|| eyre!("generated macro source has no exported final definition"))?;
    item.ident.as_ref().map(ToString::to_string).ok_or_else(|| eyre!("exported macro has no name"))
}

/// Retain the physical header directory tree so provenance determines readable, collision-free
/// Rust modules.
#[derive(Default)]
struct Directory {
    /// Child header directories, preserving physical provenance instead of flattening
    /// basenames.
    directories: BTreeMap<String, Directory>,
    /// Macro source and exported names grouped by the original header filename.
    headers: BTreeMap<String, Header>,
}

/// Accumulate one header's translated definitions and their public Rust export names.
#[derive(Default)]
struct Header {
    /// Translated definitions belonging to this header, kept in deterministic name order.
    source: String,
    /// Actual emitted Rust identifiers to forward from the header leaf.
    exports: BTreeSet<String>,
}

/// Resolve header provenance into one collision-checked module tree and render its public
/// forwarding paths.
impl Directory {
    /// Find or create the header leaf using normal relative components, keeping equal basenames
    /// in separate directories.
    fn header_mut(&mut self, source: &Path) -> eyre::Result<&mut Header> {
        let components = source
            .components()
            .map(|component| match component {
                Component::Normal(name) => name
                    .to_str()
                    .map(str::to_owned)
                    .ok_or_else(|| eyre!("macro header path is not UTF-8")),
                _ => Err(eyre!("macro header path is not relative to its PostgreSQL root")),
            })
            .collect::<eyre::Result<Vec<_>>>()?;
        let (name, parents) =
            components.split_last().ok_or_else(|| eyre!("macro header has no filename"))?;
        let mut directory = self;
        for parent in parents {
            directory = directory.directories.entry(parent.clone()).or_default();
        }
        Ok(directory.headers.entry(name.clone()).or_default())
    }

    /// Render directory indexes, header leaves, and reexports while encoding names that collide
    /// with modules or reserved support paths.
    fn write(
        &self,
        path: &Path,
        support: Option<&[PathBuf]>,
        files: &mut BTreeMap<PathBuf, String>,
    ) -> eyre::Result<()> {
        let mut index = String::new();
        if let Some(support) = support {
            for fragment in support {
                writeln!(index, "include!({:?});", source_path(fragment)?).expect("String output");
            }
        }
        let directory_names = self
            .directories
            .keys()
            .map(|name| module_identifier(name, "directory"))
            .collect::<BTreeSet<_>>();
        let mut stems = BTreeMap::<String, usize>::new();
        for name in self.headers.keys() {
            *stems.entry(header_stem(name)?.to_owned()).or_default() += 1;
        }
        for (name, directory) in &self.directories {
            let identifier = module_identifier(name, "directory");
            let filename = if name.starts_with("__pgrx_c_") || name == "mod.rs" {
                encoded_name(name, "directory")
            } else {
                name.clone()
            };
            check_component(&filename)?;
            directory.write(&path.join(&filename), None, files)?;
            write_module(&mut index, &identifier, &format!("{filename}/mod.rs"));
        }
        for (name, header) in &self.headers {
            let stem = header_stem(name)?;
            let natural_identifier = module_identifier(stem, "header");
            let identifier = if directory_names.contains(&natural_identifier) || stems[stem] > 1 {
                encoded_name(name, "header")
            } else {
                natural_identifier
            };
            let filename = if stem == "mod"
                || stem.starts_with("__pgrx_c_")
                || stems[stem] > 1
                || self.directories.contains_key(&format!("{stem}.rs"))
            {
                format!("{}.rs", encoded_name(name, "header"))
            } else {
                format!("{stem}.rs")
            };
            check_component(&filename)?;
            let mut source = String::new();
            let comment_name = name.replace(['\n', '\r'], " ");
            writeln!(source, "// C macros from {comment_name}.\n").expect("String output");
            source.push_str(&header.source);
            for export in &header.exports {
                // Absolute paths to macro_export definitions made by include!
                // trigger rustc's macro-expanded absolute-path lint. Reexport
                // the colocated definition and let indexes forward that import.
                writeln!(source, "pub use {export};").expect("String output");
            }
            insert_file(files, path.join(&filename), source)?;
            write_module(&mut index, &identifier, &filename);
        }
        insert_file(files, path.join("mod.rs"), index)
    }
}

/// Add a private module declaration and public forwarding import without using absolute paths
/// to included macro_export definitions.
fn write_module(index: &mut String, identifier: &str, filename: &str) {
    writeln!(
        index,
        "#[path = {filename:?}]\nmod {identifier};\n#[allow(unused_imports)]\npub use {identifier}::*;"
    )
    .expect("String output");
}

/// Extract a usable header filename stem for human-readable generated modules.
fn header_stem(filename: &str) -> eyre::Result<&str> {
    Path::new(filename)
        .file_stem()
        .and_then(|stem| stem.to_str())
        .filter(|stem| !stem.is_empty())
        .ok_or_else(|| eyre!("macro header filename has no usable stem"))
}

/// Preserve readable names where legal and encode reserved or invalid identifiers so the module
/// tree stays collision-free.
fn module_identifier(name: &str, category: &str) -> String {
    if name.starts_with("__pgrx_c_")
        || matches!(name, "_" | "self" | "Self" | "super" | "crate")
        || !name.bytes().enumerate().all(|(index, byte)| {
            byte.is_ascii_alphabetic() || byte == b'_' || (index != 0 && byte.is_ascii_digit())
        })
    {
        return encoded_name(name, category);
    }
    if syn::parse_str::<syn::Ident>(name).is_ok() { name.to_owned() } else { format!("r#{name}") }
}

/// Encode original UTF-8 bytes into a reserved Rust identifier, preserving distinct names
/// without lossy sanitization.
fn encoded_name(name: &str, category: &str) -> String {
    let mut encoded = format!("__pgrx_c_{category}_");
    for byte in name.bytes() {
        write!(encoded, "{byte:02x}").expect("String output");
    }
    encoded
}

/// Reject generated filesystem components exceeding the supported byte limit before any output
/// is published.
fn check_component(component: &str) -> eyre::Result<()> {
    if component.len() > 255 {
        return Err(eyre!("generated module filename exceeds the filesystem component limit"));
    }
    Ok(())
}

/// Insert one generated source path and reject collisions instead of silently replacing a
/// previous leaf.
fn insert_file(
    files: &mut BTreeMap<PathBuf, String>,
    path: PathBuf,
    source: String,
) -> eyre::Result<()> {
    if files.insert(path.clone(), source).is_some() {
        return Err(eyre!("generated C macro files collide at {}", path.display()));
    }
    Ok(())
}

/// Render a UTF-8 relative include path with stable separators and reject escaping or absolute
/// components.
fn source_path(path: &Path) -> eyre::Result<String> {
    path.components()
        .map(|component| match component {
            Component::Normal(name) => name
                .to_str()
                .map(str::to_owned)
                .ok_or_else(|| eyre!("generated module path is not UTF-8")),
            _ => Err(eyre!("generated module path must be relative")),
        })
        .collect::<eyre::Result<Vec<_>>>()
        .map(|components| components.join("/"))
}

/// Offsets are indexed once, so fragment boundaries do not repeatedly scan the
/// entire (often tens of megabytes) support source.
struct SourceOffsets<'a> {
    /// Original support source whose exact bytes must survive partitioning.
    source: &'a str,
    /// Byte offset of each source line, built once for all item boundaries.
    lines: Vec<usize>,
    /// Per-line character-to-byte corrections only for non-ASCII spans.
    unicode: BTreeMap<usize, Vec<(usize, usize)>>,
}

/// Translate compiler token positions to exact original-source byte boundaries in one indexed
/// pass.
impl<'a> SourceOffsets<'a> {
    /// Partition adapter support and group emitted macros by verified header provenance,
    /// rejecting external roots and duplicate public identifiers.
    fn new(source: &'a str) -> Self {
        let mut lines = vec![0];
        let mut unicode = BTreeMap::new();
        let mut offset = 0;
        for (index, line) in source.split_inclusive('\n').enumerate() {
            if !line.is_ascii() {
                let mut extra = 0;
                let mut changes = Vec::new();
                for (column, character) in line.chars().enumerate() {
                    if !character.is_ascii() {
                        extra += character.len_utf8() - 1;
                        changes.push((column + 1, extra));
                    }
                }
                unicode.insert(index, changes);
            }
            offset += line.len();
            if line.ends_with('\n') {
                lines.push(offset);
            }
        }
        Self { source, lines, unicode }
    }

    /// Translate a proc_macro2 character span into a checked byte offset, including corrections
    /// on non-ASCII lines.
    fn position(&self, position: proc_macro2::LineColumn) -> eyre::Result<usize> {
        let line = position
            .line
            .checked_sub(1)
            .ok_or_else(|| eyre!("generated support span has no source line"))?;
        // proc_macro2 columns count Unicode characters, while Rust strings are
        // indexed in bytes. Most generated lines are ASCII; only exceptional
        // lines need this small character-to-byte correction table.
        let extra = self.unicode.get(&line).map_or(0, |changes| {
            let end = changes.partition_point(|(column, _)| *column <= position.column);
            end.checked_sub(1).map_or(0, |index| changes[index].1)
        });
        let offset = self
            .lines
            .get(line)
            .and_then(|start| start.checked_add(position.column)?.checked_add(extra))
            .filter(|offset| *offset <= self.source.len() && self.source.is_char_boundary(*offset))
            .ok_or_else(|| eyre!("generated support span is outside its source"))?;
        Ok(offset)
    }
}

/// Accumulate complete support items into bounded include files without changing their
/// enclosing lexical scope.
struct Fragments<'a> {
    /// Fragment family used for stable shared or native output filenames.
    label: &'a str,
    /// Complete output map receiving each flushed leaf.
    files: &'a mut BTreeMap<PathBuf, String>,
    /// Whole items accumulated since the previous flush.
    current: String,
    /// Ordered include paths preserving the original support declaration sequence.
    paths: Vec<PathBuf>,
}

/// Accumulate whole support items and publish ordered include leaves without changing lexical
/// scope.
impl<'a> Fragments<'a> {
    /// Partition adapter support and group emitted macros by verified header provenance,
    /// rejecting external roots and duplicate public identifiers.
    fn new(label: &'a str, files: &'a mut BTreeMap<PathBuf, String>) -> Self {
        Self { label, files, current: String::new(), paths: Vec::new() }
    }

    /// Append a complete Rust item, flushing at the soft fragment bound without splitting
    /// tokens or lexical scopes.
    fn push(&mut self, item: &str) -> eyre::Result<()> {
        if !self.current.is_empty()
            && self.current.len().saturating_add(item.len()) > FRAGMENT_BYTES
        {
            self.flush()?;
        }
        // A complete item may exceed the soft bound. Splitting token bodies or
        // impl blocks would change scope or make the fragments invalid Rust.
        self.current.push_str(item);
        Ok(())
    }

    /// Publish the current complete fragment into the source map and retain its ordered include
    /// path.
    fn flush(&mut self) -> eyre::Result<()> {
        if self.current.is_empty() {
            return Ok(());
        }
        let path = PathBuf::from(SUPPORT_DIRECTORY).join(format!(
            "{}_{:04}.rs",
            self.label,
            self.paths.len()
        ));
        let mut source = String::from("// Shared generated C macro support.\n");
        source.push_str(&std::mem::take(&mut self.current));
        source.push('\n');
        insert_file(self.files, path.clone(), source)?;
        self.paths.push(path);
        Ok(())
    }

    /// Flush the final fragment and return include paths in their original semantic order.
    fn finish(mut self) -> eyre::Result<Vec<PathBuf>> {
        self.flush()?;
        Ok(self.paths)
    }
}

/// Split shared and private-native support at parsed item boundaries, preserving comments and
/// same-scope includes while guarding native adapters for docs.rs.
fn partition_support(
    source: &str,
    files: &mut BTreeMap<PathBuf, String>,
) -> eyre::Result<Vec<PathBuf>> {
    if source.is_empty() {
        return Ok(Vec::new());
    }
    let parsed = syn::parse_file(source).wrap_err("generated C macro support is invalid Rust")?;
    if !parsed.attrs.is_empty() {
        return Err(eyre!("generated C macro support must not contain inner attributes"));
    }
    let offsets = SourceOffsets::new(source);
    let mut pieces = Vec::with_capacity(parsed.items.len());
    let mut cursor = 0;
    let mut native_seen = false;
    for item in &parsed.items {
        let end = offsets.position(item.span().end())?;
        if let Item::Mod(module) = item
            && module.ident == "__pgrx_c_generated"
        {
            if native_seen {
                return Err(eyre!("generated native support module occurs more than once"));
            }
            native_seen = true;
            let (brace, body) = module
                .content
                .as_ref()
                .ok_or_else(|| eyre!("native support module must have an inline body"))?;
            if module
                .attrs
                .iter()
                .any(|attribute| matches!(attribute.style, syn::AttrStyle::Inner(_)))
            {
                return Err(eyre!("native support module must not contain inner attributes"));
            }
            let start = offsets.position(item.span().start())?;
            let opening = offsets.position(brace.span.open().end())?;
            let closing = offsets.position(brace.span.close().start())?;
            let mut native = Fragments::new("native", files);
            let mut previous = opening;
            for item in body {
                let item_end = offsets.position(item.span().end())?;
                native.push(&source[previous..item_end])?;
                previous = item_end;
            }
            native.push(&source[previous..closing])?;
            let fragments = native.finish()?;
            let mut shell = source[cursor..start].to_owned();
            shell.push_str("#[cfg(not(docsrs))]\n");
            shell.push_str(&source[start..opening]);
            shell.push('\n');
            for fragment in fragments {
                let filename = fragment
                    .file_name()
                    .and_then(|name| name.to_str())
                    .ok_or_else(|| eyre!("native support fragment has no UTF-8 filename"))?;
                writeln!(shell, "include!({filename:?});").expect("String output");
            }
            shell.push_str(&source[closing..end]);
            pieces.push(shell);
        } else {
            pieces.push(source[cursor..end].to_owned());
        }
        cursor = end;
    }
    let mut shared = Fragments::new("shared", files);
    for piece in pieces {
        shared.push(&piece)?;
    }
    shared.push(&source[cursor..])?;
    shared.finish()
}

/// Check catalog, publication, and invalidation invariants directly against the private
/// binding-build implementation.
///
/// The tests inspect real parsed output, generated leaf maps, and compiler dependency
/// directives. Temporary input trees make success and rejection conditions explicit without
/// relying on a configured server.
#[cfg(test)]
mod tests {

    use super::*;
    use pgrx_c_macros::{
        AnalysisStatus, ConstCapability, EvaluationContract, InvocationContract, MacroAnalysis,
        SignedOverflow, SourceSpan,
    };
    use std::fs;
    use std::process::Command;
    use std::sync::atomic::{AtomicU64, Ordering};

    /// Allocate process-local unique directory suffixes for concurrent isolated oracle runs.
    static NEXT_DIRECTORY: AtomicU64 = AtomicU64::new(0);

    /// Own an isolated consumer or header tree whose generated artifacts can be inspected
    /// without mutating the repository.
    struct Fixture(
        /// Owned fixture path used for isolated inputs and cleanup.
        PathBuf,
    );

    /// Construct and inspect owned fixtures without changing repository snapshots or sharing
    /// consumer build artifacts.
    impl Fixture {
        /// Partition adapter support and group emitted macros by verified header provenance,
        /// rejecting external roots and duplicate public identifiers.
        fn new() -> Self {
            let path = std::env::temp_dir().join(format!(
                "pgrx-macro-files-{}-{}",
                std::process::id(),
                NEXT_DIRECTORY.fetch_add(1, Ordering::Relaxed)
            ));
            fs::create_dir(&path).unwrap();
            Self(path)
        }

        /// Create a supported macro with physical fixture provenance for module-tree collision
        /// and export tests.
        fn emission(&self, header: &str, name: &str) -> MacroEmission {
            let path = self.0.join(header);
            fs::create_dir_all(path.parent().unwrap()).unwrap();
            fs::write(&path, "// source header\n").unwrap();
            MacroEmission {
                analysis: MacroAnalysis {
                    name: name.to_owned(),
                    provenance: Some(SourceSpan { file: path, start_line: 1, end_line: 1 }),
                    status: AnalysisStatus::Candidate,
                    parameters: Vec::new(),
                    expression: None,
                    invocation: InvocationContract::ParenthesizedScalarExpressions,
                    const_capability: ConstCapability::RuntimeOnly,
                    evaluation: EvaluationContract::default(),
                    dependencies: Vec::new(),
                    required_helpers: Vec::new(),
                    signed_overflow: SignedOverflow::Wrapping,
                },
                status: EmissionStatus::Emitted {
                    rust: format!(
                        "// C macro {name}\n// ```text\n// #define {name}(x) (x)\n// ```\n#[macro_export]\nmacro_rules! {name} {{ ($x:expr) => {{ /* PGRX: retained */ $x }}; }}\n"
                    ),
                    const_capability: ConstCapability::RuntimeOnly,
                },
            }
        }
    }

    /// Release only temporary artifacts owned by this fixture, including on failed compiler or
    /// assertion paths.
    impl Drop for Fixture {
        /// Remove only this fixture's owned temporary storage after the test or oracle
        /// completes.
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.0);
        }
    }

    /// Checks that an empty macro tree is renderable without resolving a PostgreSQL header
    /// root.
    #[test]
    fn empty_generation_does_not_require_a_postgres_installation() {
        let files = MacroFiles::new(String::new(), &[], Path::new("/does/not/exist")).unwrap();
        assert_eq!(files.sources.len(), 1);
        assert_eq!(files.sources[Path::new("mod.rs")], "");
        let unavailable = MacroFiles::empty();
        assert_eq!(unavailable.sources.len(), 1);
        syn::parse_file(&unavailable.sources[Path::new("mod.rs")]).unwrap();
    }

    /// Prove the actual unavailable tree exports queries at the crate root through nested
    /// inclusion, while a separate consumer never resolves unavailable item bodies.
    #[test]
    fn unavailable_tree_exports_availability_to_external_consumers() {
        let fixture = Fixture::new();
        let files = MacroFiles::empty();
        let index = fixture.0.join("mod.rs");
        fs::write(&index, &files.sources[Path::new("mod.rs")]).unwrap();
        let library = fixture.0.join("library.rs");
        fs::write(&library, format!("mod cmacros {{ mod pg18 {{ include!({index:?}); }} }}\n"))
            .unwrap();
        let metadata = fixture.0.join("libavailability.rmeta");
        let output = Command::new("rustc")
            .args([
                "--edition=2024",
                "--crate-type=rlib",
                "--crate-name=availability",
                "--emit=metadata",
                "-Dwarnings",
            ])
            .arg(&library)
            .arg("-o")
            .arg(&metadata)
            .output()
            .unwrap();
        assert!(output.status.success(), "{}", String::from_utf8_lossy(&output.stderr));
        let consumer = fixture.0.join("consumer.rs");
        fs::write(
            &consumer,
            "extern crate availability as pg;\npg::__pgrx_c_classify!(@if_available CHECK_FOR_INTERRUPTS { compile_error!(\"unavailable items escaped\"); missing::function(); });\npg::__pgrx_c_classify!(@if_available PageSetPrunable { compile_error!(\"unavailable items escaped\"); });\nfn main() {}\n",
        )
        .unwrap();
        let output = Command::new("rustc")
            .args(["--edition=2024", "--emit=metadata", "-Dwarnings", "--extern"])
            .arg(format!("availability={}", metadata.display()))
            .arg(&consumer)
            .arg("-o")
            .arg(fixture.0.join("consumer.rmeta"))
            .output()
            .unwrap();
        assert!(output.status.success(), "{}", String::from_utf8_lossy(&output.stderr));
    }

    /// Checks that equal header basenames remain in separate provenance-derived modules.
    #[test]
    fn header_paths_keep_duplicate_basenames_in_their_own_modules() {
        let fixture = Fixture::new();
        let emissions = [
            fixture.emission("access/example.h", "ACCESS_EXAMPLE"),
            fixture.emission("utils/example.h", "UTILS_EXAMPLE"),
        ];
        let files = MacroFiles::new(String::new(), &emissions, &fixture.0).unwrap();
        assert_eq!(files.sources.len(), 5);
        assert!(files.sources[Path::new("mod.rs")].contains("pub use access::*;"));
        assert!(files.sources[Path::new("access/mod.rs")].contains("pub use example::*;"));
        let source = &files.sources[Path::new("access/example.rs")];
        assert!(source.contains("// #define ACCESS_EXAMPLE(x) (x)"));
        assert!(source.contains("/* PGRX: retained */"));
        assert!(source.contains("pub use ACCESS_EXAMPLE;"));
        assert!(!source.contains("pub use crate::"));
    }

    /// Checks that indexes, support names, and header/directory collisions receive distinct
    /// Rust identifiers and output paths.
    #[test]
    fn reserved_index_names_and_header_directory_collisions_are_distinct() {
        let fixture = Fixture::new();
        let emissions = [
            fixture.emission("foo.h", "ROOT_FOO"),
            fixture.emission("foo/example.h", "NESTED_FOO"),
            fixture.emission("mod.h", "MOD_EXAMPLE"),
            fixture.emission("match.h", "MATCH_EXAMPLE"),
            fixture.emission("a-b.h", "DASH_EXAMPLE"),
            fixture.emission("a_b.h", "UNDERSCORE_EXAMPLE"),
            fixture.emission("shared.h", "FIRST_SHARED"),
            fixture.emission("shared.hpp", "SECOND_SHARED"),
            fixture.emission("__pgrx_c_support/extra.h", "RESERVED_DIRECTORY"),
            fixture.emission("mod.rs/extra.h", "RESERVED_INDEX_DIRECTORY"),
            fixture.emission("foo.rs/extra.h", "HEADER_FILENAME_DIRECTORY"),
        ];
        let files = MacroFiles::new(String::new(), &emissions, &fixture.0).unwrap();
        let index = &files.sources[Path::new("mod.rs")];
        syn::parse_file(index).unwrap();
        assert!(index.contains("mod foo;"));
        assert!(index.contains(&format!("mod {};", encoded_name("foo.h", "header"))));
        assert!(index.contains("mod r#match;"));
        assert!(index.contains("mod a_b;"));
        assert!(index.contains(&format!("mod {};", encoded_name("a-b", "header"))));
        assert!(!files.sources.contains_key(&PathBuf::from("foo.rs")));
        for header in ["foo.h", "mod.h", "shared.h", "shared.hpp"] {
            assert!(
                files
                    .sources
                    .contains_key(&PathBuf::from(format!("{}.rs", encoded_name(header, "header"))))
            );
        }
        assert!(!files.sources.contains_key(Path::new("__pgrx_c_support/mod.rs")));
    }

    /// Checks that missing, external, or unrepresentable provenance fails before publication.
    #[test]
    fn unsupported_provenance_never_becomes_a_generated_path() {
        let fixture = Fixture::new();
        let other = Fixture::new();
        let mut emission = other.emission("outside.h", "OUTSIDE");
        let error = MacroFiles::new(String::new(), &[emission.clone()], &fixture.0)
            .err()
            .unwrap()
            .to_string();
        assert!(error.contains("outside the PostgreSQL header root"));
        emission.analysis.provenance = None;
        assert!(
            MacroFiles::new(String::new(), &[emission], &fixture.0)
                .err()
                .unwrap()
                .to_string()
                .contains("no defining header")
        );
    }

    /// Checks that escaped Rust macro identifiers are forwarded from the actual emitted
    /// definition.
    #[test]
    fn public_reexports_use_the_emitted_rust_identifier_only() {
        let fixture = Fixture::new();
        let mut emission = fixture.emission("keyword.h", "self");
        let rust = "#[macro_export] macro_rules! __pgrx_c_args_self { () => {}; }\n#[macro_export] macro_rules! __pgrx_c_macro_73656c66 { () => {}; }\n";
        emission.status = EmissionStatus::Emitted {
            rust: rust.into(),
            const_capability: ConstCapability::RuntimeOnly,
        };
        let files = MacroFiles::new(String::new(), &[emission], &fixture.0).unwrap();
        let source = &files.sources[Path::new("keyword.rs")];
        assert!(source.contains("pub use __pgrx_c_macro_73656c66;"));
        assert!(!source.contains("pub use __pgrx_c_args_self;"));
        assert!(!source.contains("pub use self;"));
    }

    /// Checks character-to-byte span conversion without losing UTF-8 comments or splitting
    /// support items.
    #[test]
    fn unicode_columns_preserve_exact_support_item_boundaries() {
        let support = "// é\nconst FIRST: &str = \"é\"; #[doc(hidden)] pub mod __pgrx_c_generated { pub const VALUE: &str = \"λ\"; } // Ω\n";
        let files = MacroFiles::new(support.into(), &[], Path::new("/not/used")).unwrap();
        let shared = &files.sources[Path::new("__pgrx_c_support/shared_0000.rs")];
        let native = &files.sources[Path::new("__pgrx_c_support/native_0000.rs")];
        assert!(shared.contains("const FIRST: &str = \"é\";"));
        assert!(shared.contains("#[cfg(not(docsrs))]\n#[doc(hidden)] pub mod"));
        assert!(shared.contains("// Ω"));
        assert!(native.contains("pub const VALUE: &str = \"λ\";"));
        syn::parse_file(shared).unwrap();
        syn::parse_file(native).unwrap();
    }

    /// Checks that splitting support retains source comments, item order, and the original
    /// private module scope.
    #[test]
    fn support_fragments_preserve_comments_and_native_private_scope() {
        let block = "x".repeat(FRAGMENT_BYTES / 2);
        let mut support = String::from("// leading shared comment\n");
        for index in 0..3 {
            writeln!(support, "pub const ROOT_{index}: &str = {block:?};").unwrap();
        }
        support.push_str("#[doc(hidden)]\npub mod __pgrx_c_generated {\n// native leading comment\nunsafe extern \"C\" { fn private_native(); }\n");
        for index in 0..3 {
            writeln!(support, "pub const NATIVE_{index}: &str = {block:?};").unwrap();
        }
        support.push_str("pub unsafe fn caller() { /* PGRX: native explanation */ unsafe { private_native() }; }\n// native trailing comment\n}\n// trailing shared comment\n");
        let files = MacroFiles::new(support, &[], Path::new("/not/used")).unwrap();
        let shared = files
            .sources
            .iter()
            .filter(|(path, _)| path.file_name().unwrap().to_string_lossy().starts_with("shared_"))
            .map(|(_, source)| source.as_str())
            .collect::<Vec<_>>();
        let native = files
            .sources
            .iter()
            .filter(|(path, _)| path.file_name().unwrap().to_string_lossy().starts_with("native_"))
            .map(|(_, source)| source.as_str())
            .collect::<Vec<_>>();
        assert!(shared.len() > 1 && native.len() > 1);
        let shared = shared.join("\n");
        let native = native.join("\n");
        assert!(shared.contains("#[cfg(not(docsrs))]\n#[doc(hidden)]\npub mod __pgrx_c_generated"));
        assert!(shared.contains("include!(\"native_0000.rs\");"));
        for comment in [
            "// native leading comment",
            "// native trailing comment",
            "/* PGRX: native explanation */",
        ] {
            assert_eq!(native.matches(comment).count(), 1);
        }
        assert!(native.contains("fn private_native()"));
        assert!(native.contains("unsafe { private_native() }"));
        assert!(shared.contains("// leading shared comment"));
        assert!(shared.contains("// trailing shared comment"));
        for source in files.sources.values() {
            syn::parse_file(source).unwrap();
            assert!(source.len() <= FRAGMENT_BYTES + 256);
        }
    }

    /// Checks that duplicate public Rust macro names fail instead of silently overwriting a
    /// generated leaf.
    #[test]
    fn duplicate_public_identifiers_are_rejected_before_writing() {
        let fixture = Fixture::new();
        let emissions =
            [fixture.emission("one.h", "DUPLICATE"), fixture.emission("two.h", "DUPLICATE")];
        let error =
            MacroFiles::new(String::new(), &emissions, &fixture.0).err().unwrap().to_string();
        assert!(error.contains("identifier DUPLICATE occurs more than once"));
    }
}
