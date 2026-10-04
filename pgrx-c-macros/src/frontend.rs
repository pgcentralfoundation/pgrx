//LICENSE Portions Copyright 2026 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.

//! Establish the selected C compilation environment before macro analysis.

//! Discovery reconciles the Clang executable with the loaded libclang, including active
//! macros, included files, target facts, and declaration identities. Bounded compiler probes
//! establish facts that a declaration-only AST cannot supply, such as bitfield promotions and
//! inline definitions. Input fingerprints and recorded searches make later generation reject
//! a changed environment rather than silently mixing compiler observations.

use crate::{
    ActiveMacro, ActiveProvenance, BuildInputs, ByteOrder, CompilationProfile, CompilerIdentity,
    DeclarationCatalog, Error, FloatingKind, FloatingPointFacts, FloatingType, FrontendOutput,
    IntegerConstant, IntegerKind, IntegerType, IntegerValue, MacroDefinition, MacroDependencyGraph,
    MacroEnvironment, MacroInventory, MacroKind, MacroScanner, SignedOverflow, TargetFacts,
    TokenKind, TypeCategory, TypeInfo,
};
use clang::{EntityKind, EntityVisitResult, TranslationUnit, Type, TypeKind};
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};
use std::process::{Child, Command, ExitStatus, Stdio};
use std::sync::mpsc;
use std::thread;
use std::time::{Duration, Instant};

/// Probe implementation-specific field promotions and access units before admitting native bitfield
/// capabilities.
mod bitfields;
/// Prove supported compiler operations by prototype and bounded LLVM effect witnesses.
mod builtins;
/// Reparse original inline bodies to prove local definitions match the declaration-only catalog.
mod definitions;
/// Copy canonical C shape graphs while translation-unit handles remain live, preserving nominal
/// identity and layout.
mod types;
/// Prove source integer constant expressions are zero so null-pointer identity is never inferred from
/// runtime values.
pub(crate) mod zero_constants;

/// Bound each captured compiler pipe so pathological diagnostics or output cannot exhaust memory.
const OUTPUT_LIMIT: u64 = 16 * 1024 * 1024;
/// Bound one tool invocation so failed or stuck compiler phases release their owned child process.
const COMPILER_TIMEOUT: Duration = Duration::from_secs(60);
/// Compiler flags that consume a separate value and must be parsed without confusing it with another
/// input.
const VALUE_OPTIONS: &[&str] = &[
    "-I",
    "-isystem",
    "-iquote",
    "-idirafter",
    "-F",
    "-iframework",
    "-isysroot",
    "--sysroot",
    "-resource-dir",
    "-target",
    "--target",
    "-arch",
    "-D",
    "-U",
    "-include",
    "-imacros",
    "-std",
    "-B",
    "-ccc-gcc-name",
    "--gcc-toolchain",
    "-gcc-toolchain",
    "--gcc-install-dir",
];

/// Failure to establish an agreed compiler environment or obtain its declarations.
#[derive(Debug, thiserror::Error)]
pub enum FrontendError {
    /// The underlying scanner could not produce a complete reliable inventory.
    #[error(transparent)]
    Discovery(
        /// Original scanner failure preserved through the frontend/CLI boundary.
        #[from]
        Error,
    ),
    /// Compiler arguments could change or bypass the coherent inspected C phase.
    #[error("invalid Clang analysis arguments: {0}")]
    Arguments(
        /// The unsupported option or argument detail that prevented a reproducible compiler phase.
        String,
    ),
    /// The selected compiler could not be executed or its owned streams could not be read.
    #[error("could not run Clang at {}: {source}", compiler.display())]
    CompilerIo {
        /// The selected executable whose invocation failed or whose target facts are being recorded.
        compiler: PathBuf,
        /// The underlying I/O or parsing failure retained for actionable diagnostics.
        source: io::Error,
    },
    /// Generated native source or staged artifacts could not be accessed reliably.
    #[error("native support I/O at {}: {source}", path.display())]
    NativeIo {
        /// Native source or output path at which the filesystem operation failed.
        path: PathBuf,
        /// The underlying I/O or parsing failure retained for actionable diagnostics.
        source: io::Error,
    },
    /// A tool phase exited unsuccessfully, with status and diagnostics retained.
    #[error("Clang at {} failed ({status}): {diagnostics}", compiler.display())]
    CompilerFailed {
        /// The selected executable whose invocation failed or whose target facts are being recorded.
        compiler: PathBuf,
        /// Unsuccessful driver or archiver exit status retained alongside diagnostics.
        status: ExitStatus,
        /// Owned compiler diagnostics associated with this discovery or tool phase.
        diagnostics: String,
    },
    /// A compiler output pipe exceeded the bounded capture budget.
    #[error("Clang at {} exceeded the {OUTPUT_LIMIT}-byte output limit", .0.display())]
    OutputLimit(
        /// The tool executable whose captured output exceeded its byte budget.
        PathBuf,
    ),
    /// The tool invocation exceeded its finite time budget and is reaped.
    #[error("Clang at {} exceeded the 60-second time limit", .0.display())]
    Timeout(
        /// The tool executable whose invocation exceeded its time budget.
        PathBuf,
    ),
    /// No acceptable executable could be selected for the loaded libclang release.
    #[error("could not find a compatible Clang executable: {0}")]
    CompilerUnavailable(
        /// The missing or incompatible compiler selection detail.
        String,
    ),
    /// Compiler paths, inputs, or effective semantics disagreed with the recorded inspection.
    #[error("incompatible compiler environment: {0}")]
    Environment(
        /// The disagreement or changed-input detail that invalidates compiler fact coherence.
        String,
    ),
    /// Required compiler output was malformed, incomplete, or outside the bounded proof grammar.
    #[error("invalid Clang output: {0}")]
    Output(
        /// The malformed or incomplete compiler output detail.
        String,
    ),
}

/// Owned driver output from one bounded invocation; later phases interpret stdout facts and stderr
/// diagnostics separately.
pub(crate) struct CompilerOutput {
    /// Successful driver output consumed as preprocessing, dependency, or LLVM facts.
    pub(crate) stdout: String,
    /// Diagnostics and verbose driver configuration captured separately from phase fact output.
    pub(crate) stderr: String,
}

/// Inspect original headers under one verified compiler and libclang environment.
///
/// Clang's final macro dump supplies active definitions. Physical definitions from
/// libclang supply provenance; identical historical definitions remain ambiguous.
pub fn inspect(
    scanner: &MacroScanner,
    header: &Path,
    arguments: &[String],
    preferred_compiler: Option<&Path>,
) -> Result<FrontendOutput, FrontendError> {
    inspect_with_compiler_hint(scanner, header, arguments, preferred_compiler, None)
}

/// Resolve driver preference, reconcile driver/libclang observations, and probe facts before exposing
/// a frontend snapshot.
pub(crate) fn inspect_with_compiler_hint(
    scanner: &MacroScanner,
    header: &Path,
    arguments: &[String],
    preferred_compiler: Option<&Path>,
    configured_compiler: Option<&Path>,
) -> Result<FrontendOutput, FrontendError> {
    validate_arguments(arguments)?;
    let requested_header = input_path(header)?;
    // The supplied spelling determines quoted-include lookup, including through
    // symlink wrappers. Canonical paths are only identities for comparison/tracking.
    let header = requested_header.clone();
    let libclang_version = clang::get_version();
    let CompilerSelection { executable: compiler, search } =
        find_compiler(preferred_compiler, configured_compiler, &libclang_version)?;
    let version = run_compiler(&compiler, &["--version".into()])?.stdout;
    let version = version.lines().next().unwrap_or_default().to_owned();
    if version_major(&version) != version_major(&libclang_version) {
        return Err(FrontendError::Environment(format!(
            "{} is {version}; the loaded library is {libclang_version}",
            compiler.display()
        )));
    }
    let mut arguments = arguments.to_vec();
    if !arguments.iter().any(|arg| arg == "-resource-dir" || arg.starts_with("-resource-dir=")) {
        let resource = run_compiler(&compiler, &["-print-resource-dir".into()])?;
        let resource = input_path(Path::new(resource.stdout.trim()))?;
        arguments.push(format!("-resource-dir={}", path_string(&resource)?));
    }
    let preprocessing = preprocess(&compiler, &header, &arguments, &["-E", "-dM", "-v", "-H"])?;
    validate_driver_configuration(&preprocessing.stderr)?;
    let live = tokenize_snapshot(scanner, &preprocessing.stdout)?;
    let dependencies = preprocess(&compiler, &header, &arguments, &["-M", "-MT", "pgrx_c_macros"])?;
    let driver_files = parse_dependencies(&dependencies.stdout)?;
    let (inventory, declarations, target, included) =
        scanner.with_translation_unit(&header, &arguments, None, |unit, inventory| {
            let (declarations, included) = collect_declarations(unit);
            Ok((inventory, declarations, unit.get_target(), included))
        })?;
    let triple = compiler_triple(&preprocessing.stderr)?;
    if triple != target.triple {
        return Err(FrontendError::Environment(format!(
            "compiler target {} differs from libclang target {}",
            triple, target.triple
        )));
    }
    let library_files = included
        .into_iter()
        .chain(std::iter::once(header.clone()))
        .map(|path| absolute_path(&path))
        .collect::<Result<BTreeSet<_>, _>>()?;
    // Dependency output also includes successful header-availability searches.
    // Compare actual inclusions separately, retaining every dependency for rebuilds.
    let driver_identities = parse_include_trace(&preprocessing.stderr)?
        .iter()
        .chain(std::iter::once(&header))
        .map(|path| absolute_path(path))
        .collect::<Result<BTreeSet<_>, _>>()?;
    if library_files != driver_identities {
        let only_driver = driver_identities.difference(&library_files).collect::<Vec<_>>();
        let only_library = library_files.difference(&driver_identities).collect::<Vec<_>>();
        return Err(FrontendError::Environment(format!(
            "compiler and libclang included different files; compiler-only: {only_driver:?}; library-only: {only_library:?}"
        )));
    }
    let predefines = verify_predefines(scanner, &compiler, &arguments)?;
    let target =
        target_facts(predefines, target.triple, target.pointer_width, &preprocessing.stderr)?;
    let environment = join_active(live, &inventory);
    let macro_dependencies = MacroDependencyGraph::from_environment_with_constants(
        &environment,
        &declarations.integer_constants,
    );
    let (signed_overflow, unsupported_options) =
        semantic_options(&arguments, &preprocessing.stderr)?;
    let mut input_files = driver_files;
    input_files.extend(driver_identities);
    input_files.insert(requested_header);
    if let Some(library) = clang_sys::get_library() {
        input_files.insert(input_path(library.path())?);
        input_files.insert(absolute_path(library.path())?);
    }
    let inputs = build_inputs(&arguments, &preprocessing.stderr, input_files, search)?;
    let mut frontend = FrontendOutput {
        profile: CompilationProfile {
            header,
            compiler: CompilerIdentity { executable: compiler, version, libclang_version },
            arguments,
            target,
            signed_overflow,
            unsupported_options,
            inputs,
        },
        environment,
        declarations,
        inventory,
        dependencies: macro_dependencies,
    };
    for name in definitions::prove(scanner, &frontend)? {
        frontend
            .declarations
            .function_signatures
            .get_mut(&name)
            .expect("definition proof refers to a catalogued declaration")
            .definition_available = true;
    }
    frontend.declarations.bitfields = bitfields::probe(scanner, &frontend)?;
    let builtins = builtins::prove(scanner, &frontend)?;
    frontend.declarations.builtins = builtins.supported;
    frontend.declarations.builtin_unavailable = builtins.unavailable;
    Ok(frontend)
}

/// Resolve a filesystem identity for comparison and rebuild tracking without reusing it as include
/// lookup spelling.
fn absolute_path(path: &Path) -> Result<PathBuf, FrontendError> {
    path.canonicalize()
        .map_err(|source| FrontendError::CompilerIo { compiler: path.into(), source })
}

/// Make an input spelling absolute while retaining symlink-sensitive include lookup behavior.
fn input_path(path: &Path) -> Result<PathBuf, FrontendError> {
    if path.is_absolute() {
        Ok(path.to_owned())
    } else {
        std::env::current_dir()
            .map(|directory| directory.join(path))
            .map_err(|source| FrontendError::CompilerIo { compiler: path.into(), source })
    }
}

/// Reject non-UTF-8 compiler paths before passing them through the string-based driver interface.
fn path_string(path: &Path) -> Result<&str, FrontendError> {
    path.to_str()
        .ok_or_else(|| FrontendError::Arguments(format!("path {} is not UTF-8", path.display())))
}

/// Change-sensitive driver lookup history, including absent candidates whose creation could change
/// tool selection.
#[derive(Default)]
struct CompilerSearch {
    /// Observed or change-sensitive file inputs retained for content fingerprinting and rebuild
    /// invalidation.
    files: BTreeSet<PathBuf>,
    /// PATH search roots whose contents can change which executable wins.
    directories: BTreeSet<PathBuf>,
    /// Direct directory hints whose later replacement can make them valid executables.
    required_directories: BTreeSet<PathBuf>,
}

