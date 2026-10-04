//LICENSE Portions Copyright 2026 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.

#[path = "support/oracle.rs"]
mod oracle;

use pgrx_c_macros::{
    ArrayKind, DeclarationLinkage, FieldInfo, FloatingKind, FrontendError, IntegerKind,
    MacroScanner, RecordInfo, RecordKind, TypeCategory, TypeShapeKind, inspect,
};
use std::path::PathBuf;
use std::sync::Mutex;

static SCANNER_LOCK: Mutex<()> = Mutex::new(());

fn field<'a>(record: &'a RecordInfo, name: &str) -> &'a FieldInfo {
    record.fields.iter().find(|field| field.name.as_deref() == Some(name)).unwrap()
}

#[test]
fn compiler_size_identity_and_offset_capability_match_independent_c_assertions() {
    let _lock = SCANNER_LOCK.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let scanner = MacroScanner::new().unwrap();
    let header = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/frontend_types.h");
    let output = inspect(&scanner, &header, &["-std=c17".into()], None).unwrap();
    let target = &output.profile().target;
    let spelling = match target.size_type {
        IntegerKind::UnsignedChar => "unsigned char",
        IntegerKind::UnsignedShort => "unsigned short",
        IntegerKind::UnsignedInt => "unsigned int",
        IntegerKind::UnsignedLong => "unsigned long",
        IntegerKind::UnsignedLongLong => "unsigned long long",
        IntegerKind::UnsignedInt128 => "unsigned __int128",
        other => panic!("sizeof must have an unsigned canonical identity, found {other:?}"),
    };
    assert!(
        target.offsetof_supported,
        "Clang must prove its intrinsic for this ordinary C profile"
    );
    let record = &output.declarations().records["struct FrontNode"];
    let matrix_offset = field(record, "matrix").offset_bits.unwrap()
        + 5 * u64::from(target.integers[&IntegerKind::Int].bits);
    let source = format!(
        "_Static_assert(_Generic(sizeof(0), {spelling}: 1, default: 0), \"sizeof exact identity\");\n\
         _Static_assert(_Generic(_Alignof(int), {spelling}: 1, default: 0), \"alignof exact identity\");\n\
         _Static_assert(_Generic(__builtin_offsetof(struct FrontNode, matrix[1][2]), {spelling}: 1, default: 0), \"offsetof exact identity\");\n\
         _Static_assert(__builtin_offsetof(struct FrontNode, matrix[1][2]) * __CHAR_BIT__ == {matrix_offset}, \"nested array offset\");\n"
    );
    assert!(
        oracle::run_c(
            &output.profile().compiler.executable,
            &header,
            &source,
            &output.profile().arguments.iter().map(String::as_str).collect::<Vec<_>>(),
            false,
        )
        .is_empty()
    );
    let json = serde_json::to_value(target).unwrap();
    assert_eq!(json["size_type"], serde_json::to_value(target.size_type).unwrap());
    assert_eq!(json["offsetof_supported"], true);
}

#[test]
fn intrinsic_macro_shadow_disables_only_offsets_and_cannot_spoof_required_type_proofs() {
    let _lock = SCANNER_LOCK.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let scanner = MacroScanner::new().unwrap();
    let header = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/frontend_types.h");
    let output = inspect(
        &scanner,
        &header,
        &["-std=c17".into(), "-D__builtin_offsetof(T,M)=0".into()],
        None,
    )
    .unwrap();
    assert!(!output.profile().target.offsetof_supported);
    assert!(output.declarations().records.contains_key("struct FrontNode"));
    assert!(output.declarations().function_signatures.contains_key("front_external"));
    assert!(!output.profile().target.integers[&output.profile().target.size_type].signed);
    let rewritten_field =
        inspect(&scanner, &header, &["-std=c17".into(), "-Dfirst=0".into()], None).unwrap();
    assert!(!rewritten_field.profile().target.offsetof_supported);
    assert!(rewritten_field.declarations().function_signatures.contains_key("front_external"));
    let error = inspect(
        &scanner,
        &header,
        &["-std=c17".into(), "-D__builtin_types_compatible_p(a,b)=1".into()],
        None,
    )
    .unwrap_err();
    assert!(
        matches!(error, FrontendError::Environment(message) if message.contains("unshadowed __builtin_types_compatible_p"))
    );
}

