//LICENSE Portions Copyright 2026 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.

//! Verify that PostgreSQL server headers were configured for the inspected C target.
//!
//! `pg_config.h` records configure's or meson's results for one machine, and other headers
//! choose declaration shapes from them: x86_64 headers declare `pg_popcount32` as a function
//! pointer, while aarch64 servers define a function. Headers configured for another machine
//! therefore bind declarations the deployed server does not have. Two checks establish that
//! the headers describe the target. The machine in `PG_VERSION_STR` must match Clang's
//! triple, which is the only evidence for ISA-specific choices. The layout facts in
//! `pg_config.h` must equal the target compiler's, which distinguishes ABIs that share a
//! machine name, such as x32 or the two byte orders of meson's `ppc64`.

use crate::model::{MachineFamily, machine_family};
use crate::{ByteOrder, Error, FrontendOutput, MacroScanner, Token, TokenKind};
use clang::{EntityKind, EntityVisitResult};
use std::collections::BTreeMap;
use std::fmt::{self, Write as _};

/// Prefix of the probe's enum constants: `h_` constants hold `pg_config.h` values and `t_`
/// constants hold the target compiler's.
const PROBE_PREFIX: &str = "__pgrx_pg_config_";

/// Facts every supported `pg_config.h` defines (PG15-19, autoconf, meson and legacy MSVC).
const REQUIRED_FACTS: &[&str] = &[
    "SIZEOF_VOID_P",
    "SIZEOF_SIZE_T",
    "SIZEOF_LONG",
    "ALIGNOF_INT",
    "ALIGNOF_DOUBLE",
    "MAXIMUM_ALIGNOF",
    "PG_MAJORVERSION_NUM",
];

/// Size facts and the builtin-type expression the target compiler evaluates for each.
const SIZE_FACTS: &[(&str, &str)] = &[
    ("SIZEOF_VOID_P", "sizeof(void *)"),
    ("SIZEOF_SIZE_T", "sizeof(__SIZE_TYPE__)"),
    ("SIZEOF_LONG", "sizeof(long)"),
    ("SIZEOF_LONG_LONG", "sizeof(long long)"),
    ("SIZEOF_BOOL", "sizeof(_Bool)"),
];

/// Alignment facts that bound `MAXIMUM_ALIGNOF`, with the builtin type each measures.
///
/// configure's `AC_CHECK_ALIGNOF` and meson's `cc.alignment` both measure a member's offset
/// after a `char`, which differs from `_Alignof` on i686 Linux (4 rather than 8 for `double`).
const ALIGNMENT_FACTS: &[(&str, &str)] = &[
    ("ALIGNOF_SHORT", "short"),
    ("ALIGNOF_INT", "int"),
    ("ALIGNOF_LONG", "long"),
    ("ALIGNOF_LONG_LONG_INT", "long long"),
    ("ALIGNOF_INT64_T", "__INT64_TYPE__"),
    ("ALIGNOF_DOUBLE", "double"),
];