/// The chosen compatible executable together with the lookup dependencies that establish its
/// preference.
struct CompilerSelection {
    /// The compatible Clang driver chosen for preprocessing and typed probe checks.
    executable: PathBuf,
    /// Lookup dependencies needed to invalidate the selected driver when preference changes.
    search: CompilerSearch,
}

/// Combine explicit overrides, PostgreSQL hints, adjacent tools, and PATH roots into a compatible
/// driver search.
fn find_compiler(
    preferred: Option<&Path>,
    configured: Option<&Path>,
    library_version: &str,
) -> Result<CompilerSelection, FrontendError> {
    let explicit =
        preferred.map(PathBuf::from).or_else(|| std::env::var_os("CLANG_PATH").map(PathBuf::from));
    let major = version_major(library_version).ok_or_else(|| {
        FrontendError::CompilerUnavailable(format!(
            "unrecognized libclang version {library_version}"
        ))
    })?;
    let adjacent =
        clang_sys::get_library().and_then(|library| {
            library.path().parent().and_then(Path::parent).map(|prefix| {
                prefix.join("bin").join(format!("clang{}", std::env::consts::EXE_SUFFIX))
            })
        });
    let search_roots = std::env::var_os("PATH")
        .map(|path| std::env::split_paths(&path).collect::<Vec<_>>())
        .unwrap_or_default();
    select_compiler(explicit.as_deref(), configured, adjacent.as_deref(), major, &search_roots)
}

/// Apply driver preference in order while retaining searches whose later changes could select a
/// different executable.
fn select_compiler(
    explicit: Option<&Path>,
    configured: Option<&Path>,
    adjacent: Option<&Path>,
    major: u32,
    search_roots: &[PathBuf],
) -> Result<CompilerSelection, FrontendError> {
    let mut search = CompilerSearch::default();
    if let Some(explicit) = explicit {
        let executable = search
            .resolve(explicit, search_roots)?
            .ok_or_else(|| FrontendError::CompilerUnavailable(explicit.display().to_string()))?;
        return Ok(CompilerSelection { executable, search });
    }
    let names = [
        format!("clang-{major}{}", std::env::consts::EXE_SUFFIX),
        format!("clang{}", std::env::consts::EXE_SUFFIX),
    ];
    for candidate in configured.into_iter().chain(adjacent).chain(names.iter().map(Path::new)) {
        if let Some(executable) = search.resolve(candidate, search_roots)?
            && let Ok(output) = run_compiler(&executable, &["--version".into()])
            && version_major(&output.stdout) == Some(major)
        {
            return Ok(CompilerSelection { executable, search });
        }
    }
    Err(FrontendError::CompilerUnavailable(format!(
        "no executable matches libclang major {major}; set CLANG_PATH"
    )))
}

/// Record executable spelling, identity, and lookup roots during driver preference resolution.
impl CompilerSearch {
    /// Resolve a direct executable or PATH name and record both successful and change-sensitive
    /// lookup locations.
    fn resolve(
        &mut self,
        path: &Path,
        search_roots: &[PathBuf],
    ) -> Result<Option<PathBuf>, FrontendError> {
        if path.components().count() > 1 || path.is_absolute() {
            let path = input_path(path)?;
            if path.is_dir() {
                // A non-file hint can become an executable after replacement.
                self.required_directories.insert(path.clone());
                if let Ok(identity) = path.canonicalize() {
                    self.required_directories.insert(identity);
                }
            } else {
                self.record_file(&path);
            }
            return Ok(path.is_file().then_some(path));
        }
        for root in search_roots {
            let root = input_path(root)?;
            self.directories.insert(root.clone());
            let candidate = root.join(path);
            if candidate.is_file() {
                self.record_file(&candidate);
                return Ok(Some(candidate));
            }
        }
        Ok(None)
    }

    /// Track the requested executable spelling and any current canonical identity for rebuild
    /// invalidation.
    fn record_file(&mut self, path: &Path) {
        self.files.insert(path.to_owned());
        if let Ok(identity) = path.canonicalize() {
            self.files.insert(identity);
        }
    }
}

/// Build a C-mode invocation from recorded flags and phase-specific options without changing the
/// inspected input profile.
pub(crate) fn driver_arguments(
    arguments: &[String],
    options: &[&str],
    header: Option<&Path>,
) -> Vec<String> {
    let mut all = vec!["-x".into(), "c".into()];
    all.extend_from_slice(arguments);
    all.extend(options.iter().map(|option| (*option).to_owned()));
    if let Some(header) = header {
        // The public entry point validated this path before constructing commands.
        all.push(header.to_str().expect("analysis header is UTF-8").into());
    }
    all
}

/// Run a bounded driver preprocessing phase against the selected header using validated path
/// spelling.
pub(crate) fn preprocess(
    compiler: &Path,
    header: &Path,
    arguments: &[String],
    options: &[&str],
) -> Result<CompilerOutput, FrontendError> {
    path_string(header)?;
    run_compiler(compiler, &driver_arguments(arguments, options, Some(header)))
}

/// Recover the effective frontend target from the driver command instead of assuming the generator
/// host target.
pub(crate) fn compiler_triple(verbose: &str) -> Result<String, FrontendError> {
    for line in verbose.lines() {
        let Some(arguments) = shlex::split(line) else { continue };
        if arguments.iter().any(|argument| argument == "-cc1")
            && let Some(pair) = arguments.windows(2).find(|pair| pair[0] == "-triple")
        {
            return Ok(pair[1].clone());
        }
    }
    Err(FrontendError::Output("Clang verbose output did not identify its frontend target".into()))
}

/// Read actual driver inclusions so inspection can compare them with libclang, separately from
/// availability searches.
fn parse_include_trace(verbose: &str) -> Result<BTreeSet<PathBuf>, FrontendError> {
    let mut included = BTreeSet::new();
    for line in verbose.lines() {
        let depth = line.bytes().take_while(|byte| *byte == b'.').count();
        if depth != 0 && line.as_bytes().get(depth) == Some(&b' ') {
            included.insert(input_path(Path::new(&line[depth + 1..]))?);
        }
    }
    Ok(included)
}

/// Replay the final macro dump through libclang with protected line endings to recover lexical token
/// categories.
pub(crate) fn tokenize_snapshot(
    scanner: &MacroScanner,
    contents: &str,
) -> Result<Vec<MacroDefinition>, FrontendError> {
    // Clang emits one complete definition per dump line. A literal terminal
    // backslash must not splice that line into the next definition when replayed.
    /// Seed a comment marker that prevents terminal backslashes from joining adjacent macro-dump
    /// definitions.
    const PREFIX: &str = "/*__pgrx_c_snapshot_end_";
    let occupied = contents
        .match_indices(PREFIX)
        .filter_map(|(offset, _)| {
            let suffix = &contents[offset + PREFIX.len()..];
            let digits = suffix.bytes().take_while(u8::is_ascii_digit).count();
            suffix[digits..]
                .starts_with("__*/")
                .then(|| suffix[..digits].parse::<usize>().ok())
                .flatten()
        })
        .collect::<HashSet<_>>();
    let mut number = 0;
    while occupied.contains(&number) {
        number += 1;
    }
    let marker = format!("{PREFIX}{number}__*/");
    let mut protected = String::new();
    for line in contents.lines() {
        protected.push_str(line);
        protected.push(' ');
        protected.push_str(&marker);
        protected.push('\n');
    }
    let header = std::env::temp_dir().join("pgrx-c-macros-snapshot.h");
    let inventory = scanner.scan_unsaved(
        &header,
        &protected,
        &["-undef".into(), "-Wno-builtin-macro-redefined".into()],
    )?;
    Ok(inventory
        .macros
        .into_iter()
        .filter(|definition| definition.main_file)
        .map(|mut definition| {
            definition
                .tokens
                .retain(|token| token.kind != TokenKind::Comment || token.spelling != marker);
            definition
        })
        .collect())
}

/// Build the lexical definition key used to join a final active macro with physical discovery
/// history.
fn signature(definition: &MacroDefinition) -> (bool, Vec<&str>) {
    (
        definition.kind == MacroKind::FunctionLike,
        definition
            .tokens
            .iter()
            .filter(|token| token.kind != TokenKind::Comment)
            .map(|token| token.spelling.as_str())
            .collect(),
    )
}

/// Attach physical provenance to the final active definitions, retaining ambiguity when multiple
/// source definitions match.
fn join_active(live: Vec<MacroDefinition>, inventory: &MacroInventory) -> MacroEnvironment {
    let mut history = HashMap::<_, (Vec<_>, HashSet<_>)>::new();
    for definition in &inventory.macros {
        let (definitions, locations) = history.entry(signature(definition)).or_default();
        // Re-including an unguarded header can record one physical definition
        // several times. Only distinct physical definitions make provenance ambiguous.
        if let Some(location) = &definition.location
            && !locations.insert((&location.file, location.offset))
        {
            continue;
        }
        definitions.push(definition);
    }
    let mut environment = MacroEnvironment::default();
    for mut definition in live {
        let matches = history.get(&signature(&definition)).map(|(definitions, _)| definitions);
        let (original, provenance) = match matches.map(Vec::as_slice).unwrap_or_default() {
            [original] => (Some((**original).clone()), ActiveProvenance::Resolved),
            [] => (None, ActiveProvenance::Unresolved),
            matches => (
                Some((**matches.last().expect("matching history is nonempty")).clone()),
                ActiveProvenance::Ambiguous(
                    matches.iter().filter_map(|original| original.provenance.clone()).collect(),
                ),
            ),
        };
        if let Some(original) = original {
            definition = original;
        } else {
            definition.provenance = None;
            definition.location = None;
            definition.main_file = false;
        }
        environment.active.insert(definition.name.clone(), ActiveMacro { definition, provenance });
    }
    environment
}

/// Copy top-level declarations and include identities before the translation unit releases its Clang
/// handles.
fn collect_declarations(unit: &TranslationUnit<'_>) -> (DeclarationCatalog, BTreeSet<PathBuf>) {
    let mut catalog = DeclarationCatalog::default();
    let mut included = BTreeSet::new();
    unit.get_entity().visit_children(|entity, _| {
        if entity.get_kind() == EntityKind::InclusionDirective {
            if let Some(file) = entity.get_file() {
                included.insert(file.get_path());
            }
            return EntityVisitResult::Continue;
        }
        match entity.get_kind() {
            EntityKind::TypedefDecl => {
                if let (Some(name), Some(ty)) =
                    (entity.get_name(), entity.get_typedef_underlying_type())
                {
                    let mut info = type_info(ty);
                    if entity.has_attributes() {
                        info.category = TypeCategory::Other;
                    }
                    catalog.types.insert(name, info);
                }
            }
            EntityKind::StructDecl | EntityKind::UnionDecl | EntityKind::EnumDecl => {
                if let Some(ty) = entity.get_type() {
                    let info = type_info(ty);
                    if info.size.is_some() || !catalog.types.contains_key(&info.spelling) {
                        catalog.types.insert(info.spelling.clone(), info);
                    }
                }
                return EntityVisitResult::Recurse;
            }
            EntityKind::EnumConstantDecl => {
                if let (Some(name), Some(ty), Some((signed, unsigned))) =
                    (entity.get_name(), entity.get_type(), entity.get_enum_constant_value())
                {
                    let value_type = if ty.get_canonical_type().get_kind() == TypeKind::Enum {
                        ty.get_declaration()
                            .and_then(|declaration| declaration.get_enum_underlying_type())
                            .unwrap_or(ty)
                    } else {
                        ty
                    };
                    if value_type.get_sizeof().is_ok_and(|size| size <= 8) {
                        let value = if value_type.get_canonical_type().is_unsigned_integer() {
                            IntegerValue::Unsigned(unsigned)
                        } else {
                            IntegerValue::Signed(signed)
                        };
                        catalog.integer_constants.insert(
                            name,
                            IntegerConstant { ty: type_info(ty), value, literal: None },
                        );
                    }
                }
            }
            EntityKind::VarDecl | EntityKind::FunctionDecl => {
                if let (Some(name), Some(ty)) = (entity.get_name(), entity.get_type()) {
                    if entity.get_kind() == EntityKind::FunctionDecl {
                        catalog.functions.insert(name, type_info(ty));
                    } else {
                        catalog.variables.insert(name, type_info(ty));
                    }
                }
            }
            _ => {}
        }
        EntityVisitResult::Continue
    });
    types::collect(unit, &mut catalog);
    (catalog, included)
}