#[test]
fn compiler_catalog_retains_type_edges_signatures_and_record_layout() {
    let _lock = SCANNER_LOCK.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let scanner = MacroScanner::new().expect("libclang must be available");
    let header = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/frontend_types.h");
    let output =
        inspect(&scanner, &header, &["-std=c17".into()], None).expect("inspect C declarations");
    let catalog = output.declarations();
    let record = &catalog.records["struct FrontNode"];
    assert_eq!(record.kind, RecordKind::Struct);
    assert_eq!(record.name.as_deref(), Some("FrontNode"));
    assert!(!record.is_anonymous);
    assert!(!record.identity.is_empty());
    assert_eq!(field(record, "flags").bit_width, Some(7));
    assert_eq!(field(record, "code").bit_width, Some(5));
    assert!(record.fields.iter().any(|field| field.bit_width == Some(0)));
    assert_eq!(field(record, "flags").ty.category, TypeCategory::Integer(IntegerKind::UnsignedInt));
    assert_eq!(field(record, "code").ty.category, TypeCategory::Integer(IntegerKind::Int));
    assert!(field(record, "tick").ty.is_volatile);
    assert_eq!(field(record, "next").ty.canonical_spelling, "struct FrontNode *");
    let TypeShapeKind::Pointer { pointee } = &catalog.type_shapes["struct FrontNode *"].kind else {
        panic!("recursive record edge must be a pointer");
    };
    assert_eq!(pointee.canonical_spelling, "struct FrontNode");
    let TypeShapeKind::Record { identity } = &catalog.type_shapes["struct FrontNode"].kind else {
        panic!("record edge must retain its identity");
    };
    assert_eq!(identity, &record.identity);
    let TypeShapeKind::Pointer { pointee } = &catalog.type_shapes["const struct FrontNode *"].kind
    else {
        panic!("const qualification belongs to the pointee");
    };
    assert!(pointee.is_const);
    assert!(!catalog.type_shapes["const struct FrontNode *"].ty.is_const);
    let qualified = &catalog.variables["front_const_pointer"];
    assert!(qualified.is_const);
    let TypeShapeKind::Pointer { pointee } =
        &catalog.type_shapes[&qualified.canonical_spelling].kind
    else {
        panic!("typedef pointer qualifications must survive canonicalization");
    };
    assert!(pointee.is_const && pointee.is_volatile);
    let TypeShapeKind::Array { element, length, array_kind } =
        &catalog.type_shapes["int[2][3]"].kind
    else {
        panic!("multidimensional arrays retain immediate element edges");
    };
    assert_eq!(*length, Some(2));
    assert_eq!(*array_kind, ArrayKind::Constant);
    assert_eq!(element.canonical_spelling, "int[3]");
    let TypeShapeKind::Array { length, array_kind, .. } =
        &catalog.type_shapes["unsigned char[]"].kind
    else {
        panic!("flexible arrays remain incomplete");
    };
    assert_eq!(*length, None);
    assert_eq!(*array_kind, ArrayKind::Incomplete);
    assert_eq!(catalog.records["union FrontChoice"].kind, RecordKind::Union);
    assert_eq!(catalog.type_shapes["struct FrontPacked"].ty.category, TypeCategory::Record);
    let anonymous = record
        .fields
        .iter()
        .find(|field| field.name.is_none() && field.bit_width.is_none())
        .expect("anonymous union field");
    assert!(anonymous.is_anonymous);
    assert!(catalog.records[&anonymous.ty.canonical_spelling].is_anonymous);
    assert_eq!(catalog.records[&anonymous.ty.canonical_spelling].kind, RecordKind::Union);
    assert_eq!(catalog.records["struct FrontIncomplete"].size, None);
    assert!(catalog.records["struct FrontIncomplete"].fields.is_empty());

    let function = &catalog.function_signatures["front_static"];
    assert_eq!(function.linkage, Some(DeclarationLinkage::Internal));
    assert!(function.is_static && function.is_inline);
    assert_eq!(
        function.signature.result.category,
        TypeCategory::Integer(IntegerKind::UnsignedLongLong)
    );
    assert_eq!(function.signature.parameters.as_ref().unwrap().len(), 2);
    assert!(
        catalog.type_shapes[&function.signature.parameters.as_ref().unwrap()[0].canonical_spelling]
            .is_restrict
    );
    let function = &catalog.function_signatures["front_external"];
    assert_eq!(function.linkage, Some(DeclarationLinkage::External));
    assert!(!function.is_static && !function.is_inline && !function.definition_available);
    assert!(!function.signature.variadic);
    assert!(function.signature.calling_convention.is_some());
    assert_eq!(
        catalog.function_signatures["front_void"].signature.parameters.as_ref().unwrap().len(),
        0
    );
    assert!(catalog.function_signatures["front_unprototyped"].signature.parameters.is_none());
    assert!(catalog.function_signatures["front_variadic"].signature.variadic);
    let callback = &catalog.variables["front_callback"];
    let TypeShapeKind::Pointer { pointee } =
        &catalog.type_shapes[&callback.canonical_spelling].kind
    else {
        panic!("function-pointer typedef must retain its pointee signature");
    };
    let TypeShapeKind::Function { signature } =
        &catalog.type_shapes[&pointee.canonical_spelling].kind
    else {
        panic!("function pointer target must have a signature");
    };
    assert_eq!(signature.parameters.as_ref().unwrap().len(), 2);
    assert_eq!(signature.result.category, TypeCategory::Integer(IntegerKind::Int));

    // Compare the catalog with independent native C layout/type assertions.
    let mut source = String::new();
    for name in ["struct FrontNode", "union FrontChoice", "struct FrontPacked"] {
        let record = &catalog.records[name];
        source.push_str(&format!(
            "_Static_assert(sizeof({name}) == {}, \"{name} size\");\n_Static_assert(_Alignof({name}) == {}, \"{name} alignment\");\n",
            record.size.unwrap(), record.alignment.unwrap(),
        ));
        for field in
            record.fields.iter().filter(|field| field.name.is_some() && field.bit_width.is_none())
        {
            let field_name = field.name.as_ref().unwrap();
            source.push_str(&format!(
                "_Static_assert(__builtin_offsetof({name}, {field_name}) * __CHAR_BIT__ == {}, \"{name}.{field_name} offset\");\n",
                field.offset_bits.unwrap(),
            ));
        }
    }
    source.push_str(
        "_Static_assert(_Generic(&front_external, int (*)(const FrontNode *, unsigned long): 1, default: 0), \"external signature\");\n_Static_assert(_Generic(&front_static, unsigned long long (*)(FrontNode *, unsigned long long): 1, default: 0), \"static signature\");\n",
    );
    assert!(
        oracle::run_c(
            &output.profile().compiler.executable,
            &header,
            &source,
            &output.profile().arguments.iter().map(String::as_str).collect::<Vec<_>>(),
            false,
        )
        .is_empty()
    );
    // No Clang lifetimes or recursively serialized record graphs escape inspection.
    drop(scanner);
    let json = serde_json::to_string(catalog).expect("owned graph is serializable");
    assert!(json.len() < 200_000, "small cyclic fixture must remain bounded");
}