/// Headers configured for a machine other than the inspected C target.
#[derive(Debug, thiserror::Error)]
pub enum ConfigurationError {
    /// `PG_VERSION_STR` names a machine whose architecture, OS, C library or float ABI differs
    /// from Clang's target.
    #[error(
        "pg_config.h was configured for {recorded} (PG_VERSION_STR), but Clang targets {target}"
    )]
    Machine {
        /// The machine named in `PG_VERSION_STR`.
        recorded: String,
        /// Clang's effective target triple.
        target: String,
    },
    /// The headers or the target name no recognized machine, and the target is not the
    /// generator's host, so nothing shows that the headers were configured for the target.
    #[error(
        "cannot establish that pg_config.h ({}) was configured for Clang target {target}: \
         the machine is not recognized, and the target is not this generator's host",
        recorded.as_deref().unwrap_or("without PG_VERSION_STR")
    )]
    UnknownMachine {
        /// The machine named in `PG_VERSION_STR`, or its whole text when it names none.
        recorded: Option<String>,
        /// Clang's effective target triple.
        target: String,
    },
    /// The headers belong to another PostgreSQL major version.
    #[error("pg_config.h is PostgreSQL {headers}, but PostgreSQL {expected} was selected")]
    Major {
        /// `PG_MAJORVERSION_NUM` in the headers.
        headers: i64,
        /// The selected installation's major version.
        expected: u16,
    },
    /// `pg_config.h` lacks a fact every supported version defines.
    #[error("pg_config.h does not define {0}")]
    MissingFact(
        /// The missing macro.
        &'static str,
    ),
    /// `pg_config.h` layout facts differ from the target compiler's.
    #[error(
        "pg_config.h facts disagree with Clang target {target}: {}",
        Disagreements(disagreements)
    )]
    Layout {
        /// Clang's effective target triple.
        target: String,
        /// Every disagreement, in fact-name order.
        disagreements: Vec<LayoutDisagreement>,
    },
    /// Clang could not evaluate the layout probe.
    #[error("could not evaluate pg_config.h facts for the target: {0}")]
    Probe(
        /// The libclang failure, which names the diagnostics.
        Error,
    ),
}

/// One `pg_config.h` fact whose recorded value differs from the target compiler's value.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LayoutDisagreement {
    /// The `pg_config.h` macro, or `sizeof(TYPE)` for a type macro.
    pub fact: String,
    /// The value in `pg_config.h`; a defined flag is 1 and an undefined one 0.
    pub headers: i64,
    /// The value the target compiler requires.
    pub target: i64,
}

/// List disagreements as `FACT headers H, target T`.
struct Disagreements<'a>(&'a [LayoutDisagreement]);

impl fmt::Display for Disagreements<'_> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (index, LayoutDisagreement { fact, headers, target }) in self.0.iter().enumerate() {
            if index != 0 {
                formatter.write_str("; ")?;
            }
            write!(formatter, "{fact} headers {headers}, target {target}")?;
        }
        Ok(())
    }
}

/// Require PostgreSQL headers to have been configured for the inspected C target.
///
/// An input without `PG_VERSION_NUM` is not a PostgreSQL configuration, such as a synthetic
/// header, and is accepted unchecked. Otherwise the machine in `PG_VERSION_STR` must match the
/// target, and one libclang parse with the inspected arguments must find every layout fact in
/// `pg_config.h` equal to the target's. A header without a recognized machine is accepted only
/// when the target is the generator's own host; in a build script that is Cargo's host.
pub fn verify_postgres_configuration(
    scanner: &MacroScanner,
    frontend: &FrontendOutput,
    expected_major: u16,
) -> Result<(), ConfigurationError> {
    let active = &frontend.environment().active;
    if !active.contains_key("PG_VERSION_NUM") {
        return Ok(());
    }
    let target = &frontend.profile().target;
    let version =
        active.get("PG_VERSION_STR").map(|active| version_text(&active.definition.tokens));
    check_machine(
        version.as_deref(),
        &target.triple,
        (std::env::consts::ARCH, std::env::consts::OS),
    )?;
    let values = probe_layout(scanner, frontend)?;
    check_layout(&values, &target.triple, target.byte_order == ByteOrder::Big, expected_major)
}

/// Concatenate the string literals in a `PG_VERSION_STR` replacement.
///
/// Legacy MSVC builds splice in `CppAsString2(_MSC_VER)`; that call contributes no text.
fn version_text(tokens: &[Token]) -> String {
    tokens
        .iter()
        .skip(1)
        .filter(|token| token.kind == TokenKind::Literal)
        .filter_map(|token| token.spelling.strip_prefix('"')?.strip_suffix('"'))
        .collect()
}