/// Copy representation, qualifiers, and canonical C identity from a live Clang type for downstream
/// checks.
pub(crate) fn type_info(ty: Type<'_>) -> TypeInfo {
    let canonical = ty.get_canonical_type();
    let category = if has_type_attributes(ty) {
        TypeCategory::Other
    } else if let Some(integer) = integer_kind(canonical.get_kind()) {
        TypeCategory::Integer(integer)
    } else {
        match canonical.get_kind() {
            TypeKind::Float | TypeKind::Double | TypeKind::LongDouble => TypeCategory::Floating,
            TypeKind::Pointer => TypeCategory::Pointer,
            TypeKind::Record => TypeCategory::Record,
            TypeKind::Enum => TypeCategory::Enum,
            TypeKind::FunctionPrototype | TypeKind::FunctionNoPrototype => TypeCategory::Function,
            TypeKind::Void => TypeCategory::Void,
            _ => TypeCategory::Other,
        }
    };
    TypeInfo {
        spelling: ty.get_display_name(),
        canonical_spelling: canonical.get_display_name(),
        category,
        size: ty.get_sizeof().ok().map(|size| size as u64),
        alignment: ty.get_alignof().ok().map(|size| size as u64),
        is_const: ty.is_const_qualified() || canonical.is_const_qualified(),
        is_volatile: ty.is_volatile_qualified() || canonical.is_volatile_qualified(),
    }
}

// Canonicalization can erase arithmetic-affecting attributes on a typedef or an alias.
// Keep such types outside the integer family until their semantics are modeled.
/// Reject type spellings with attributes that ordinary canonical type facts cannot fully model.
fn has_type_attributes(mut ty: Type<'_>) -> bool {
    for _ in 0..64 {
        if ty.get_kind() == TypeKind::Attributed {
            return true;
        }
        let Some(declaration) = ty.get_declaration() else {
            return false;
        };
        if declaration.has_attributes() {
            let attributes = declaration
                .get_children()
                .into_iter()
                .filter(|child| child.is_attribute())
                .collect::<Vec<_>>();
            // Record field offsets, size and alignment describe these layout
            // attributes. Arithmetic typedef attributes still need their own model.
            if ty.get_canonical_type().get_kind() != TypeKind::Record
                || attributes.is_empty()
                || attributes.iter().any(|attribute| {
                    !matches!(
                        attribute.get_kind(),
                        EntityKind::PackedAttr | EntityKind::AlignedAttr
                    )
                })
            {
                return true;
            }
        }
        if ty.get_kind() != TypeKind::Typedef {
            return false;
        }
        let Some(underlying) = declaration.get_typedef_underlying_type() else {
            return true;
        };
        ty = underlying;
    }
    true
}

/// Classify a fundamental Clang integer kind without collapsing equal-width C identities.
fn integer_kind(kind: TypeKind) -> Option<IntegerKind> {
    Some(match kind {
        TypeKind::Bool => IntegerKind::Bool,
        TypeKind::CharS | TypeKind::CharU => IntegerKind::Char,
        TypeKind::SChar => IntegerKind::SignedChar,
        TypeKind::UChar => IntegerKind::UnsignedChar,
        TypeKind::Short => IntegerKind::Short,
        TypeKind::UShort => IntegerKind::UnsignedShort,
        TypeKind::Int => IntegerKind::Int,
        TypeKind::UInt => IntegerKind::UnsignedInt,
        TypeKind::Long => IntegerKind::Long,
        TypeKind::ULong => IntegerKind::UnsignedLong,
        TypeKind::LongLong => IntegerKind::LongLong,
        TypeKind::ULongLong => IntegerKind::UnsignedLongLong,
        TypeKind::Int128 => IntegerKind::Int128,
        TypeKind::UInt128 => IntegerKind::UnsignedInt128,
        _ => return None,
    })
}

/// Target witnesses agreed by driver preprocessing and typed libclang probes before profile
/// construction.
struct VerifiedPredefines {
    /// Final target predefines observed by the driver and replayed through libclang.
    macros: Vec<MacroDefinition>,
    /// Typed fundamental C integer witnesses agreeing with driver predefines.
    integers: BTreeMap<IntegerKind, IntegerType>,
    /// Canonical unsigned C identity of sizeof/alignment results verified by both compiler paths.
    size_type: IntegerKind,
    /// Whether optional intrinsic offset witnesses passed without hiding required fact errors.
    offsetof_supported: bool,
    /// Whether supported ASCII characters and basic escapes match the bounded literal decoder.
    ascii_execution_charset: bool,
    /// Typed float representation facts awaiting semantic option reconciliation.
    floating_types: BTreeMap<FloatingKind, (u64, Option<u64>)>,
    /// The effective C floating evaluation mode, including excess-precision behavior.
    evaluation_method: i32,
    /// Native function-pointer layout agreed by driver assertions and libclang.
    function_pointer: crate::PointerLayout,
}

/// Require driver macros and typed libclang witnesses to agree on the target facts used by semantic
/// helpers.
fn verify_predefines(
    scanner: &MacroScanner,
    compiler: &Path,
    arguments: &[String],
) -> Result<VerifiedPredefines, FrontendError> {
    let mut without_includes = Vec::new();
    let mut arguments = arguments.iter();
    while let Some(argument) = arguments.next() {
        if argument == "-include" || argument == "-imacros" {
            arguments.next();
        } else if !argument.starts_with("-include") && !argument.starts_with("-imacros") {
            without_includes.push(argument.clone());
        }
    }
    let mut command = driver_arguments(&without_includes, &["-E", "-dM"], None);
    command.push("-".into());
    let dump = run_compiler(compiler, &command)?;
    let predefines = tokenize_snapshot(scanner, &dump.stdout)?;
    let actual: BTreeMap<_, _> =
        predefines.iter().map(|definition| (definition.name.as_str(), definition)).collect();
    for name in
        ["sizeof", "_Alignof", "__typeof__", "_Static_assert", "__builtin_types_compatible_p"]
    {
        if actual.contains_key(name) {
            return Err(FrontendError::Environment(format!(
                "compiler fundamental type proof requires unshadowed {name}"
            )));
        }
    }
    let probe_offsetof = !actual.keys().any(|name| {
        name.starts_with("__pgrx_c_offset_")
            || matches!(
                *name,
                "__builtin_offsetof"
                    | "__has_builtin"
                    | "struct"
                    | "first"
                    | "nested"
                    | "prefix"
                    | "values"
            )
    });
    let header = std::env::temp_dir().join("pgrx-c-macros-target.h");
    let fundamental_source = "\
typedef _Bool __pgrx_c_bool;\n\
typedef char __pgrx_c_char;\n\
typedef signed char __pgrx_c_schar;\n\
typedef unsigned char __pgrx_c_uchar;\n\
typedef short __pgrx_c_short;\n\
typedef unsigned short __pgrx_c_ushort;\n\
typedef int __pgrx_c_int;\n\
typedef unsigned int __pgrx_c_uint;\n\
typedef long __pgrx_c_long;\n\
typedef unsigned long __pgrx_c_ulong;\n\
typedef long long __pgrx_c_llong;\n\
typedef unsigned long long __pgrx_c_ullong;\n\
typedef __typeof__(sizeof(0)) __pgrx_c_size_type;\n\
typedef __typeof__(_Alignof(int)) __pgrx_c_align_type;\n\
typedef float __pgrx_c_float;\n\
typedef double __pgrx_c_double;\n\
typedef long double __pgrx_c_ldouble;\n\
typedef void (*__pgrx_c_function_pointer)(void);\n\
enum { __pgrx_c_float_eval_method = __FLT_EVAL_METHOD__ };\n\
#ifdef __SIZEOF_INT128__\n\
typedef __int128 __pgrx_c_i128;\n\
typedef unsigned __int128 __pgrx_c_u128;\n\
_Static_assert((((unsigned __int128)-1) >> 127) == 1, \"unsupported __int128 precision\");\n\
#endif\n";
    let ascii = ascii_character_predicate();
    let offsetof_source = if probe_offsetof {
        "\
#if __has_builtin(__builtin_offsetof)\n\
struct __pgrx_c_offset_nested { char prefix; unsigned long values[3]; };\n\
struct __pgrx_c_offset_probe { char first; struct __pgrx_c_offset_nested nested; };\n\
typedef __typeof__(__builtin_offsetof(struct __pgrx_c_offset_probe, first)) __pgrx_c_offset_type;\n\
enum { __pgrx_c_offset_first = __builtin_offsetof(struct __pgrx_c_offset_probe, first),\n\
__pgrx_c_offset_nested = __builtin_offsetof(struct __pgrx_c_offset_probe, nested),\n\
__pgrx_c_offset_indexed = __builtin_offsetof(struct __pgrx_c_offset_probe, nested.values[2]) };\n\
#endif\n"
    } else {
        ""
    };
    let fundamental_source =
        format!("{fundamental_source}\nenum {{ __pgrx_c_ascii = ({ascii}) }};\n{offsetof_source}");
    let mut probe = driver_arguments(
        &without_includes,
        &[
            "-fsyntax-only",
            "-ferror-limit=0",
            "-fno-color-diagnostics",
            "-fdiagnostics-format=clang",
        ],
        None,
    );
    probe.push("-".into());
    let source = format!("_Static_assert(({ascii}), \"pgrx_ascii_execution_charset\");\n");
    let driver_ascii = match run_compiler_with_input(compiler, &probe, Some(source)) {
        Ok(_) => true,
        Err(FrontendError::CompilerFailed { diagnostics, .. })
            if diagnostics.contains("pgrx_ascii_execution_charset") =>
        {
            false
        }
        Err(error) => return Err(error),
    };
    let (
        inventory,
        (
            widths,
            floating_types,
            library_ascii,
            evaluation_method,
            function_pointer,
            size_type,
            offsetof,
        ),
    ) = scanner.with_translation_unit(
        &header,
        &without_includes,
        Some(&fundamental_source),
        |unit, inventory| {
            let mut widths = BTreeMap::new();
            let mut library_ascii = false;
            let mut floating_types = BTreeMap::new();
            let mut evaluation_method = None;
            let mut function_pointer = None;
            let mut size_type = None;
            let mut align_type = None;
            let mut offsetof_type = None;
            let mut offsetof_values = BTreeMap::new();
            let mut offset_nested = None;
            let mut offset_record = None;
            unit.get_entity().visit_children(|entity, _| {
                if entity.get_kind() == EntityKind::EnumDecl {
                    entity.visit_children(|constant, _| {
                        if constant.get_name().as_deref() == Some("__pgrx_c_ascii") {
                            library_ascii = constant
                                .get_enum_constant_value()
                                .is_some_and(|(value, _)| value == 1);
                        } else if constant.get_name().as_deref()
                            == Some("__pgrx_c_float_eval_method")
                        {
                            evaluation_method = constant
                                .get_enum_constant_value()
                                .and_then(|(value, _)| i32::try_from(value).ok());
                        } else if let Some(name) = constant.get_name()
                            && matches!(
                                name.as_str(),
                                "__pgrx_c_offset_first"
                                    | "__pgrx_c_offset_nested"
                                    | "__pgrx_c_offset_indexed"
                            )
                            && let Some((value, _)) = constant.get_enum_constant_value()
                            && let Ok(value) = u64::try_from(value)
                        {
                            offsetof_values.insert(name, value);
                        }
                        EntityVisitResult::Continue
                    });
                }
                if entity.get_kind() == EntityKind::StructDecl {
                    match entity.get_name().as_deref() {
                        Some("__pgrx_c_offset_nested") => offset_nested = entity.get_type(),
                        Some("__pgrx_c_offset_probe") => offset_record = entity.get_type(),
                        _ => {}
                    }
                }
                if entity.get_kind() == EntityKind::TypedefDecl
                    && entity.get_name().is_some_and(|name| name.starts_with("__pgrx_c_"))
                    && let Some(ty) =
                        entity.get_typedef_underlying_type().map(|ty| ty.get_canonical_type())
                {
                    match entity.get_name().as_deref() {
                        Some("__pgrx_c_size_type") => size_type = integer_kind(ty.get_kind()),
                        Some("__pgrx_c_align_type") => align_type = integer_kind(ty.get_kind()),
                        Some("__pgrx_c_offset_type") => offsetof_type = integer_kind(ty.get_kind()),
                        _ => {}
                    }
                    if entity.get_name().as_deref() == Some("__pgrx_c_function_pointer")
                        && let (Ok(size), Ok(alignment)) = (ty.get_sizeof(), ty.get_alignof())
                    {
                        function_pointer = Some(crate::PointerLayout {
                            size: size as u64,
                            alignment: alignment as u64,
                        });
                    }
                    if let (Some(kind), Ok(size)) = (integer_kind(ty.get_kind()), ty.get_sizeof()) {
                        widths.insert(kind, (size, ty.get_kind() == TypeKind::CharS));
                    }
                    let kind = match ty.get_kind() {
                        TypeKind::Float => Some(FloatingKind::Float),
                        TypeKind::Double => Some(FloatingKind::Double),
                        TypeKind::LongDouble => Some(FloatingKind::LongDouble),
                        _ => None,
                    };
                    if let (Some(kind), Ok(size)) = (kind, ty.get_sizeof()) {
                        floating_types.insert(
                            kind,
                            (size as u64, ty.get_alignof().ok().map(|align| align as u64)),
                        );
                    }
                }
                EntityVisitResult::Continue
            });
            if align_type != size_type {
                return Err(Error::InvalidInput(
                    "libclang sizeof and _Alignof canonical result identities differ".into(),
                ));
            }
            let offsetof = (|| {
                let nested = offset_nested?;
                let record = offset_record?;
                let first = record.get_offsetof("first").ok()? as u64;
                let nested_offset = record.get_offsetof("nested").ok()? as u64;
                let values = nested.get_offsetof("values").ok()? as u64;
                // Clang offsets use bits; predefines establish how many comprise a C byte.
                let bits = macro_number(&actual, "__CHAR_BIT__").ok()?;
                if bits == 0 {
                    return None;
                }
                let indexed = nested_offset.checked_add(values)?.checked_add(
                    (widths.get(&IntegerKind::UnsignedLong)?.0 as u64)
                        .checked_mul(bits)?
                        .checked_mul(2)?,
                )?;
                let expected = [first, nested_offset, indexed];
                if offsetof_type != size_type {
                    return None;
                }
                for (name, offset) in
                    ["__pgrx_c_offset_first", "__pgrx_c_offset_nested", "__pgrx_c_offset_indexed"]
                        .into_iter()
                        .zip(expected)
                {
                    if offsetof_values.get(name)?.checked_mul(bits)? != offset {
                        return None;
                    }
                }
                Some(expected)
            })();
            Ok((
                inventory,
                (
                    widths,
                    floating_types,
                    library_ascii,
                    evaluation_method,
                    function_pointer,
                    size_type,
                    offsetof,
                ),
            ))
        },
    )?;
    let mut expected: BTreeMap<_, _> =
        inventory.macros.iter().map(|definition| (definition.name.as_str(), definition)).collect();
    let mut overrides = BTreeMap::new();
    let mut arguments = without_includes.iter();
    while let Some(argument) = arguments.next() {
        let operation = match argument.as_str() {
            "-D" => arguments.next().map(|value| (true, value.as_str())),
            "-U" => arguments.next().map(|value| (false, value.as_str())),
            _ => argument
                .strip_prefix("-D")
                .map(|value| (true, value))
                .or_else(|| argument.strip_prefix("-U").map(|value| (false, value))),
        };
        if let Some((defined, value)) = operation {
            let name = value
                .split(|character: char| !character.is_ascii_alphanumeric() && character != '_')
                .next()
                .unwrap_or_default();
            overrides.insert(name, defined);
        }
    }
    expected.retain(|name, _| overrides.get(name) != Some(&false));
    let differing: Vec<_> = actual
        .iter()
        .filter_map(|(name, definition)| {
            expected
                .get(name)
                .filter(|expected| signature(expected) == signature(definition))
                .is_none()
                .then_some(*name)
        })
        .chain(expected.keys().filter(|name| !actual.contains_key(**name)).copied())
        .collect();
    if !differing.is_empty() {
        return Err(FrontendError::Environment(format!(
            "compiler and libclang predefined macros differ: {}",
            differing.join(", ")
        )));
    }
    let evaluation_method = evaluation_method.ok_or_else(|| {
        FrontendError::Output("libclang did not establish __FLT_EVAL_METHOD__".into())
    })?;
    let function_pointer = function_pointer.ok_or_else(|| {
        FrontendError::Output("libclang did not establish the C function-pointer layout".into())
    })?;
    let size_type = size_type.ok_or_else(|| {
        FrontendError::Output("libclang did not establish the sizeof result identity".into())
    })?;
    let size_spelling = match size_type {
        IntegerKind::UnsignedChar => "unsigned char",
        IntegerKind::UnsignedShort => "unsigned short",
        IntegerKind::UnsignedInt => "unsigned int",
        IntegerKind::UnsignedLong => "unsigned long",
        IntegerKind::UnsignedLongLong => "unsigned long long",
        IntegerKind::UnsignedInt128 => "unsigned __int128",
        _ => {
            return Err(FrontendError::Environment(format!(
                "sizeof has non-unsigned canonical result {size_type:?}"
            )));
        }
    };
    let mut float_source = format!(
        "_Static_assert(__FLT_EVAL_METHOD__ == {evaluation_method}, \"pgrx_float_evaluation_method\");\n"
    );
    float_source.push_str(&format!(
        "_Static_assert(__builtin_types_compatible_p(__typeof__(sizeof(0)), {size_spelling}), \"pgrx_size_type\");\n"
    ));
    float_source.push_str(&format!(
        "_Static_assert(__builtin_types_compatible_p(__typeof__(_Alignof(int)), {size_spelling}), \"pgrx_align_type\");\n"
    ));
    float_source.push_str(&format!("typedef void (*__pgrx_c_function_pointer)(void);\n_Static_assert(sizeof(__pgrx_c_function_pointer) == {}, \"pgrx_function_pointer_size\");\n_Static_assert(_Alignof(__pgrx_c_function_pointer) == {}, \"pgrx_function_pointer_alignment\");\n", function_pointer.size, function_pointer.alignment));
    for (kind, spelling) in [
        (FloatingKind::Float, "float"),
        (FloatingKind::Double, "double"),
        (FloatingKind::LongDouble, "long double"),
    ] {
        if let Some(&(size, alignment)) = floating_types.get(&kind) {
            float_source.push_str(&format!(
                "_Static_assert(sizeof({spelling}) == {size}, \"pgrx_float_size\");\n"
            ));
            if let Some(alignment) = alignment {
                float_source.push_str(&format!("_Static_assert(_Alignof({spelling}) == {alignment}, \"pgrx_float_alignment\");\n"));
            }
        }
    }
    if let Some(offsets) = offsetof {
        // Optional failures must be attributable to this final capability block.
        // Required fundamental assertions precede it, with diagnostic limits disabled.
        float_source.push_str("#line 1 \"pgrx_c_offsetof_proof\"\n");
        float_source.push_str(offsetof_source);
        float_source.push_str(&format!(
            "_Static_assert(__builtin_types_compatible_p(__typeof__(__builtin_offsetof(struct __pgrx_c_offset_probe, first)), {size_spelling}), \"pgrx_offsetof_type\");\n"
        ));
        for (designator, offset) in ["first", "nested", "nested.values[2]"].into_iter().zip(offsets)
        {
            float_source.push_str(&format!(
                "_Static_assert(__builtin_offsetof(struct __pgrx_c_offset_probe, {designator}) * __CHAR_BIT__ == {offset}, \"pgrx_offsetof_layout\");\n"
            ));
        }
    }
    let offsetof_supported = match run_compiler_with_input(compiler, &probe, Some(float_source)) {
        Ok(_) => offsetof.is_some(),
        Err(FrontendError::CompilerFailed { diagnostics, status, .. })
            if status.code().is_some()
                && offsetof.is_some()
                && only_offsetof_errors(&diagnostics) =>
        {
            false
        }
        Err(FrontendError::CompilerFailed { diagnostics, .. }) => {
            return Err(FrontendError::Environment(format!(
                "compiler fundamental type facts differ from libclang: {diagnostics}"
            )));
        }
        Err(error) => return Err(error),
    };
    let bits = macro_number(&actual, "__CHAR_BIT__")?
        .try_into()
        .map_err(|_| FrontendError::Output("invalid CHAR_BIT".into()))?;
    let mut integers = BTreeMap::new();
    for (kind, (bytes, char_signed)) in widths {
        let bits = u32::try_from(bytes)
            .ok()
            .and_then(|bytes| bytes.checked_mul(bits))
            .ok_or_else(|| FrontendError::Output("integer storage width overflow".into()))?;
        let rank = match kind {
            IntegerKind::Bool => 0,
            IntegerKind::Char | IntegerKind::SignedChar | IntegerKind::UnsignedChar => 1,
            IntegerKind::Short | IntegerKind::UnsignedShort => 2,
            IntegerKind::Int | IntegerKind::UnsignedInt => 3,
            IntegerKind::Long | IntegerKind::UnsignedLong => 4,
            IntegerKind::LongLong | IntegerKind::UnsignedLongLong => 5,
            IntegerKind::Int128 | IntegerKind::UnsignedInt128 => 6,
        };
        let signed = match kind {
            IntegerKind::Char => char_signed,
            IntegerKind::SignedChar
            | IntegerKind::Short
            | IntegerKind::Int
            | IntegerKind::Long
            | IntegerKind::LongLong
            | IntegerKind::Int128 => true,
            _ => false,
        };
        integers.insert(kind, IntegerType { kind, bits, signed, rank });
    }
    Ok(VerifiedPredefines {
        macros: predefines,
        integers,
        size_type,
        offsetof_supported,
        ascii_execution_charset: driver_ascii && library_ascii,
        floating_types,
        evaluation_method,
        function_pointer,
    })
}