#[test]
fn verified_float_profile_preserves_c_representation_and_evaluation_facts() {
    let _lock = SCANNER_LOCK.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let scanner = MacroScanner::new().expect("libclang must be available");
    let header = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/frontend_types.h");
    let output = inspect(&scanner, &header, &["-std=c17".into(), "-ffp-contract=off".into()], None)
        .expect("inspect a noncontracting C float profile");
    let profile = output.profile();
    profile.supports_float_values().expect("verified float representations and conversions");
    profile.supports_float_expressions().expect("verified binary32 and binary64 profile");
    assert!(
        profile.unsupported_options.is_empty(),
        "float modes must not reject integer expressions"
    );
    let facts = &profile.target.floating_point;
    assert_eq!(facts.evaluation_method, Some(0));
    assert!(!facts.fast_math && !facts.finite_math_only);
    assert!(facts.effective_options.iter().any(|option| option == "-ffp-contract=off"));
    let mut original = String::new();
    for (kind, spelling, prefix) in [
        (FloatingKind::Float, "float", "__FLT"),
        (FloatingKind::Double, "double", "__DBL"),
        (FloatingKind::LongDouble, "long double", "__LDBL"),
    ] {
        let ty = &facts.types[&kind];
        original.push_str(&format!(
            "_Static_assert(sizeof({spelling}) * __CHAR_BIT__ == {}, \"storage\");\n\
             _Static_assert(_Alignof({spelling}) == {}, \"alignment\");\n\
             _Static_assert(__FLT_RADIX__ == {}, \"radix\");\n\
             _Static_assert({prefix}_MANT_DIG__ == {}, \"mantissa\");\n\
             _Static_assert({prefix}_MIN_EXP__ == {}, \"minimum exponent\");\n\
             _Static_assert({prefix}_MAX_EXP__ == {}, \"maximum exponent\");\n",
            ty.storage_bits,
            ty.alignment.unwrap(),
            ty.radix,
            ty.mantissa_digits,
            ty.min_exponent,
            ty.max_exponent,
        ));
    }
    original.push_str(
        r#"
_Static_assert(__FLT_EVAL_METHOD__ == 0, "no excess precision");
_Static_assert(_Generic(1.0f + 1, float: 1, default: 0), "integer converts to float");
_Static_assert(_Generic(1.0f + 1.0, double: 1, default: 0), "usual conversion to double");
_Static_assert(((1.0f + 16777216.0f) - 16777216.0f) == 0.0f, "binary32 rounding");
_Static_assert(((1.0 + 9007199254740992.0) - 9007199254740992.0) == 0.0, "binary64 rounding");
"#,
    );
    assert!(
        oracle::run_c(
            &profile.compiler.executable,
            &header,
            &original,
            &profile.arguments.iter().map(String::as_str).collect::<Vec<_>>(),
            false
        )
        .is_empty()
    );
    let json = serde_json::to_value(profile).unwrap();
    assert_eq!(json["target"]["floating_point"]["types"]["float"]["mantissa_digits"], 24);
    assert_eq!(json["target"]["floating_point"]["evaluation_method"], 0);
    let mut unsupported = profile.target.clone();
    unsupported.char_bits = 16;
    assert!(unsupported.supports_float_expressions().unwrap_err().contains("eight-bit"));
    unsupported.char_bits = profile.target.char_bits;
    unsupported.floating_point = Default::default();
    assert!(
        unsupported.supports_float_expressions().is_err(),
        "missing compiler proof must reject"
    );
    assert!(unsupported.supports_float_values().is_err());
    let restored = inspect(
        &scanner,
        &header,
        &["-ffast-math".into(), "-fno-fast-math".into(), "-ffp-contract=off".into()],
        None,
    )
    .expect("the compiler resolves option overrides");
    restored
        .profile()
        .supports_float_expressions()
        .expect("effective overrides restore ordinary float semantics");

    let default = inspect(&scanner, &header, &["-std=c17".into()], None)
        .expect("inspect the ordinary contract-on C profile");
    default
        .profile()
        .supports_float_values()
        .expect("contraction does not alter storage, loads, comparisons or prototype conversions");
    assert!(
        default.profile().supports_float_expressions().is_err(),
        "the values proof must not authorize composed arithmetic that may contract"
    );
}