/// The machine `PG_VERSION_STR` names and its family.
///
/// The text before ", compiled by " ends with " on <machine>": a config.guess triple such as
/// `x86_64-pc-linux-gnu`, or meson's `<cpu family>-<system>` such as `aarch64-darwin`. Legacy
/// MSVC builds name no machine, but only build for Windows.
fn configured_machine(version: &str) -> (&str, MachineFamily) {
    let configured = version.split(", compiled by ").next().unwrap_or_default();
    let Some((_, machine)) = configured.rsplit_once(" on ") else {
        let os = version.contains(", compiled by Visual C++").then_some("windows");
        return (version, MachineFamily { arch: None, os, libc: None, hard_float: None });
    };
    let machine = machine.trim();
    let mut family = machine_family(machine);
    // config.guess named Apple silicon arm-apple-darwin before it learned aarch64. No
    // 32-bit ARM macOS server exists.
    if machine.starts_with("arm-apple-darwin") {
        family.arch = Some("aarch64");
    }
    (machine, family)
}

/// Compare the recorded machine with the target, given the generator's `(arch, os)` host.
///
/// Every property known on both sides must be equal. Without a recognized architecture and OS
/// on both sides, only the host's own target is accepted: native builds keep working with
/// vendor-patched strings, while cross builds must supply evidence.
fn check_machine(
    version: Option<&str>,
    target: &str,
    host: (&str, &str),
) -> Result<(), ConfigurationError> {
    let target_family = machine_family(target);
    let recorded = version.map(configured_machine);
    if let Some((machine, family)) = recorded {
        if known_and_different(family.arch, target_family.arch)
            || known_and_different(family.os.map(os_family), target_family.os.map(os_family))
            || known_and_different(family.libc, target_family.libc)
            || known_and_different(family.hard_float, target_family.hard_float)
        {
            return Err(ConfigurationError::Machine {
                recorded: machine.into(),
                target: target.into(),
            });
        }
        if family.arch.is_some()
            && family.os.is_some()
            && target_family.arch.is_some()
            && target_family.os.is_some()
        {
            return Ok(());
        }
    }
    let native = target_family.arch == Some(host.0)
        && target_family.os.map(os_family) == Some(os_family(host.1));
    if native {
        return Ok(());
    }
    Err(ConfigurationError::UnknownMachine {
        recorded: recorded.map(|(machine, _)| machine.into()),
        target: target.into(),
    })
}

/// Whether both sides name a property and the names differ.
fn known_and_different<T: PartialEq>(recorded: Option<T>, target: Option<T>) -> bool {
    recorded.zip(target).is_some_and(|(recorded, target)| recorded != target)
}

/// config.guess reports illumos as Solaris, while Clang and Rust distinguish them.
fn os_family(os: &str) -> &str {
    if os == "illumos" { "solaris" } else { os }
}

/// `pg_config.h` values and the target compiler's values, keyed by `pg_config.h` macro.
///
/// A defined flag is 1; its target value is 1 when configure would define it for the target.
/// The type macros `PG_INT64_TYPE` and `PG_INT128_TYPE` appear only as target values, holding
/// their `sizeof`.
#[derive(Debug, Default)]
struct LayoutValues {
    /// Values from `pg_config.h`.
    headers: BTreeMap<String, i64>,
    /// Values the target compiler computes from builtin types.
    target: BTreeMap<String, i64>,
}