/// Allow an optional offsetof proof to fail only when diagnostics belong exclusively to that optional
/// witness.
fn only_offsetof_errors(diagnostics: &str) -> bool {
    let mut found = false;
    for line in diagnostics.lines().filter(|line| line.contains("error:")) {
        if !line.starts_with("pgrx_c_offsetof_proof:") || line.contains("fatal error:") {
            return false;
        }
        found = true;
    }
    found
}

/// Construct the character-set witness that makes supported character literal values independent of
/// host assumptions.
fn ascii_character_predicate() -> String {
    let mut terms = (32_u8..=126)
        .map(|byte| {
            let literal = match byte {
                b'\'' => "\\'".to_owned(),
                b'\\' => "\\\\".to_owned(),
                _ => char::from(byte).to_string(),
            };
            format!("('{literal}' == {byte})")
        })
        .collect::<Vec<_>>();
    for (escape, value) in
        [("a", 7), ("b", 8), ("f", 12), ("n", 10), ("r", 13), ("t", 9), ("v", 11)]
    {
        terms.push(format!("('\\{escape}' == {value})"));
    }
    terms.join(" && ")
}

/// Parse a verified numeric predefined macro for target representation checks.
fn macro_number(
    macros: &BTreeMap<&str, &MacroDefinition>,
    name: &str,
) -> Result<u64, FrontendError> {
    let mut name = name;
    let mut visited = BTreeSet::new();
    while visited.insert(name) {
        let definition = macros
            .get(name)
            .ok_or_else(|| FrontendError::Output(format!("missing compiler macro {name}")))?;
        let mut tokens =
            definition.tokens.iter().skip(1).filter(|token| token.kind != TokenKind::Comment);
        let token = tokens
            .next()
            .ok_or_else(|| FrontendError::Output(format!("empty compiler macro {name}")))?;
        if tokens.next().is_some() {
            return Err(FrontendError::Output(format!(
                "compiler macro {name} is not one numeric token"
            )));
        }
        if token.kind == TokenKind::Identifier {
            name = &token.spelling;
        } else {
            let digits = token.spelling.trim_end_matches(['u', 'U', 'l', 'L']);
            return digits.parse().map_err(|_| {
                FrontendError::Output(format!(
                    "compiler macro {name} has unsupported value {}",
                    token.spelling
                ))
            });
        }
    }
    Err(FrontendError::Output(format!("recursive compiler macro {name}")))
}

/// Assemble agreed scalar, pointer, character, and layout facts into the profile consumed by
/// analysis.
fn target_facts(
    verified: VerifiedPredefines,
    triple: String,
    pointer_width: usize,
    verbose: &str,
) -> Result<TargetFacts, FrontendError> {
    let VerifiedPredefines {
        macros: predefines,
        integers,
        size_type,
        offsetof_supported,
        ascii_execution_charset,
        floating_types,
        evaluation_method,
        function_pointer,
    } = verified;
    let macros = predefines
        .iter()
        .map(|definition| (definition.name.as_str(), definition))
        .collect::<BTreeMap<_, _>>();
    let char_bits = u32::try_from(macro_number(&macros, "__CHAR_BIT__")?)
        .map_err(|_| FrontendError::Output("invalid CHAR_BIT".into()))?;
    let pointer_bits = macro_number(&macros, "__SIZEOF_POINTER__")?
        .checked_mul(u64::from(char_bits))
        .and_then(|bits| u32::try_from(bits).ok())
        .ok_or_else(|| FrontendError::Output("invalid pointer width".into()))?;
    if pointer_width != pointer_bits as usize {
        return Err(FrontendError::Environment(format!(
            "compiler pointer width {pointer_bits} differs from libclang width {pointer_width}"
        )));
    }
    for (name, kind) in [
        ("__SIZEOF_SHORT__", IntegerKind::Short),
        ("__SIZEOF_INT__", IntegerKind::Int),
        ("__SIZEOF_LONG__", IntegerKind::Long),
        ("__SIZEOF_LONG_LONG__", IntegerKind::LongLong),
    ] {
        let bits = macro_number(&macros, name)?
            .checked_mul(u64::from(char_bits))
            .ok_or_else(|| FrontendError::Output("invalid integer width".into()))?;
        if integers.get(&kind).is_none_or(|ty| u64::from(ty.bits) != bits) {
            return Err(FrontendError::Environment(format!(
                "compiler {name} differs from libclang type layout"
            )));
        }
    }
    for (name, kind) in [
        ("__SCHAR_MAX__", IntegerKind::SignedChar),
        ("__SHRT_MAX__", IntegerKind::Short),
        ("__INT_MAX__", IntegerKind::Int),
        ("__LONG_MAX__", IntegerKind::Long),
        ("__LONG_LONG_MAX__", IntegerKind::LongLong),
    ] {
        let bits = integers
            .get(&kind)
            .ok_or_else(|| {
                FrontendError::Output(format!("missing fundamental integer type {kind:?}"))
            })?
            .bits;
        let maximum = 1u64
            .checked_shl(
                bits.checked_sub(1)
                    .ok_or_else(|| FrontendError::Output("integer has no value bits".into()))?,
            )
            .and_then(|value| value.checked_sub(1))
            .ok_or_else(|| {
                FrontendError::Environment(format!("unsupported integer precision for {kind:?}"))
            })?;
        if macro_number(&macros, name)? != maximum {
            return Err(FrontendError::Environment(format!(
                "{name} does not match the {bits}-bit integer representation; padding bits are unsupported"
            )));
        }
    }
    let byte_order = macro_number(&macros, "__BYTE_ORDER__")?;
    let byte_order = if byte_order == macro_number(&macros, "__ORDER_LITTLE_ENDIAN__")? {
        ByteOrder::Little
    } else if byte_order == macro_number(&macros, "__ORDER_BIG_ENDIAN__")? {
        ByteOrder::Big
    } else {
        return Err(FrontendError::Output("unsupported compiler byte order".into()));
    };
    let char_is_signed = integers
        .get(&IntegerKind::Char)
        .ok_or_else(|| FrontendError::Output("missing plain char type".into()))?
        .signed;
    if char_is_signed == macros.contains_key("__CHAR_UNSIGNED__") {
        return Err(FrontendError::Environment(
            "compiler plain-char signedness differs from libclang".into(),
        ));
    }
    let c_standard = if macros.contains_key("__STDC_VERSION__") {
        Some(macro_number(&macros, "__STDC_VERSION__")?)
    } else {
        None
    };
    let floating_point =
        floating_point_facts(&macros, floating_types, char_bits, evaluation_method, verbose)?;
    Ok(TargetFacts {
        triple,
        pointer_bits,
        function_pointer,
        size_type,
        offsetof_supported,
        char_bits,
        char_is_signed,
        ascii_execution_charset,
        byte_order,
        c_standard,
        integers,
        floating_point,
    })
}

