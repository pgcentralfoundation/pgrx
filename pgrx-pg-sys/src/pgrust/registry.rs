//! What the macros register at link time and how it reaches pgrust.
//!
//! Each `#[pg_extern]` adds a [`FnEntry`] to [`PGRX_FUNCTIONS`]; each schema
//! entity adds its serialized payload to [`PGRX_ENTITIES`] (the same bytes
//! that go into the `.pgrxsc` section on Postgres); `pg_module_magic!` emits
//! an [`ExtensionDesc`] and an `init_seams()` that calls
//! [`register_extension`], which registers the crate as a builtin library
//! with pgrust's `dfmgr` (so `CREATE FUNCTION ... AS 'MODULE_PATHNAME',
//! 'foo_wrapper'` resolves) and as an embedded extension (control file plus
//! the SQL script generated from the entity graph).

use ::pgr_fmgr::PGFunction as NativePGFunction;
use ::pgr_types_error::PgResult;
use linkme::distributed_slice;

/// One `#[pg_extern]` wrapper.
pub struct FnEntry {
    pub krate: &'static str,
    pub symbol: &'static str,
    pub func: NativePGFunction,
}

/// One serialized SQL entity (a `.pgrxsc` section entry).
pub struct EntityEntry {
    pub krate: &'static str,
    pub bytes: &'static [u8],
}

/// The extension an extension crate declares with `pg_module_magic!`.
pub struct ExtensionDesc {
    pub krate: &'static str,
    pub name: &'static str,
    pub version: &'static str,
    pub control: &'static str,
    pub pg_init: Option<fn() -> PgResult<()>>,
}

/// A `#[pg_guard] fn _PG_init()`.
pub struct PgInitEntry {
    pub krate: &'static str,
    pub func: fn(),
}

#[distributed_slice]
pub static PGRX_FUNCTIONS: [FnEntry];

#[distributed_slice]
pub static PGRX_PG_INIT: [PgInitEntry];

/// Run crate `krate`'s `_PG_init` functions (C: dfmgr's first-load hook). A
/// panic inside becomes the load's error.
pub fn run_pg_init(krate: &str) -> PgResult<()> {
    for e in PGRX_PG_INIT.iter().filter(|e| e.krate == krate) {
        let r = std::panic::catch_unwind(e.func);
        if let Err(payload) = r {
            return Err(super::error::caught_to_pg_error(
                crate::panic::downcast_panic_payload(payload),
            ));
        }
    }
    Ok(())
}

#[distributed_slice]
pub static PGRX_ENTITIES: [EntityEntry];

/// The wrapper `symbol` of crate `krate`, as pgrust's dfmgr lookup wants it.
pub fn lookup_in(krate: &str, symbol: &str) -> Option<NativePGFunction> {
    PGRX_FUNCTIONS
        .iter()
        .find(|e| e.krate == krate && e.symbol == symbol)
        .map(|e| e.func)
}

/// The extension's SQL script, generated from its entity graph exactly as
/// `cargo pgrx schema` would.
pub fn generate_sql(desc: &ExtensionDesc) -> Result<String, String> {
    use pgrx_sql_entity_graph::{ControlFile, PgrxSql, SqlGraphEntity};

    let mut section: Vec<u8> = Vec::new();
    for e in PGRX_ENTITIES.iter().filter(|e| e.krate == desc.krate) {
        section.extend_from_slice(e.bytes);
    }
    let entities = pgrx_sql_entity_graph::section::decode_entities(&section)
        .map_err(|e| format!("decoding schema entities: {e}"))?;
    let control = ControlFile::from_str_with_cargo_version(desc.control, desc.version)
        .map_err(|e| format!("parsing control file: {e}"))?;
    let all = entities
        .into_iter()
        .chain(core::iter::once(SqlGraphEntity::ExtensionRoot(control)));
    let sql = PgrxSql::build(all, desc.name.to_owned(), false)
        .map_err(|e| format!("building schema graph: {e}"))?;
    sql.to_sql()
        .map_err(|e| format!("rendering schema SQL: {e}"))
}

/// Register an extension crate with pgrust. Called from the crate's
/// generated `init_seams()`, once per process, before any session runs.
pub fn register_extension(
    desc: &'static ExtensionDesc,
    lookup: fn(&str) -> Option<NativePGFunction>,
) {
    // The library name is what the control file's module_pathname resolves
    // to (its basename); pgrx defaults it to the crate name.
    let libname: &'static str = match control_module_pathname(desc.control) {
        Some(p) => Box::leak(
            p.rsplit('/')
                .next()
                .unwrap_or(&p)
                .to_owned()
                .into_boxed_str(),
        ),
        None => desc.name,
    };
    ::pgr_dfmgr::register_builtin_library(::pgr_dfmgr::BuiltinLibraryEntry {
        name: libname,
        lookup,
        pg_init: desc.pg_init,
    });
    let sql = match generate_sql(desc) {
        Ok(s) => s,
        Err(e) => panic!("pgrx extension \"{}\": {e}", desc.name),
    };
    let control = desc.control.replace("@CARGO_VERSION@", desc.version);
    // The install script is registered under the crate version and, when the
    // control file pins a different default_version, under that too (cargo
    // pgrx names the script after default_version).
    let mut scripts = vec![(desc.version.to_owned(), sql.clone())];
    if let Some(v) = control_default_version(&control) {
        if v != desc.version {
            scripts.push((v, sql));
        }
    }
    ::pgr_dfmgr::register_embedded_extension(::pgr_dfmgr::EmbeddedExtension {
        name: desc.name.to_owned(),
        control,
        scripts,
    });
}

fn control_value(control: &str, key: &str) -> Option<String> {
    for line in control.lines() {
        let line = line.trim();
        let Some(rest) = line.strip_prefix(key) else {
            continue;
        };
        let rest = rest.trim_start();
        let Some(rest) = rest.strip_prefix('=') else {
            continue;
        };
        let v = rest.trim().trim_matches('\'').trim_matches('"');
        if !v.is_empty() {
            return Some(v.to_owned());
        }
    }
    None
}

/// `module_pathname = '...'` from a control file's text, if present.
fn control_module_pathname(control: &str) -> Option<String> {
    control_value(control, "module_pathname")
}

/// `default_version = '...'` from a control file's text, if present.
fn control_default_version(control: &str) -> Option<String> {
    for line in control.lines() {
        let line = line.trim();
        let Some(rest) = line.strip_prefix("default_version") else {
            continue;
        };
        let rest = rest.trim_start();
        let Some(rest) = rest.strip_prefix('=') else {
            continue;
        };
        let v = rest.trim().trim_matches('\'').trim_matches('"');
        if !v.is_empty() {
            return Some(v.to_owned());
        }
    }
    None
}