/// Build the probe source: the inspected header, then one guarded enum per fact.
///
/// Including the inspected header resolves `pg_config.h` exactly as inspection did. Each fact
/// is guarded by `#ifdef`, since versions define different facts. Target values use builtin
/// types only, so a header cannot change what they measure.
fn probe_source(frontend: &FrontendOutput) -> Result<String, crate::FrontendError> {
    let header = crate::c_header_path(&frontend.profile().header)?;
    let mut source = format!(
        "#include \"{header}\"\n\
         #if defined(sizeof) || defined(__builtin_offsetof)\n\
         #error \"a macro replaces an operator the pg_config.h probe needs\"\n\
         #endif\n"
    );
    for (name, expression) in SIZE_FACTS {
        probe_fact(&mut source, name, &format!("({name})"), Some(expression));
    }
    for (name, ty) in ALIGNMENT_FACTS {
        probe_alignment(&mut source, name, ty);
    }
    source.push_str("#ifdef PG_INT128_TYPE\n");
    probe_alignment(&mut source, "ALIGNOF_PG_INT128_TYPE", "PG_INT128_TYPE");
    source.push_str("#endif\n");
    for name in ["MAXIMUM_ALIGNOF", "PG_MAJORVERSION_NUM"] {
        probe_fact(&mut source, name, &format!("({name})"), None);
    }
    // A defined flag has the value 1, whatever its replacement. configure defines these
    // integer flags exactly when the type is 64 bits wide.
    probe_fact(&mut source, "WORDS_BIGENDIAN", "1", None);
    for (name, ty) in [("HAVE_LONG_INT_64", "long"), ("HAVE_LONG_LONG_INT_64", "long long")] {
        probe_fact(&mut source, name, "1", Some(&format!("sizeof({ty}) == 8")));
    }
    for name in ["PG_INT64_TYPE", "PG_INT128_TYPE"] {
        writeln!(
            source,
            "#ifdef {name}\nenum {{ {PROBE_PREFIX}t_{name} = sizeof({name}) }};\n#endif"
        )
        .expect("String output");
    }
    Ok(source)
}

/// Append a fact's header and target values, guarded by the fact's definition.
fn probe_fact(source: &mut String, name: &str, header: &str, target: Option<&str>) {
    writeln!(source, "#ifdef {name}\nenum {{ {PROBE_PREFIX}h_{name} = {header} }};")
        .expect("String output");
    if let Some(target) = target {
        writeln!(source, "enum {{ {PROBE_PREFIX}t_{name} = {target} }};").expect("String output");
    }
    source.push_str("#endif\n");
}

/// Append an alignment fact whose target value is the offset of a `ty` member after a `char`,
/// which configure's `AC_CHECK_ALIGNOF` and meson's `cc.alignment` measure.
fn probe_alignment(source: &mut String, name: &str, ty: &str) {
    let record = format!("{PROBE_PREFIX}a_{name}");
    writeln!(source, "#ifdef {name}\nstruct {record} {{ char __pgrx_c; {ty} __pgrx_v; }};\n#endif")
        .expect("String output");
    let offset = format!("__builtin_offsetof(struct {record}, __pgrx_v)");
    probe_fact(source, name, &format!("({name})"), Some(&offset));
}

/// Evaluate the probe with one libclang parse under the inspected arguments.
fn probe_layout(
    scanner: &MacroScanner,
    frontend: &FrontendOutput,
) -> Result<LayoutValues, ConfigurationError> {
    let source = probe_source(frontend)
        .map_err(|error| ConfigurationError::Probe(Error::InvalidInput(error.to_string())))?;
    let header = std::env::temp_dir().join("pgrx-c-macros-configuration.h");
    scanner
        .with_declarations(&header, &frontend.profile().arguments, Some(&source), |unit| {
            let mut values = LayoutValues::default();
            unit.get_entity().visit_children(|entity, _| match entity.get_kind() {
                EntityKind::EnumDecl if entity.is_in_main_file() => EntityVisitResult::Recurse,
                EntityKind::EnumConstantDecl => {
                    if let Some(name) = entity.get_name()
                        && let Some(fact) = name.strip_prefix(PROBE_PREFIX)
                        && let Some((value, _)) = entity.get_enum_constant_value()
                    {
                        if let Some(fact) = fact.strip_prefix("h_") {
                            values.headers.insert(fact.into(), value);
                        } else if let Some(fact) = fact.strip_prefix("t_") {
                            values.target.insert(fact.into(), value);
                        }
                    }
                    EntityVisitResult::Continue
                }
                _ => EntityVisitResult::Continue,
            });
            Ok(values)
        })
        .map_err(ConfigurationError::Probe)
}