/// Reconcile floating representation with effective compiler options so lowering can reject altered
/// arithmetic semantics.
fn floating_point_facts(
    macros: &BTreeMap<&str, &MacroDefinition>,
    layouts: BTreeMap<FloatingKind, (u64, Option<u64>)>,
    char_bits: u32,
    evaluation_method: i32,
    verbose: &str,
) -> Result<FloatingPointFacts, FrontendError> {
    let radix = u32::try_from(macro_number(macros, "__FLT_RADIX__")?)
        .map_err(|_| FrontendError::Output("invalid floating-point radix".into()))?;
    let mut types = BTreeMap::new();
    for (kind, prefix, sizeof) in [
        (FloatingKind::Float, "__FLT", "__SIZEOF_FLOAT__"),
        (FloatingKind::Double, "__DBL", "__SIZEOF_DOUBLE__"),
        (FloatingKind::LongDouble, "__LDBL", "__SIZEOF_LONG_DOUBLE__"),
    ] {
        let Some(&(size, alignment)) = layouts.get(&kind) else { continue };
        if macro_number(macros, sizeof)? != size {
            return Err(FrontendError::Environment(format!(
                "compiler {sizeof} differs from libclang type layout"
            )));
        }
        let storage_bits = size
            .checked_mul(u64::from(char_bits))
            .and_then(|bits| u32::try_from(bits).ok())
            .ok_or_else(|| FrontendError::Output("invalid floating-point width".into()))?;
        let mantissa_digits = u32::try_from(macro_number(macros, &format!("{prefix}_MANT_DIG__"))?)
            .map_err(|_| FrontendError::Output("invalid floating-point precision".into()))?;
        types.insert(
            kind,
            FloatingType {
                storage_bits,
                alignment,
                radix,
                mantissa_digits,
                min_exponent: macro_signed_number(macros, &format!("{prefix}_MIN_EXP__"))?,
                max_exponent: macro_signed_number(macros, &format!("{prefix}_MAX_EXP__"))?,
                has_subnormals: macro_number(macros, &format!("{prefix}_HAS_DENORM__"))? == 1,
                has_infinity: macro_number(macros, &format!("{prefix}_HAS_INFINITY__"))? == 1,
                has_quiet_nan: macro_number(macros, &format!("{prefix}_HAS_QUIET_NAN__"))? == 1,
            },
        );
    }
    let effective_options = frontend_arguments(verbose)?
        .into_iter()
        .filter(|option| {
            option.starts_with("-ffp-")
                || option.starts_with("-fexcess-precision=")
                || option.starts_with("-fdenormal-fp-math")
                || matches!(
                    option.as_str(),
                    "-frounding-math"
                        | "-fno-rounding-math"
                        | "-ffast-math"
                        | "-ffinite-math-only"
                        | "-funsafe-math-optimizations"
                        | "-fno-signed-zeros"
                        | "-freciprocal-math"
                        | "-fapprox-func"
                        | "-menable-no-nans"
                        | "-menable-no-infs"
                        | "-menable-unsafe-fp-math"
                        | "-fno-strict-float-cast-overflow"
                )
        })
        .collect::<Vec<_>>();
    let unsupported_options = effective_options
        .iter()
        .filter(|option| {
            !matches!(
                option.as_str(),
                "-ffp-contract=off"
                    | "-fno-rounding-math"
                    | "-ffp-exception-behavior=ignore"
                    | "-ffp-eval-method=source"
                    | "-fexcess-precision=standard"
                    | "-fdenormal-fp-math=ieee"
                    | "-fdenormal-fp-math=ieee,ieee"
                    | "-fdenormal-fp-math-f32=ieee"
                    | "-fdenormal-fp-math-f32=ieee,ieee"
            )
        })
        .cloned()
        .collect();
    Ok(FloatingPointFacts {
        types,
        evaluation_method: Some(evaluation_method),
        fast_math: macros.contains_key("__FAST_MATH__"),
        finite_math_only: macro_number(macros, "__FINITE_MATH_ONLY__")? != 0,
        effective_options,
        unsupported_options,
    })
}

/// Read a signed predefined numeric fact without losing the negative exponent or evaluation value.
fn macro_signed_number(
    macros: &BTreeMap<&str, &MacroDefinition>,
    name: &str,
) -> Result<i32, FrontendError> {
    let definition = macros
        .get(name)
        .ok_or_else(|| FrontendError::Output(format!("missing compiler macro {name}")))?;
    let tokens = definition
        .tokens
        .iter()
        .skip(1)
        .filter(|token| token.kind != TokenKind::Comment)
        .map(|token| token.spelling.as_str())
        .collect::<Vec<_>>();
    let tokens = match tokens.as_slice() {
        ["(", rest @ .., ")"] => rest,
        tokens => tokens,
    };
    let (negative, digits) = match tokens {
        [digits] => (false, *digits),
        ["-", digits] => (true, *digits),
        _ => {
            return Err(FrontendError::Output(format!(
                "compiler macro {name} is not a signed integer"
            )));
        }
    };
    let value = digits.trim_end_matches(['u', 'U', 'l', 'L']).parse::<i32>().map_err(|_| {
        FrontendError::Output(format!("compiler macro {name} has an invalid exponent"))
    })?;
    Ok(if negative { -value } else { value })
}

/// Extract the effective cc1 command from verbose output for semantic option auditing.
fn frontend_arguments(verbose: &str) -> Result<Vec<String>, FrontendError> {
    verbose
        .lines()
        .filter_map(shlex::split)
        .find(|arguments| arguments.iter().any(|argument| argument == "-cc1"))
        .ok_or_else(|| FrontendError::Output("missing effective Clang frontend arguments".into()))
}

/// Determine signed-overflow policy and unsupported semantic modes from recorded and effective
/// compiler options.
fn semantic_options(
    arguments: &[String],
    verbose: &str,
) -> Result<(SignedOverflow, Vec<String>), FrontendError> {
    let frontend = frontend_arguments(verbose)?;
    // The driver resolves option overrides. Clang's trap mode takes precedence over
    // wrapping; interpreting GCC's ordering rules here would produce a different policy.
    let overflow = if frontend.iter().any(|argument| argument == "-ftrapv") {
        SignedOverflow::Trapping
    } else if frontend.iter().any(|argument| argument == "-fwrapv") {
        SignedOverflow::Wrapping
    } else {
        SignedOverflow::Undefined
    };
    let mut unsupported = Vec::new();
    for argument in arguments {
        if argument.starts_with("-f")
            && !is_floating_point_option(argument)
            && !matches!(
                argument.as_str(),
                "-fwrapv"
                    | "-fno-wrapv"
                    | "-ftrapv"
                    | "-fno-trapv"
                    | "-fstrict-overflow"
                    | "-fno-strict-overflow"
                    | "-fno-strict-aliasing"
                    | "-fstrict-aliasing"
                    | "-fPIC"
                    | "-fpic"
                    | "-fPIE"
                    | "-fpie"
                    | "-fno-omit-frame-pointer"
                    | "-fomit-frame-pointer"
                    | "-fno-common"
                    | "-fcommon"
                    | "-ffunction-sections"
                    | "-fdata-sections"
                    | "-fno-asynchronous-unwind-tables"
                    | "-fasynchronous-unwind-tables"
                    | "-fno-unwind-tables"
                    | "-funwind-tables"
                    | "-fstack-protector"
                    | "-fstack-protector-strong"
                    | "-fstack-protector-all"
                    | "-fno-stack-protector"
                    // These add stack-page/control-flow instrumentation without
                    // changing C values, types or access qualification. Keep
                    // their original arguments for native helper compilation.
                    | "-fstack-clash-protection"
                    | "-fno-stack-clash-protection"
                    | "-fcf-protection"
                    | "-fcf-protection=full"
                    | "-fcf-protection=branch"
                    | "-fcf-protection=return"
                    | "-fcf-protection=none"
                    // LTO controls optimization and object representation, not
                    // C expression types or values. Native support still needs
                    // ordinary object code for the Rust linker's toolchain.
                    | "-flto"
                    | "-flto=full"
                    | "-flto=thin"
                    | "-flto=auto"
                    | "-flto=jobserver"
                    | "-fno-lto"
                    | "-ffat-lto-objects"
                    | "-fno-fat-lto-objects"
                    | "-funsigned-char"
                    | "-fsigned-char"
                    // Every enum's compatible integer, size and alignment is
                    // obtained from the compiler and reconciled with bindgen.
                    | "-fshort-enums"
                    | "-fno-short-enums"
            )
            || argument == "-Ofast"
        {
            unsupported.push(argument.clone());
        }
    }
    Ok((overflow, unsupported))
}

/// Identify flags that can alter floating arithmetic and therefore require explicit profile
/// reasoning.
fn is_floating_point_option(argument: &str) -> bool {
    argument.starts_with("-ffp-")
        || argument.starts_with("-fdenormal-fp-math")
        || argument.starts_with("-fexcess-precision=")
        || matches!(
            argument,
            "-ffast-math"
                | "-fno-fast-math"
                | "-ffinite-math-only"
                | "-fno-finite-math-only"
                | "-frounding-math"
                | "-fno-rounding-math"
                | "-funsafe-math-optimizations"
                | "-fno-unsafe-math-optimizations"
                | "-fassociative-math"
                | "-fno-associative-math"
                | "-freciprocal-math"
                | "-fno-reciprocal-math"
                | "-fsigned-zeros"
                | "-fno-signed-zeros"
                | "-fhonor-nans"
                | "-fno-honor-nans"
                | "-fhonor-infinities"
                | "-fno-honor-infinities"
                | "-fapprox-func"
                | "-fno-approx-func"
                | "-ftrapping-math"
                | "-fno-trapping-math"
                | "-fstrict-float-cast-overflow"
                | "-fno-strict-float-cast-overflow"
        )
}

/// Decode Clang's dependency filename quoting for later input tracking.
///
/// Clang emits native Windows separators without escaping ordinary backslashes. It
/// doubles dollars, escapes hashes, and doubles preceding backslashes only when
/// escaping a space. Decode that format instead of treating every backslash as a
/// shell escape; otherwise valid header paths lose their directory separators.
pub(crate) fn parse_dependencies(output: &str) -> Result<BTreeSet<PathBuf>, FrontendError> {
    let (_, paths) = output
        .split_once(':')
        .ok_or_else(|| FrontendError::Output("missing Clang dependency target".into()))?;
    let mut files = BTreeSet::new();
    let mut word = String::new();
    let mut characters = paths.chars().peekable();
    while let Some(character) = characters.next() {
        match character {
            '\\' => {
                let mut backslashes = 1;
                while characters.peek() == Some(&'\\') {
                    characters.next();
                    backslashes += 1;
                }
                match characters.peek().copied() {
                    Some(' ') if backslashes % 2 == 1 => {
                        word.extend(std::iter::repeat_n('\\', backslashes / 2));
                        word.push(' ');
                        characters.next();
                    }
                    Some('#') => {
                        word.extend(std::iter::repeat_n('\\', backslashes - 1));
                        word.push('#');
                        characters.next();
                    }
                    Some('\n') if backslashes == 1 && word.is_empty() => {
                        characters.next();
                    }
                    Some('\r') if backslashes == 1 && word.is_empty() => {
                        characters.next();
                        if characters.next() != Some('\n') {
                            return Err(FrontendError::Output(
                                "unfinished dependency line continuation".into(),
                            ));
                        }
                    }
                    Some(_) => word.extend(std::iter::repeat_n('\\', backslashes)),
                    None => {
                        return Err(FrontendError::Output("unfinished dependency escape".into()));
                    }
                }
            }
            '$' if characters.peek() == Some(&'$') => {
                characters.next();
                word.push('$');
            }
            character if character.is_whitespace() => {
                if !word.is_empty() {
                    files.insert(input_path(Path::new(&word))?);
                    word.clear();
                }
            }
            character => word.push(character),
        }
    }
    if !word.is_empty() {
        files.insert(input_path(Path::new(&word))?);
    }
    Ok(files)
}

