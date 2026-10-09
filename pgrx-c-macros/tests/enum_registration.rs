//LICENSE Portions Copyright 2026 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.

//! Check the registration boundary between enum identity and numeric storage.
//!
//! Standalone consumer compilation exercises coexistence with legacy wrappers,
//! exact pointer compatibility, and rejection of wrong ranks or duplicate impls.
//! Identical integer representations must not merge distinct C identities.

/// Compile generated consumers and paired negative cases through the bounded Rust oracle
/// harness.
#[path = "support/rust_oracle.rs"]
#[allow(dead_code)]
mod rust_oracle;

use std::path::PathBuf;

/// Assemble emitted macro definitions for the consumer, failing the test if an expected
/// candidate is skipped.
fn source(body: &str) -> String {
    let support = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../pgrx-pg-sys/src/c_macros/support.rs")
        .canonicalize()
        .unwrap();
    format!(
        "#![allow(dead_code)]\n#[path={support:?}] pub mod __pgrx_c_macros;\n{IDENTITIES}\nfn main() {{ {body} }}"
    )
}

/// Fixture enum identities and storage declarations used to check nominal registration
/// boundaries.
const IDENTITIES: &str = r#"
use __pgrx_c_macros as c;
use __pgrx_c_macros::{CUnsignedInt, CUnsignedLong, CUnsignedLongLong, CLong};
use __pgrx_c_macros::expression::{CEnum, CompatibleIdentity, EnumIdentity, EnumStorage};
use __pgrx_c_macros::expression::enumeration::NumericEnum;
macro_rules! identity {
    ($name:ident $(, $kind:ty)?) => {
        #[derive(Clone, Copy, Debug, PartialEq, Eq)] struct $name;
        impl __pgrx_c_macros::sealed::Sealed for $name {}
        impl EnumIdentity for $name {}
        $(impl NumericEnum for $name { type Compatible = $kind; })?
    };
}
identity!(Unsigned, CUnsignedInt);
identity!(OtherUnsigned, CUnsignedInt);
identity!(PlainChar, c::CChar);
identity!(Wide, CUnsignedLong);
identity!(WideLongLong, CUnsignedLongLong);
identity!(Legacy);
identity!(Unregistered);
impl CompatibleIdentity<CUnsignedInt> for CEnum<Legacy, CUnsignedInt> {}
impl CompatibleIdentity<CEnum<Legacy, CUnsignedInt>> for CUnsignedInt {}
fn compatible<Left: CompatibleIdentity<Right>, Right>() {}
#[repr(u32)] #[derive(Clone, Copy)] enum Checked { One = 1 }
// SAFETY: repr(u32) has the compatible integer layout; encoding checks validity.
unsafe impl EnumStorage<Unsigned, CUnsignedInt> for Checked {
    fn decode(value: Self) -> u32 { value as u32 }
    fn encode(value: u32) -> Self {
        match value { 1 => Self::One, _ => panic!("invalid checked enum value") }
    }
}
// SAFETY: Legacy storage is exactly CUnsignedInt::Repr, with identity conversions.
unsafe impl EnumStorage<Legacy, CUnsignedInt> for u32 {
    fn decode(value: Self) -> u32 { value }
    fn encode(value: u32) -> Self { value }
}
"#;

/// Checks that target-exact numeric registration coexists with checked and legacy storage.
#[test]
fn numeric_registration_coexists_with_checked_and_legacy_storage() {
    let output = rust_oracle::run_rust(&source(
        r#"
        assert_eq!(<u32 as EnumStorage<Unsigned,CUnsignedInt>>::decode(u32::MAX), u32::MAX);
        assert_eq!(<u32 as EnumStorage<Unsigned,CUnsignedInt>>::encode(17), 17);
        assert_eq!(<core::ffi::c_ulong as EnumStorage<Wide,CUnsignedLong>>::encode(core::ffi::c_ulong::MAX), core::ffi::c_ulong::MAX);
        assert_eq!(<u64 as EnumStorage<WideLongLong,CUnsignedLongLong>>::encode(19), 19);
        assert_eq!(<u32 as EnumStorage<Legacy,CUnsignedInt>>::encode(23), 23);
        assert_eq!(<Checked as EnumStorage<Unsigned,CUnsignedInt>>::decode(Checked::One), 1);
        assert!(matches!(<Checked as EnumStorage<Unsigned,CUnsignedInt>>::encode(1), Checked::One));
        assert!(std::panic::catch_unwind(|| <Checked as EnumStorage<Unsigned,CUnsignedInt>>::encode(0)).is_err());
        "#,
    ));
    assert!(output.is_empty());
}