/// Compare `pg_config.h` with the target compiler and report every disagreement together.
fn check_layout(
    values: &LayoutValues,
    target: &str,
    big_endian: bool,
    expected_major: u16,
) -> Result<(), ConfigurationError> {
    if let Some(missing) = REQUIRED_FACTS.iter().find(|fact| !values.headers.contains_key(**fact)) {
        return Err(ConfigurationError::MissingFact(missing));
    }
    let major = values.headers["PG_MAJORVERSION_NUM"];
    if major != i64::from(expected_major) {
        return Err(ConfigurationError::Major { headers: major, expected: expected_major });
    }
    let mut disagreements = Vec::new();
    let mut disagree = |fact: &str, headers: i64, target: i64| {
        if headers != target {
            disagreements.push(LayoutDisagreement { fact: fact.into(), headers, target });
        }
    };
    // This also checks HAVE_LONG_INT_64 and HAVE_LONG_LONG_INT_64 when they are defined. PG18
    // and later no longer define them, so an absent flag is not compared.
    for (fact, &headers) in &values.headers {
        if let Some(&target) = values.target.get(fact) {
            disagree(fact, headers, target);
        }
    }
    // configure and meson set MAXIMUM_ALIGNOF to the largest alignment they record. Taking the
    // target's values reports the alignment the target needs even when one also disagrees.
    let maximum = ALIGNMENT_FACTS
        .iter()
        .filter(|(fact, _)| values.headers.contains_key(*fact))
        .filter_map(|(fact, _)| values.target.get(*fact))
        .max();
    if let Some(&maximum) = maximum {
        disagree("MAXIMUM_ALIGNOF", values.headers["MAXIMUM_ALIGNOF"], maximum);
    }
    disagree(
        "WORDS_BIGENDIAN",
        i64::from(values.headers.contains_key("WORDS_BIGENDIAN")),
        i64::from(big_endian),
    );
    for (name, size) in [("PG_INT64_TYPE", 8), ("PG_INT128_TYPE", 16)] {
        if let Some(&actual) = values.target.get(name) {
            disagree(&format!("sizeof({name})"), size, actual);
        }
    }
    if disagreements.is_empty() {
        Ok(())
    } else {
        Err(ConfigurationError::Layout { target: target.into(), disagreements })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::Libc;

    /// The machine comes from the text after the last " on " before ", compiled by ", in the
    /// spellings autoconf, meson, vendors and legacy MSVC produce.
    #[test]
    fn configured_machine_parses_autoconf_meson_vendor_and_legacy_strings() {
        let family = |arch, os, libc, hard_float| MachineFamily { arch, os, libc, hard_float };
        let linux = |arch| family(Some(arch), Some("linux"), None, None);
        for (version, machine, expected) in [
            (
                "PostgreSQL 18.4 (Debian 18.4-1.pgdg13+1) on x86_64-pc-linux-gnu, compiled by gcc (Debian 14.2.0-19) 14.2.0, 64-bit",
                "x86_64-pc-linux-gnu",
                family(Some("x86_64"), Some("linux"), Some(Libc::Gnu), None),
            ),
            (
                "PostgreSQL 18.4 on aarch64-apple-darwin25.6.0, compiled by Apple clang version 21.0.0, 64-bit",
                "aarch64-apple-darwin25.6.0",
                family(Some("aarch64"), Some("macos"), None, None),
            ),
            (
                "PostgreSQL 17.2 on amd64-portbld-freebsd14.1, compiled by FreeBSD clang version 18.1.6, 64-bit",
                "amd64-portbld-freebsd14.1",
                family(Some("x86_64"), Some("freebsd"), None, None),
            ),
            (
                "PostgreSQL 16.1 on x86_64-w64-mingw32, compiled by gcc.exe (Rev3, Built by MSYS2 project) 13.2.0, 64-bit",
                "x86_64-w64-mingw32",
                family(Some("x86_64"), Some("windows"), None, None),
            ),
            (
                "PostgreSQL 17.0 on x86_64-linux, compiled by gcc-14.2.0, 64-bit",
                "x86_64-linux",
                linux("x86_64"),
            ),
            (
                "PostgreSQL 17.0 on ppc64-linux, compiled by gcc-14.2.0, 64-bit",
                "ppc64-linux",
                linux("powerpc64"),
            ),
            (
                "PostgreSQL 17.0 on aarch64-darwin, compiled by clang-17.0.0, 64-bit",
                "aarch64-darwin",
                family(Some("aarch64"), Some("macos"), None, None),
            ),
            (
                "PostgreSQL 17.0 on x86_64-windows, compiled by msvc-19.40.33811, 64-bit",
                "x86_64-windows",
                family(Some("x86_64"), Some("windows"), None, None),
            ),
            (
                "PostgreSQL 15.4 on arm-unknown-linux-gnueabihf, compiled by gcc (Raspbian 12.2.0-14) 12.2.0, 32-bit",
                "arm-unknown-linux-gnueabihf",
                family(Some("arm"), Some("linux"), Some(Libc::Gnu), Some(true)),
            ),
            (
                "PostgreSQL 13.1 on arm-apple-darwin20.1.0, compiled by Apple clang version 12.0.0, 64-bit",
                "arm-apple-darwin20.1.0",
                family(Some("aarch64"), Some("macos"), None, None),
            ),
        ] {
            assert_eq!(configured_machine(version), (machine, expected), "{version}");
        }
        // Concatenated tokens of "…, compiled by Visual C++ build " CppAsString2(_MSC_VER) ", 64-bit".
        let legacy = "PostgreSQL 15.4, compiled by Visual C++ build , 64-bit";
        assert_eq!(configured_machine(legacy), (legacy, family(None, Some("windows"), None, None)));
        assert_eq!(configured_machine("garbage"), ("garbage", family(None, None, None, None)));
        assert_eq!(
            configured_machine("PostgreSQL 18.4 on mystery-box, compiled by cc").1,
            family(None, None, None, None)
        );
    }

    /// Only the literal tokens of PG_VERSION_STR form its text.
    #[test]
    fn version_text_concatenates_string_literals() {
        let token = |kind, spelling: &str| Token { kind, spelling: spelling.into() };
        let tokens = [
            token(TokenKind::Identifier, "PG_VERSION_STR"),
            token(TokenKind::Literal, "\"PostgreSQL 15.4, compiled by Visual C++ build \""),
            token(TokenKind::Identifier, "CppAsString2"),
            token(TokenKind::Punctuation, "("),
            token(TokenKind::Identifier, "_MSC_VER"),
            token(TokenKind::Punctuation, ")"),
            token(TokenKind::Literal, "\", 64-bit\""),
        ];
        assert_eq!(version_text(&tokens), "PostgreSQL 15.4, compiled by Visual C++ build , 64-bit");
    }

    /// Known machines must match in every known property. An unrecognized or absent machine is
    /// accepted only for the generator's own host.
    #[test]
    fn check_machine_policy() {
        let host = ("aarch64", "macos");
        let debian = "PostgreSQL 18.4 on x86_64-pc-linux-gnu, compiled by gcc, 64-bit";
        let machine = |version, target| check_machine(Some(version), target, host);
        assert!(machine(debian, "x86_64-unknown-linux-gnu").is_ok());
        let error = machine(debian, "aarch64-unknown-linux-gnu").unwrap_err();
        assert!(
            matches!(&error, ConfigurationError::Machine { recorded, target }
                if recorded == "x86_64-pc-linux-gnu" && target == "aarch64-unknown-linux-gnu"),
            "{error:?}"
        );
        let message = error.to_string();
        assert!(
            message.contains("x86_64-pc-linux-gnu")
                && message.contains("aarch64-unknown-linux-gnu")
        );
        assert!(matches!(
            machine(debian, "x86_64-unknown-linux-musl"),
            Err(ConfigurationError::Machine { .. })
        ));
        let armhf = "PostgreSQL 15.4 on arm-unknown-linux-gnueabihf, compiled by gcc, 32-bit";
        assert!(machine(armhf, "armv7-unknown-linux-gnueabihf").is_ok());
        assert!(matches!(
            machine(armhf, "armv7-unknown-linux-gnueabi"),
            Err(ConfigurationError::Machine { .. })
        ));
        let illumos = "PostgreSQL 16.0 on x86_64-pc-solaris2.11, compiled by gcc, 64-bit";
        assert!(machine(illumos, "x86_64-unknown-illumos").is_ok());
        let macos =
            "PostgreSQL 18.4 on aarch64-apple-darwin25.6.0, compiled by Apple clang, 64-bit";
        assert!(machine(macos, "arm64-apple-macosx26.0.0").is_ok());

        let vendor = "PostgreSQL 18.4 on custom-build, compiled by cc, 64-bit";
        assert!(machine(vendor, "arm64-apple-macosx26.0.0").is_ok());
        assert!(matches!(
            machine(vendor, "aarch64-unknown-linux-gnu"),
            Err(ConfigurationError::UnknownMachine { recorded: Some(recorded), .. })
                if recorded == "custom-build"
        ));
        assert!(check_machine(None, "arm64-apple-macosx26.0.0", host).is_ok());
        assert!(matches!(
            check_machine(None, "x86_64-apple-macosx26.0.0", host),
            Err(ConfigurationError::UnknownMachine { recorded: None, .. })
        ));
        let legacy = "PostgreSQL 15.4, compiled by Visual C++ build , 64-bit";
        assert!(
            check_machine(Some(legacy), "x86_64-pc-windows-msvc19.40.0", ("x86_64", "windows"))
                .is_ok()
        );
        assert!(matches!(
            check_machine(Some(legacy), "x86_64-pc-windows-msvc19.40.0", host),
            Err(ConfigurationError::UnknownMachine { .. })
        ));
        assert!(matches!(
            check_machine(Some(legacy), "x86_64-unknown-linux-gnu", ("x86_64", "linux")),
            Err(ConfigurationError::Machine { .. })
        ));
    }

    /// An LP64 little-endian PG18 configuration as the target compiler sees it.
    fn lp64() -> LayoutValues {
        let facts = [
            ("SIZEOF_VOID_P", 8),
            ("SIZEOF_SIZE_T", 8),
            ("SIZEOF_LONG", 8),
            ("SIZEOF_LONG_LONG", 8),
            ("ALIGNOF_SHORT", 2),
            ("ALIGNOF_INT", 4),
            ("ALIGNOF_LONG", 8),
            ("ALIGNOF_INT64_T", 8),
            ("ALIGNOF_DOUBLE", 8),
            ("ALIGNOF_PG_INT128_TYPE", 16),
        ];
        let mut values = LayoutValues::default();
        for (fact, value) in facts {
            values.headers.insert(fact.into(), value);
            values.target.insert(fact.into(), value);
        }
        values.headers.insert("MAXIMUM_ALIGNOF".into(), 8);
        values.headers.insert("PG_MAJORVERSION_NUM".into(), 18);
        values.target.insert("PG_INT128_TYPE".into(), 16);
        values
    }

    /// Every disagreement is reported with both values, rather than only the first.
    #[test]
    fn check_layout_reports_every_disagreement() {
        let target = "x86_64-unknown-linux-gnu";
        assert!(check_layout(&lp64(), target, false, 18).is_ok());

        let mut x32 = lp64();
        for fact in ["SIZEOF_VOID_P", "SIZEOF_SIZE_T", "SIZEOF_LONG", "ALIGNOF_LONG"] {
            x32.target.insert(fact.into(), 4);
        }
        let error = check_layout(&x32, "x86_64-unknown-linux-gnux32", false, 18).unwrap_err();
        let ConfigurationError::Layout { disagreements, .. } = &error else { panic!("{error:?}") };
        let disagreement =
            |fact: &str, headers, target| LayoutDisagreement { fact: fact.into(), headers, target };
        assert_eq!(
            disagreements,
            &[
                disagreement("ALIGNOF_LONG", 8, 4),
                disagreement("SIZEOF_LONG", 8, 4),
                disagreement("SIZEOF_SIZE_T", 8, 4),
                disagreement("SIZEOF_VOID_P", 8, 4),
            ]
        );
        assert_eq!(
            error.to_string(),
            "pg_config.h facts disagree with Clang target x86_64-unknown-linux-gnux32: \
             ALIGNOF_LONG headers 8, target 4; SIZEOF_LONG headers 8, target 4; \
             SIZEOF_SIZE_T headers 8, target 4; SIZEOF_VOID_P headers 8, target 4"
        );

        let mut missing = lp64();
        missing.headers.remove("ALIGNOF_DOUBLE");
        assert!(matches!(
            check_layout(&missing, target, false, 18),
            Err(ConfigurationError::MissingFact("ALIGNOF_DOUBLE"))
        ));

        let mut maximum = lp64();
        maximum.headers.insert("MAXIMUM_ALIGNOF".into(), 4);
        assert!(matches!(
            check_layout(&maximum, target, false, 18),
            Err(ConfigurationError::Layout { disagreements, .. })
                if disagreements == [disagreement("MAXIMUM_ALIGNOF", 4, 8)]
        ));

        let big = disagreement("WORDS_BIGENDIAN", 0, 1);
        assert!(matches!(
            check_layout(&lp64(), "powerpc64-unknown-linux-gnu", true, 18),
            Err(ConfigurationError::Layout { disagreements, .. }) if disagreements == [big]
        ));
        let mut big_endian = lp64();
        big_endian.headers.insert("WORDS_BIGENDIAN".into(), 1);
        assert!(check_layout(&big_endian, "powerpc64-unknown-linux-gnu", true, 18).is_ok());
        assert!(matches!(
            check_layout(&big_endian, "powerpc64le-unknown-linux-gnu", false, 18),
            Err(ConfigurationError::Layout { disagreements, .. })
                if disagreements == [disagreement("WORDS_BIGENDIAN", 1, 0)]
        ));

        // PG18 no longer defines HAVE_LONG_INT_64; PG15-17 must agree when they do.
        let mut pg17 = lp64();
        pg17.headers.insert("HAVE_LONG_INT_64".into(), 1);
        pg17.target.insert("HAVE_LONG_INT_64".into(), 1);
        pg17.headers.insert("PG_MAJORVERSION_NUM".into(), 17);
        assert!(check_layout(&pg17, target, false, 17).is_ok());
        pg17.target.insert("HAVE_LONG_INT_64".into(), 0);
        assert!(matches!(
            check_layout(&pg17, target, false, 17),
            Err(ConfigurationError::Layout { disagreements, .. })
                if disagreements == [disagreement("HAVE_LONG_INT_64", 1, 0)]
        ));
        let mut int128 = lp64();
        int128.target.insert("PG_INT128_TYPE".into(), 8);
        assert!(matches!(
            check_layout(&int128, target, false, 18),
            Err(ConfigurationError::Layout { disagreements, .. })
                if disagreements == [disagreement("sizeof(PG_INT128_TYPE)", 16, 8)]
        ));

        assert!(matches!(
            check_layout(&lp64(), target, false, 17),
            Err(ConfigurationError::Major { headers: 18, expected: 17 })
        ));
    }
}