/// Record files, fingerprints, search roots, and environment values that can change compiler
/// observations.
fn build_inputs(
    arguments: &[String],
    verbose: &str,
    mut files: BTreeSet<PathBuf>,
    compiler_search: CompilerSearch,
) -> Result<BuildInputs, FrontendError> {
    // Successful header-availability searches appear in dependency output even
    // when the header is never included. Track both its lookup spelling and
    // canonical identity, just as we do for actual inclusions.
    let identities = files.iter().map(|path| absolute_path(path)).collect::<Result<Vec<_>, _>>()?;
    files.extend(identities);
    let mut directories = BTreeSet::new();
    // Quoted includes search their including file's directory before -I directories.
    for path in &files {
        if let Some(parent) = path.parent() {
            directories.insert(parent.to_owned());
        }
    }
    let mut searching = false;
    for line in verbose.lines() {
        if line.contains("search starts here:") {
            searching = true;
            continue;
        }
        if line.trim() == "End of search list." {
            searching = false;
            continue;
        }
        if searching {
            let path = line.trim().trim_end_matches(" (framework directory)");
            directories.insert(absolute_path(Path::new(path))?);
        }
    }
    let mut arguments = arguments.iter();
    while let Some(argument) = arguments.next() {
        let directory = if matches!(
            argument.as_str(),
            "-I" | "-isystem"
                | "-iquote"
                | "-idirafter"
                | "-F"
                | "-iframework"
                | "-isysroot"
                | "--sysroot"
                | "-resource-dir"
        ) {
            arguments.next().map(String::as_str)
        } else {
            [
                "-isystem",
                "-iquote",
                "-idirafter",
                "-iframework",
                "-I",
                "-F",
                "--sysroot=",
                "-resource-dir=",
            ]
            .into_iter()
            .find_map(|prefix| argument.strip_prefix(prefix).filter(|value| !value.is_empty()))
        };
        if let Some(directory) = directory {
            let path = Path::new(directory);
            let path = if path.is_absolute() {
                path.to_owned()
            } else {
                std::env::current_dir()
                    .map_err(|source| FrontendError::CompilerIo { compiler: path.into(), source })?
                    .join(path)
            };
            // Keep absent search roots: creating them can change include resolution.
            if let Ok(identity) = path.canonicalize() {
                directories.insert(identity);
            }
            directories.insert(path);
        }
    }
    files.extend(compiler_search.files);
    directories.extend(compiler_search.required_directories);
    let environment = [
        "PATH",
        "CLANG_PATH",
        "LIBCLANG_PATH",
        "LLVM_CONFIG_PATH",
        "SDKROOT",
        "MACOSX_DEPLOYMENT_TARGET",
        "CPATH",
        "C_INCLUDE_PATH",
        "CPLUS_INCLUDE_PATH",
        "INCLUDE",
    ]
    .into_iter()
    .map(|name| {
        let value = match std::env::var(name) {
            Ok(value) => Some(value),
            Err(std::env::VarError::NotPresent) => None,
            Err(std::env::VarError::NotUnicode(_)) => {
                return Err(FrontendError::Environment(format!(
                    "{name} is not UTF-8 and cannot be recorded"
                )));
            }
        };
        Ok((name.into(), value))
    })
    .collect::<Result<_, _>>()?;
    let fingerprints = fingerprint_files(files.iter())?;
    Ok(BuildInputs {
        current_directory: std::env::current_dir().map_err(|error| {
            FrontendError::Environment(format!("could not record working directory: {error}"))
        })?,
        files: files.into_iter().collect(),
        fingerprints,
        directories: directories.into_iter().collect(),
        executable_search_directories: compiler_search.directories.into_iter().collect(),
        environment,
    })
}

/// Snapshot file bytes, following each requested spelling so symlink replacement is observed.
pub(crate) fn fingerprint_files<'a>(
    files: impl IntoIterator<Item = &'a PathBuf>,
) -> Result<BTreeMap<PathBuf, Option<String>>, FrontendError> {
    let mut fingerprints = BTreeMap::new();
    let mut buffer = [0u8; 65536];
    for path in files {
        let mut file = match std::fs::File::open(path) {
            Ok(file) => file,
            Err(error) if error.kind() == io::ErrorKind::NotFound => {
                fingerprints.insert(path.clone(), None);
                continue;
            }
            Err(source) => {
                return Err(FrontendError::CompilerIo { compiler: path.clone(), source });
            }
        };
        let mut digest = Sha256::new();
        loop {
            let length = file
                .read(&mut buffer)
                .map_err(|source| FrontendError::CompilerIo { compiler: path.clone(), source })?;
            if length == 0 {
                break;
            }
            digest.update(&buffer[..length]);
        }
        use std::fmt::Write as _;
        let mut fingerprint = String::with_capacity(64);
        for byte in digest.finalize() {
            write!(&mut fingerprint, "{byte:02x}").expect("writing to a String cannot fail");
        }
        fingerprints.insert(path.clone(), Some(fingerprint));
    }
    Ok(fingerprints)
}

/// Reject file-content changes after inspection before later phases combine stale and fresh facts.
pub(crate) fn verify_input_files(inputs: &BuildInputs) -> Result<(), FrontendError> {
    let actual = fingerprint_files(inputs.fingerprints.keys())?;
    if let Some((path, _)) =
        actual.iter().find(|(path, digest)| inputs.fingerprints.get(*path) != Some(*digest))
    {
        return Err(FrontendError::Environment(format!(
            "{} changed after inspection",
            path.display()
        )));
    }
    Ok(())
}

// Read stdout and stderr concurrently so either pipe can fill without blocking Clang.
// The channel also lets an output limit stop the child without waiting for a timeout.
/// Run a driver phase with bounded concurrent output capture so either pipe cannot deadlock the
/// other.
pub(crate) fn run_compiler(
    compiler: &Path,
    arguments: &[String],
) -> Result<CompilerOutput, FrontendError> {
    run_compiler_with_input(compiler, arguments, None)
}

/// Compile and archive generated access primitives under the inspected C flags.
/// The source must include the same inspected header; callers retain the source
/// and archive alongside their other generated bindings. Both tools have the
/// frontend's bounded diagnostics and timeout, and failed children are reaped.
/// Fresh staged outputs are checked before publication; publication errors can
/// leave the object updated without replacing the archive.
/// Preprocess with the original flags before compiling the fixed token stream
/// as position-independent machine code. This preserves header branches that
/// depend on PIC/PIE predefines even when native backend flags differ from the
/// inspected mode, and prevents forced includes from running a second time.
/// Backend output flags keep the archive independent of Clang's LLVM version
/// and allow the linker to discard unused PostgreSQL entry points.
pub fn compile_native_support(
    profile: &CompilationProfile,
    source: &Path,
    object: &Path,
    archive: &Path,
) -> Result<(), FrontendError> {
    validate_arguments(&profile.arguments)?;
    let path = |path: &Path| {
        path.to_str().map(str::to_owned).ok_or_else(|| {
            FrontendError::Output(format!("native support path is not UTF-8: {}", path.display()))
        })
    };
    let source_identity = std::fs::canonicalize(source)
        .map_err(|error| FrontendError::NativeIo { path: source.into(), source: error })?;
    let object_identity = native_output_identity(object)?;
    let archive_identity = native_output_identity(archive)?;
    if source_identity == object_identity
        || source_identity == archive_identity
        || object_identity == archive_identity
    {
        return Err(FrontendError::Arguments(
            "native source, object and archive must have distinct paths".into(),
        ));
    }
    let stage = |destination: &Path| {
        let parent = destination.parent().ok_or_else(|| {
            FrontendError::Arguments(format!(
                "native output has no parent directory: {}",
                destination.display()
            ))
        })?;
        tempfile::Builder::new()
            .prefix(".pgrx-native-")
            .tempdir_in(parent)
            .map_err(|source| FrontendError::NativeIo { path: parent.into(), source })
    };
    let object_stage = stage(&object_identity)?;
    let archive_stage = stage(&archive_identity)?;
    let staged_object = object_stage
        .path()
        .join(object.file_name().expect("native output identity validated its filename"));
    let staged_archive = archive_stage.path().join("support.a");
    // Keep the input beside the fixed archive name, separate from the object
    // staging directory whose filename is chosen by the caller.
    let staged_source = archive_stage.path().join("support.i");
    let mut preprocessing = profile.arguments.clone();
    preprocessing.extend(["-x".into(), "c".into(), "-E".into(), path(source)?]);
    let expanded = run_compiler(&profile.compiler.executable, &preprocessing)?;
    std::fs::write(&staged_source, expanded.stdout)
        .map_err(|source| FrontendError::NativeIo { path: staged_source.clone(), source })?;
    let mut args = profile.arguments.clone();
    args.extend([
        "-x".into(),
        "cpp-output".into(),
        // Preprocessor-only flags already took effect. Clang ignores them for
        // cpp-output; suppress its unused-argument diagnostic even with -Werror.
        "-Qunused-arguments".into(),
        "-c".into(),
        "-fPIC".into(),
        "-fno-lto".into(),
        "-ffunction-sections".into(),
        "-fdata-sections".into(),
        path(&staged_source)?,
        "-o".into(),
        path(&staged_object)?,
    ]);
    run_compiler(&profile.compiler.executable, &args)?;
    require_native_artifact(&staged_object, "object")?;
    let adjacent = profile.compiler.executable.with_file_name("llvm-ar");
    let archiver = if adjacent.is_file() { adjacent } else { PathBuf::from("ar") };
    run_compiler(&archiver, &["crs".into(), path(&staged_archive)?, path(&staged_object)?])?;
    require_native_artifact(&staged_archive, "archive")?;
    std::fs::rename(&staged_object, object)
        .map_err(|source| FrontendError::NativeIo { path: object.into(), source })?;
    std::fs::rename(&staged_archive, archive)
        .map_err(|source| FrontendError::NativeIo { path: archive.into(), source })?;
    Ok(())
}

/// Resolve output identities, including absent filenames, to reject source/object/archive aliasing.
fn native_output_identity(path: &Path) -> Result<PathBuf, FrontendError> {
    let name = path.file_name().ok_or_else(|| {
        FrontendError::Arguments(format!("native output has no filename: {}", path.display()))
    })?;
    std::fs::canonicalize(path)
        .or_else(|error| {
            if error.kind() != io::ErrorKind::NotFound {
                return Err(error);
            }
            let parent = path.parent().filter(|parent| !parent.as_os_str().is_empty());
            std::fs::canonicalize(parent.unwrap_or_else(|| Path::new(".")))
                .map(|parent| parent.join(name))
        })
        .map_err(|source| FrontendError::NativeIo { path: path.into(), source })
}

/// Require a fresh nonempty regular output before native generation accepts a tool invocation as
/// successful.
fn require_native_artifact(path: &Path, kind: &str) -> Result<(), FrontendError> {
    let metadata = std::fs::symlink_metadata(path).map_err(|error| {
        FrontendError::Output(format!("native tool did not produce its {kind}: {error}"))
    })?;
    if !metadata.is_file() || metadata.len() == 0 {
        return Err(FrontendError::Output(format!(
            "native tool did not produce a nonempty regular {kind}"
        )));
    }
    Ok(())
}