/// Checks that numeric registration rejects wrong ranks, storage widths, and identity pairs.
#[test]
fn numeric_registration_rejects_other_ranks_storage_and_identity_pairs() {
    let wrong_width = if core::mem::size_of::<core::ffi::c_ulong>() == 4 {
        "<u64 as EnumStorage<Wide,CUnsignedLong>>::encode(0)"
    } else {
        "<u32 as EnumStorage<Wide,CUnsignedLong>>::encode(0)"
    };
    for expression in [
        "<u64 as EnumStorage<Wide,CUnsignedLongLong>>::encode(0)",
        "<u64 as EnumStorage<Wide,CLong>>::encode(0)",
        wrong_width,
        "<u64 as EnumStorage<Unsigned,CUnsignedLong>>::encode(0)",
        "<u32 as EnumStorage<Unregistered,CUnsignedInt>>::encode(0)",
        "<Checked as EnumStorage<Wide,CUnsignedLong>>::encode(0)",
    ] {
        let diagnostics = rust_oracle::reject_rust(&source(&format!("let _ = {expression};")));
        assert!(diagnostics.contains("EnumStorage"), "{expression}: {diagnostics}");
    }
}

/// Checks that numeric registration shares exact pointer compatibility without merging
/// identities.
#[test]
fn numeric_registration_shares_exact_pointer_compatibility_without_merging_identities() {
    let output = rust_oracle::run_rust(&source(
        r#"
        use c::expression::{ArrayIdentity, CVoid, IncompleteArrayIdentity, NonVolatile};
        macro_rules! compatible_pair {
            ($name:ident, $kind:ty) => {{
                identity!($name, $kind);
                compatible::<CEnum<$name, $kind>, $kind>();
                compatible::<$kind, CEnum<$name, $kind>>();
                compatible::<CEnum<$name, $kind>, CEnum<$name, $kind>>();
                compatible::<$kind, $kind>();
                compatible::<CEnum<$name, $kind>, CVoid>();
                compatible::<CVoid, CEnum<$name, $kind>>();
            }};
        }
        compatible_pair!(BoolId, c::CBool);
        compatible_pair!(CharId, c::CChar);
        compatible_pair!(SignedCharId, c::CSignedChar);
        compatible_pair!(UnsignedCharId, c::CUnsignedChar);
        compatible_pair!(ShortId, c::CShort);
        compatible_pair!(UnsignedShortId, c::CUnsignedShort);
        compatible_pair!(IntId, c::CInt);
        compatible_pair!(UnsignedIntId, c::CUnsignedInt);
        compatible_pair!(LongId, c::CLong);
        compatible_pair!(UnsignedLongId, c::CUnsignedLong);
        compatible_pair!(LongLongId, c::CLongLong);
        compatible_pair!(UnsignedLongLongId, c::CUnsignedLongLong);
        compatible_pair!(Int128Id, c::CInt128);
        compatible_pair!(UnsignedInt128Id, c::CUnsignedInt128);
        compatible::<CEnum<Legacy, CUnsignedInt>, CUnsignedInt>();
        compatible::<CUnsignedInt, CEnum<Legacy, CUnsignedInt>>();
        compatible::<CEnum<Unregistered, CUnsignedInt>, CEnum<Unregistered, CUnsignedInt>>();
        compatible::<ArrayIdentity<c::CInt, NonVolatile, 3>, IncompleteArrayIdentity<c::CInt, NonVolatile>>();
        compatible::<IncompleteArrayIdentity<c::CInt, NonVolatile>, ArrayIdentity<c::CInt, NonVolatile, 3>>();
        "#,
    ));
    assert!(output.is_empty());
}

/// Checks that numeric pointer compatibility rejects wrong kind rank and unregistered
/// identities.
#[test]
fn numeric_pointer_compatibility_rejects_wrong_kind_rank_and_unregistered_identities() {
    for (left, right) in [
        ("CEnum<Unsigned,CUnsignedInt>", "CEnum<OtherUnsigned,CUnsignedInt>"),
        ("CEnum<Unsigned,CUnsignedInt>", "c::CInt"),
        ("CEnum<Wide,CUnsignedLong>", "CUnsignedLongLong"),
        ("CUnsignedLong", "CEnum<WideLongLong,CUnsignedLongLong>"),
        ("CEnum<PlainChar,c::CChar>", "c::CSignedChar"),
        ("c::CSignedChar", "CEnum<PlainChar,c::CChar>"),
        ("CEnum<Unsigned,c::CInt>", "c::CInt"),
        ("CEnum<Unregistered,CUnsignedInt>", "CUnsignedInt"),
        ("CUnsignedInt", "CEnum<Unregistered,CUnsignedInt>"),
    ] {
        let diagnostics =
            rust_oracle::reject_rust(&source(&format!("compatible::<{left}, {right}>();")));
        assert!(diagnostics.contains("CompatibleIdentity"), "{left} -> {right}: {diagnostics}");
    }
}

/// Checks that registered pointer compatibility rejects duplicate implementations.
#[test]
fn registered_pointer_compatibility_rejects_duplicate_implementations() {
    for implementation in [
        "impl CompatibleIdentity<CUnsignedInt> for CEnum<Unsigned,CUnsignedInt> {}",
        "impl CompatibleIdentity<CEnum<Unsigned,CUnsignedInt>> for CUnsignedInt {}",
    ] {
        let diagnostics = rust_oracle::reject_rust(&source(implementation));
        assert!(diagnostics.contains("E0119"), "{implementation}: {diagnostics}");
        assert!(diagnostics.contains("CompatibleIdentity"), "{implementation}: {diagnostics}");
    }
}