#[test]
fn floating_point_modes_that_rust_operators_cannot_preserve_are_rejected() {
    let _lock = SCANNER_LOCK.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let scanner = MacroScanner::new().expect("libclang must be available");
    let header = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/frontend_types.h");
    for (options, explanation) in [
        (vec!["-std=c17"], "-ffp-contract=on"),
        (vec!["-ffp-contract=off", "-ffast-math"], "fast-math"),
        (vec!["-ffp-contract=fast"], "-ffp-contract=fast"),
        (vec!["-ffp-contract=off", "-frounding-math"], "-frounding-math"),
        (
            vec!["-ffp-contract=off", "-fdenormal-fp-math=positive-zero"],
            "-fdenormal-fp-math=positive-zero",
        ),
        (vec!["-ffp-contract=off", "-ffp-eval-method=double"], "excess precision"),
    ] {
        let options = options.into_iter().map(str::to_owned).collect::<Vec<_>>();
        let output =
            inspect(&scanner, &header, &options, None).expect("inspect a supported Clang option");
        let error = output
            .profile()
            .supports_float_expressions()
            .expect_err("unmodeled float semantics must reject");
        assert!(error.contains(explanation), "{options:?}: {error}");
        if options == ["-std=c17"] {
            output.profile().supports_float_values().unwrap();
        } else {
            assert!(
                output.profile().supports_float_values().is_err(),
                "the representation/conversion gate must retain unsafe mode rejection: {options:?}"
            );
        }
        assert!(
            output.profile().unsupported_options.is_empty(),
            "float-only modes must not prevent integer macro conversion"
        );
    }
}