/// Feed an optional owned probe to the driver while enforcing output/time limits and reaping failed
/// children.
fn run_compiler_with_input(
    compiler: &Path,
    arguments: &[String],
    source: Option<String>,
) -> Result<CompilerOutput, FrontendError> {
    let failure = |source| FrontendError::CompilerIo { compiler: compiler.into(), source };
    let mut child = RunningCompiler(
        Command::new(compiler)
            .args(arguments)
            .env("LC_ALL", "C")
            .stdin(if source.is_some() { Stdio::piped() } else { Stdio::null() })
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(failure)?,
    );
    if let Some(source) = source {
        let mut stdin = child.0.stdin.take().expect("piped stdin is present after spawning");
        // The writer owns its bounded probe and pipe. Killing/reaping the compiler on an
        // error or timeout also closes the reader; no borrowing crosses this thread.
        thread::Builder::new()
            .name("pgrx-c-macros-stdin".into())
            .spawn(move || {
                let _ = stdin.write_all(source.as_bytes());
            })
            .map_err(failure)?;
    }
    let stdout = child.0.stdout.take().expect("piped stdout is present after spawning");
    let stderr = child.0.stderr.take().expect("piped stderr is present after spawning");
    let (sender, receiver) = mpsc::channel();
    start_reader(stdout, true, sender.clone()).map_err(failure)?;
    start_reader(stderr, false, sender.clone()).map_err(failure)?;
    drop(sender);
    let started = Instant::now();
    let mut stdout = None;
    let mut stderr = None;
    let mut status = None;
    'result: loop {
        while let Ok((is_stdout, result)) = receiver.try_recv() {
            let bytes = match result {
                Ok(bytes) => bytes,
                Err(error) => break 'result Err(failure(error)),
            };
            if bytes.len() as u64 > OUTPUT_LIMIT {
                break 'result Err(FrontendError::OutputLimit(compiler.into()));
            }
            let text = String::from_utf8(bytes).map_err(|error| {
                FrontendError::Output(format!(
                    "{} emitted non-UTF-8 output: {error}",
                    compiler.display()
                ))
            });
            match text {
                Ok(text) if is_stdout => stdout = Some(text),
                Ok(text) => stderr = Some(text),
                Err(error) => break 'result Err(error),
            }
        }
        if status.is_none() {
            match child.0.try_wait() {
                Ok(exit) => status = exit,
                Err(error) => break Err(failure(error)),
            }
        }
        if let (Some(status), true, true) = (status, stdout.is_some(), stderr.is_some()) {
            let stdout = stdout.take().expect("the completed stdout reader supplied its result");
            let stderr = stderr.take().expect("the completed stderr reader supplied its result");
            break if status.success() {
                Ok(CompilerOutput { stdout, stderr })
            } else {
                Err(FrontendError::CompilerFailed {
                    compiler: compiler.into(),
                    status,
                    diagnostics: stderr,
                })
            };
        }
        if started.elapsed() >= COMPILER_TIMEOUT {
            break Err(FrontendError::Timeout(compiler.into()));
        }
        thread::sleep(Duration::from_millis(10));
    }
}

/// Own a child process so early returns, diagnostic failures, and time limits still kill and reap it.
struct RunningCompiler(
    /// The child process owned until completion or explicit kill/reap on failure.
    Child,
);

/// Release resources owned by RunningCompiler even when a compiler or proof phase exits early.
impl Drop for RunningCompiler {
    /// Kill and reap a still-running compiler so errors and time limits do not leave owned child
    /// processes behind.
    fn drop(&mut self) {
        if self.0.try_wait().ok().flatten().is_none() {
            let _ = self.0.kill();
            let _ = self.0.wait();
        }
    }
}

/// Drain one compiler pipe on an owned thread with a byte budget and report its result to the
/// coordinator.
fn start_reader(
    stream: impl Read + Send + 'static,
    is_stdout: bool,
    sender: mpsc::Sender<(bool, io::Result<Vec<u8>>)>,
) -> io::Result<()> {
    thread::Builder::new()
        .spawn(move || {
            let mut bytes = Vec::new();
            let result = stream.take(OUTPUT_LIMIT + 1).read_to_end(&mut bytes).map(|_| bytes);
            let _ = sender.send((is_stdout, result));
        })
        .map(|_| ())
}

/// Extract the Clang release major used to require a driver compatible with the loaded library.
pub(crate) fn version_major(version: &str) -> Option<u32> {
    let (_, version) = version.split_once("clang version ")?;
    version.split('.').next()?.parse().ok()
}

/// Reject automatically loaded driver configuration whose unrecorded flags would defeat reproducible
/// analysis.
pub(crate) fn validate_driver_configuration(verbose: &str) -> Result<(), FrontendError> {
    if let Some(configuration) =
        verbose.lines().find(|line| line.trim_start().starts_with("Configuration file:"))
    {
        return Err(FrontendError::Environment(format!(
            "automatic Clang configuration is unsupported for reproducible analysis: {}",
            configuration.trim()
        )));
    }
    Ok(())
}

/// Reject extra inputs, driver actions, and hidden configuration that could change or bypass the
/// inspected C phase.
fn validate_arguments(arguments: &[String]) -> Result<(), FrontendError> {
    let mut arguments = arguments.iter();
    while let Some(argument) = arguments.next() {
        if argument.contains('\0') {
            return Err(FrontendError::Arguments("an argument contains a NUL byte".into()));
        }
        if argument == "-x" {
            match arguments.next().map(String::as_str) {
                Some("c" | "c-header") => continue,
                _ => {
                    return Err(FrontendError::Arguments(
                        "analysis requires C language mode".into(),
                    ));
                }
            }
        }
        if VALUE_OPTIONS.contains(&argument.as_str()) {
            match arguments.next() {
                Some(value) if !value.contains('\0') => {
                    if argument == "-D" || argument == "-U" {
                        validate_fact_override(value)?;
                    }
                    continue;
                }
                _ => {
                    return Err(FrontendError::Arguments(format!(
                        "{argument} requires a value without NUL bytes"
                    )));
                }
            }
        }
        if let Some(value) = argument.strip_prefix("-D").or_else(|| argument.strip_prefix("-U")) {
            validate_fact_override(value)?;
        }
        if !argument.starts_with('-') || argument == "-" {
            return Err(FrontendError::Arguments(format!(
                "additional input {argument} is unsupported"
            )));
        }
        if argument.starts_with("-x") && !matches!(argument.as_str(), "-xc" | "-xc-header") {
            return Err(FrontendError::Arguments("analysis requires C language mode".into()));
        }
        if matches!(
            argument.as_str(),
            "-c" | "-S"
                | "-E"
                | "-fsyntax-only"
                | "-M"
                | "-MM"
                | "-MD"
                | "-MMD"
                | "-Xclang"
                | "-Xpreprocessor"
                | "-cc1"
                | "--"
        ) || argument.starts_with('@')
            || argument.starts_with("-o")
            || argument.starts_with("--output")
            || argument.starts_with("-MF")
            || argument.starts_with("-MJ")
            || argument.starts_with("-MT")
            || argument.starts_with("-MQ")
            || argument.starts_with("-save-temps")
            || argument.starts_with("--save-temps")
            || argument.starts_with("-fplugin")
            || argument.starts_with("-fmodules")
            || argument.starts_with("-fmodule-")
            || argument.starts_with("-include-pch")
            || argument.starts_with("-working-directory")
            || argument.starts_with("-Wp,")
            || argument.starts_with("-Xclang")
            || argument.starts_with("-Xpreprocessor")
            || argument == "--config"
            || argument.starts_with("--config=")
        {
            return Err(FrontendError::Arguments(format!(
                "{argument} is unsupported for reproducible analysis"
            )));
        }
    }
    Ok(())
}

/// Prevent -D/-U options from forging protected target macros that serve as semantic witnesses.
fn validate_fact_override(value: &str) -> Result<(), FrontendError> {
    let name = value
        .split(|character: char| !character.is_ascii_alphanumeric() && character != '_')
        .next()
        .unwrap_or_default();
    let protected = matches!(
        name,
        "__CHAR_BIT__"
            | "__CHAR_UNSIGNED__"
            | "__BYTE_ORDER__"
            | "__ORDER_LITTLE_ENDIAN__"
            | "__ORDER_BIG_ENDIAN__"
            | "__ORDER_PDP_ENDIAN__"
            | "__STDC__"
            | "__STDC_VERSION__"
            | "__STDC_HOSTED__"
            | "__STRICT_ANSI__"
            | "__SCHAR_MAX__"
            | "__SHRT_MAX__"
            | "__INT_MAX__"
            | "__LONG_MAX__"
            | "__LONG_LONG_MAX__"
            | "__SIZE_TYPE__"
            | "__PTRDIFF_TYPE__"
            | "__INTMAX_TYPE__"
            | "__UINTMAX_TYPE__"
            | "__INTPTR_TYPE__"
            | "__UINTPTR_TYPE__"
            | "__WCHAR_TYPE__"
            | "__WINT_TYPE__"
    ) || name.starts_with("__SIZEOF_")
        || name.starts_with("__pgrx_c_")
        || (name.ends_with("_TYPE__") && (name.starts_with("__INT") || name.starts_with("__UINT")));
    if protected {
        return Err(FrontendError::Arguments(format!(
            "{name} is a compiler target fact; use compiler target/language options instead of macro overrides"
        )));
    }
    Ok(())
}

/// Exercise this phase’s semantic boundaries with owned fixtures.
/// These regressions check accepted proofs and explicit refusals without changing production
/// headers or weakening the C identity and evaluation contracts.
#[cfg(test)]
mod tests {
    use super::*;

    /// Preserve native drive, UNC, and ordinary backslash spellings in Clang dependencies.
    #[test]
    fn dependency_paths_retain_native_backslash_separators() {
        let paths = [
            r"D:\a\pgrx\pgrx\pgrx-c-macros\tests\fixtures\field_adapters.h",
            r"\\server\share\postgres\header.h",
            r"/tmp/literal\backslash.h",
        ];
        let output = format!("pgrx_c_macros: {}\n", paths.join(" "));
        let actual = parse_dependencies(&output).unwrap();
        let expected = paths
            .into_iter()
            .map(|path| input_path(Path::new(path)).unwrap())
            .collect::<BTreeSet<_>>();
        assert_eq!(actual, expected);
    }

    /// Undo only Clang's filename escapes, including separators immediately before spaces or hashes.
    #[test]
    fn dependency_paths_decode_clang_space_hash_and_dollar_quoting() {
        let output = concat!(
            r"pgrx_c_macros: C:\Program\ Files\postgres.h ",
            r"C:\root\\\ leading.h ",
            r"C:\root\\#header.h ",
            r"/tmp/dollar$$name.h ",
            r"/tmp/literal\\slashes.h ",
            r"/tmp/many\\\\\ spaces.h ",
            r"/tmp/backslash\$$dollar.h",
            "\n",
        );
        let paths = [
            r"C:\Program Files\postgres.h",
            r"C:\root\ leading.h",
            r"C:\root\#header.h",
            r"/tmp/dollar$name.h",
            r"/tmp/literal\\slashes.h",
            r"/tmp/many\\ spaces.h",
            r"/tmp/backslash\$dollar.h",
        ];
        let actual = parse_dependencies(output).unwrap();
        let expected = paths
            .into_iter()
            .map(|path| input_path(Path::new(path)).unwrap())
            .collect::<BTreeSet<_>>();
        assert_eq!(actual, expected);
    }

    /// Accept wrapped LF/CRLF dependency lists without treating filename backslashes as continuations.
    #[test]
    fn dependency_line_continuations_are_separate_from_filename_backslashes() {
        let output = "pgrx_c_macros: first.h \\\n second.h \\\r\n third.h\n";
        let actual = parse_dependencies(output).unwrap();
        let expected = ["first.h", "second.h", "third.h"]
            .into_iter()
            .map(|path| input_path(Path::new(path)).unwrap())
            .collect::<BTreeSet<_>>();
        assert_eq!(actual, expected);

        let actual = parse_dependencies("pgrx_c_macros: /tmp/ends\\\n").unwrap();
        assert_eq!(actual, BTreeSet::from([input_path(Path::new(r"/tmp/ends\")).unwrap()]));
    }

    /// Refuse incomplete dependency structure instead of tracking a truncated or fabricated input.
    #[test]
    fn dependency_output_rejects_missing_target_and_unfinished_continuations() {
        for output in ["missing-target.h", "pgrx_c_macros: \\", "pgrx_c_macros: \\\rbroken.h"] {
            assert!(matches!(parse_dependencies(output), Err(FrontendError::Output(_))));
        }
    }

    /// Checks optional offsetof failures cannot hide required fact errors.
    #[test]
    fn optional_offsetof_failures_cannot_hide_required_fact_errors() {
        assert!(only_offsetof_errors(
            "pgrx_c_offsetof_proof:7:1: error: static assertion failed: pgrx_offsetof_layout\n1 error generated.\n"
        ));
        assert!(!only_offsetof_errors(
            "<stdin>:2:1: error: static assertion failed: pgrx_size_type\npgrx_c_offsetof_proof:7:1: error: static assertion failed: pgrx_offsetof_layout\n2 errors generated.\n"
        ));
        assert!(!only_offsetof_errors("clang: error: unable to execute command\n"));
        assert!(!only_offsetof_errors(
            "pgrx_c_offsetof_proof:7:1: fatal error: internal compiler failure\n"
        ));
        assert!(!only_offsetof_errors(""));
    }

    /// Checks protection codegen options require exact reviewed spellings.
    #[test]
    fn protection_codegen_options_require_exact_reviewed_spellings() {
        let verbose = "clang -cc1 -fwrapv\n";
        for argument in [
            "-fstack-clash-protection",
            "-fno-stack-clash-protection",
            "-fcf-protection",
            "-fcf-protection=full",
            "-fcf-protection=branch",
            "-fcf-protection=return",
            "-fcf-protection=none",
        ] {
            let (overflow, unsupported) = semantic_options(&[argument.into()], verbose).unwrap();
            assert_eq!(overflow, SignedOverflow::Wrapping);
            assert!(unsupported.is_empty(), "reviewed codegen option: {argument}");
        }
        for argument in [
            "-fstack-clash-protection=other",
            "-fno-stack-clash-protection-other",
            "-fcf-protection=check",
            "-fcf-protection=unknown",
            "-fno-cf-protection",
            "-fstack-check",
            "-fpack-struct=1",
        ] {
            let (_, unsupported) = semantic_options(&[argument.into()], verbose).unwrap();
            assert_eq!(unsupported, [argument], "unreviewed/ABI option must stay gated");
        }
    }

    /// Check that reviewed LTO representation flags do not reject an otherwise supported C profile.
    #[test]
    fn lto_codegen_options_require_exact_reviewed_spellings() {
        let verbose = "clang -cc1 -fwrapv -flto=full\n";
        for argument in [
            "-flto",
            "-flto=full",
            "-flto=thin",
            "-flto=auto",
            "-flto=jobserver",
            "-fno-lto",
            "-ffat-lto-objects",
            "-fno-fat-lto-objects",
        ] {
            let (overflow, unsupported) = semantic_options(&[argument.into()], verbose).unwrap();
            assert_eq!(overflow, SignedOverflow::Wrapping);
            assert!(unsupported.is_empty(), "reviewed LTO option: {argument}");
        }
        for argument in ["-flto=unknown", "-ffat-lto-objects=unknown", "-fwhole-program"] {
            let (_, unsupported) = semantic_options(&[argument.into()], verbose).unwrap();
            assert_eq!(unsupported, [argument], "unreviewed option must stay gated");
        }
    }

    /// Exercise driver preference and change-sensitive filesystem searches with isolated synthetic
    /// executables.
    #[cfg(unix)]
    mod compiler_selection {
        use super::*;
        use std::fs;
        use std::os::unix::fs::PermissionsExt;
        use std::sync::atomic::{AtomicU64, Ordering};

        /// Give compiler-selection tests unique owned temporary directory names.
        static NEXT_FIXTURE: AtomicU64 = AtomicU64::new(0);

        /// Checks creating an adjacent compiler invalidates the previous path selection.
        #[test]
        fn creating_an_adjacent_compiler_invalidates_the_previous_path_selection() {
            let fixture = Fixture::new();
            let adjacent = fixture.0.join("runtime/bin/clang");
            fs::create_dir_all(adjacent.parent().unwrap()).unwrap();
            let roots = [
                fixture.0.join("early/bin"),
                fixture.0.join("chosen/bin"),
                fixture.0.join("unused/bin"),
            ];
            for root in &roots {
                fs::create_dir_all(root).unwrap();
            }
            let path_compiler = roots[1].join("clang-21");
            compiler(&path_compiler, 21);
            let selection = select_compiler(None, None, Some(&adjacent), 21, &roots).unwrap();
            assert_eq!(selection.executable, path_compiler);
            assert_eq!(
                selection.search.directories,
                BTreeSet::from([roots[0].clone(), roots[1].clone()])
            );
            let inputs = build_inputs(&[], "", BTreeSet::new(), selection.search).unwrap();
            assert!(inputs.files.contains(&adjacent));
            assert_eq!(inputs.fingerprints[&adjacent], None);
            assert!(inputs.fingerprints[&path_compiler].is_some());
            assert!(!inputs.executable_search_directories.contains(&roots[2]));

            compiler(&adjacent, 21);
            let selection = select_compiler(None, None, Some(&adjacent), 21, &roots).unwrap();
            assert_eq!(selection.executable, adjacent);
            assert!(
                selection.search.directories.is_empty(),
                "a matching adjacent tool needs no PATH search"
            );
            assert!(selection.search.files.contains(&adjacent));
            assert!(!selection.search.files.contains(&path_compiler));
        }

        /// Checks replacing a rejected configured compiler changes preference and keeps aliases.
        #[test]
        fn replacing_a_rejected_configured_compiler_changes_preference_and_keeps_aliases() {
            let fixture = Fixture::new();
            let actual = fixture.0.join("configured-clang");
            let configured = fixture.0.join("configured-alias");
            let adjacent = fixture.0.join("adjacent-clang");
            compiler(&actual, 19);
            std::os::unix::fs::symlink(&actual, &configured).unwrap();
            compiler(&adjacent, 21);
            let selection =
                select_compiler(None, Some(&configured), Some(&adjacent), 21, &[]).unwrap();
            assert_eq!(selection.executable, adjacent);
            assert!(selection.search.files.contains(&configured));
            assert!(selection.search.files.contains(&actual.canonicalize().unwrap()));
            let inputs = build_inputs(&[], "", BTreeSet::new(), selection.search).unwrap();
            let rejected_identity = inputs.fingerprints[&configured].clone();
            assert!(rejected_identity.is_some());

            compiler(&actual, 21);
            let selection =
                select_compiler(None, Some(&configured), Some(&adjacent), 21, &[]).unwrap();
            assert_eq!(selection.executable, configured, "replay must retain the requested alias");
            assert!(
                !selection.search.files.contains(&adjacent),
                "lower-priority tools were not examined"
            );
            let inputs = build_inputs(&[], "", BTreeSet::new(), selection.search).unwrap();
            assert_ne!(inputs.fingerprints[&configured], rejected_identity);
        }

        /// Checks bare configured names keep path precedence and explicit failures stay strict.
        #[test]
        fn bare_configured_names_keep_path_precedence_and_explicit_failures_stay_strict() {
            let fixture = Fixture::new();
            let roots = [fixture.0.join("first"), fixture.0.join("later")];
            for root in &roots {
                fs::create_dir(root).unwrap();
            }
            let first = roots[0].join("configured-clang");
            let later = roots[1].join("configured-clang");
            let adjacent = fixture.0.join("adjacent-clang");
            compiler(&first, 19);
            compiler(&later, 21);
            compiler(&adjacent, 21);
            let hint = Path::new("configured-clang");
            let selection = select_compiler(None, Some(hint), Some(&adjacent), 21, &roots).unwrap();
            assert_eq!(selection.executable, adjacent);
            assert!(selection.search.files.contains(&first));
            assert!(!selection.search.files.contains(&later));
            assert_eq!(selection.search.directories, BTreeSet::from([roots[0].clone()]));

            compiler(&first, 21);
            let selection = select_compiler(None, Some(hint), Some(&adjacent), 21, &roots).unwrap();
            assert_eq!(selection.executable, first);
            assert_eq!(selection.search.directories, BTreeSet::from([roots[0].clone()]));
            let missing = fixture.0.join("missing-explicit-clang");
            assert!(matches!(
                select_compiler(Some(&missing), Some(hint), Some(&adjacent), 21, &roots),
                Err(FrontendError::CompilerUnavailable(_))
            ));
        }

        /// Checks replacing a direct directory hint keeps it distinct from path lookups.
        #[test]
        fn replacing_a_direct_directory_hint_keeps_it_distinct_from_path_lookups() {
            let fixture = Fixture::new();
            let actual = fixture.0.join("compiler-placeholder");
            let configured = fixture.0.join("configured-alias");
            let adjacent = fixture.0.join("adjacent-clang");
            fs::create_dir(&actual).unwrap();
            std::os::unix::fs::symlink(&actual, &configured).unwrap();
            compiler(&adjacent, 21);
            let selection =
                select_compiler(None, Some(&configured), Some(&adjacent), 21, &[]).unwrap();
            assert_eq!(selection.executable, adjacent);
            assert!(selection.search.directories.is_empty());
            let inputs = build_inputs(&[], "", BTreeSet::new(), selection.search).unwrap();
            assert!(inputs.directories.contains(&configured));
            assert!(inputs.directories.contains(&actual.canonicalize().unwrap()));
            assert!(inputs.executable_search_directories.is_empty());
            assert!(
                !inputs.files.contains(&configured),
                "directory hints cannot be fingerprinted as files"
            );

            fs::remove_dir(&actual).unwrap();
            compiler(&actual, 21);
            let selection =
                select_compiler(None, Some(&configured), Some(&adjacent), 21, &[]).unwrap();
            assert_eq!(selection.executable, configured);
            assert!(selection.search.required_directories.is_empty());
            assert!(selection.search.files.contains(&configured));
            assert!(selection.search.files.contains(&actual.canonicalize().unwrap()));
        }

        /// Create a synthetic executable whose version response exercises driver preference and
        /// invalidation without invoking Clang.
        fn compiler(path: &Path, major: u32) {
            fs::write(path, format!("#!/bin/sh\nprintf '%s\\n' 'clang version {major}.0.0'\n"))
                .unwrap();
            let mut permissions = fs::metadata(path).unwrap().permissions();
            permissions.set_mode(0o755);
            fs::set_permissions(path, permissions).unwrap();
        }

        /// Own an isolated synthetic compiler directory for preference and rebuild-invalidation
        /// tests.
        struct Fixture(
            /// The isolated synthetic compiler directory owned by this test fixture.
            PathBuf,
        );

        /// Create uniquely owned synthetic tool trees for driver-selection regression tests.
        impl Fixture {
            /// Create a uniquely owned compiler-selection fixture directory so tests do not alter
            /// configured installations.
            fn new() -> Self {
                let number = NEXT_FIXTURE.fetch_add(1, Ordering::Relaxed);
                for attempt in 0..64 {
                    let path = std::env::temp_dir().join(format!(
                        "pgrx-compiler-choice-{}-{number}-{attempt}",
                        std::process::id()
                    ));
                    match fs::create_dir(&path) {
                        Ok(()) => return Self(path),
                        Err(error) if error.kind() == io::ErrorKind::AlreadyExists => continue,
                        Err(error) => panic!("could not create compiler fixture: {error}"),
                    }
                }
                panic!("could not reserve a compiler fixture directory");
            }
        }

        /// Release resources owned by Fixture even when a compiler or proof phase exits early.
        impl Drop for Fixture {
            /// Remove only the synthetic compiler directory owned by this fixture.
            fn drop(&mut self) {
                let _ = fs::remove_dir_all(&self.0);
            }
        }
    }

    /// Checks compiler target facts cannot be forged by macro options.
    #[test]
    fn compiler_target_facts_cannot_be_forged_by_macro_options() {
        for arguments in [
            vec!["-D__BYTE_ORDER__=__ORDER_BIG_ENDIAN__"],
            vec!["-D", "__STDC_VERSION__=202311L"],
            vec!["-U__CHAR_UNSIGNED__"],
            vec!["-U", "__SIZEOF_POINTER__"],
            vec!["-D__INT_MAX__(x)=x"],
            vec!["-D__UINT64_TYPE__=unsigned_char"],
            vec!["-D__pgrx_c_int=forged"],
            vec!["-Wp,-D__STDC_VERSION__=202311L"],
            vec!["-Wp,-D__BYTE_ORDER__=__ORDER_BIG_ENDIAN__"],
            vec!["-Xclang=-D", "-Xclang=__STDC_VERSION__=202311L"],
            vec!["-Xpreprocessor=-D__BYTE_ORDER__=__ORDER_BIG_ENDIAN__"],
            vec!["--config=/tmp/analysis.cfg"],
            vec!["--config", "/tmp/analysis.cfg"],
        ] {
            let arguments = arguments.into_iter().map(str::to_owned).collect::<Vec<_>>();
            assert!(
                matches!(validate_arguments(&arguments), Err(FrontendError::Arguments(_))),
                "{arguments:?}"
            );
        }
        validate_arguments(&[
            "-DUSER_FLAG=1".into(),
            "-UUSER_FLAG".into(),
            "-funsigned-char".into(),
            "-std=c11".into(),
        ])
        .unwrap();
    }

    /// Preserve a cross GCC driver's name as one option value while rejecting missing operands,
    /// embedded NUL bytes, and unrelated extra inputs.
    #[test]
    fn cross_gcc_driver_names_are_option_values_not_analysis_inputs() {
        validate_arguments(&[
            "-target".into(),
            "aarch64-unknown-linux-gnu".into(),
            "-ccc-gcc-name".into(),
            "aarch64-linux-gnu-gcc".into(),
        ])
        .unwrap();
        for arguments in [
            vec!["-ccc-gcc-name"],
            vec!["-ccc-gcc-name", "aarch64-linux-gnu-gcc\0"],
            vec!["-ccc-gcc-name", "aarch64-linux-gnu-gcc", "extra.c"],
        ] {
            let arguments = arguments.into_iter().map(str::to_owned).collect::<Vec<_>>();
            assert!(
                matches!(validate_arguments(&arguments), Err(FrontendError::Arguments(_))),
                "{arguments:?}"
            );
        }
    }

    /// Checks automatically loaded driver configuration is rejected.
    #[test]
    fn automatically_loaded_driver_configuration_is_rejected() {
        assert!(matches!(
            validate_driver_configuration(
                "clang version 21.0.0\nConfiguration file: /tmp/clang.cfg\n"
            ),
            Err(FrontendError::Environment(_))
        ));
        validate_driver_configuration("clang version 21.0.0\nTarget: x86_64-linux-gnu\n").unwrap();
    }
}
