//LICENSE Portions Copyright 2026 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.

//! Validate ordinary and bitfield access with original C record operations.
//!
//! Fresh fixture storage and compiler-owned layout feed generation. C/Rust
//! executions compare values and effects, including partially initialized
//! bitfield units, so adapters cannot read unrelated uninitialized neighbors.

/// Run original C headers through the bounded independent oracle harness.
#[path = "support/oracle.rs"]
mod oracle;
/// Compile generated consumers and paired negative cases through the bounded Rust oracle
/// harness.
#[path = "support/rust_oracle.rs"]
mod rust_oracle;

use pgrx_c_macros::{
    AnalysisSession, BindingCatalog, BitfieldBinding, EmissionStatus, FieldBinding, MacroScanner,
    RecordBinding, RecordKind, RustBindingType, emit_batch_with_bindings,
    emit_support_with_bindings, inspect,
};
use std::path::PathBuf;
use std::sync::Mutex;

/// Serialize libclang-backed inspection within this test process because its safe runtime
/// permits one active owner.
static SCANNER_LOCK: Mutex<()> = Mutex::new(());

/// Selected fixture macro names; explicit selection also exercises demand-driven adapter
/// generation.
const NAMES: &[&str] = &[
    "READ_COUNT",
    "SET_COUNT",
    "READ_CHILD",
    "READ_NEXT",
    "READ_SIGNAL",
    "ADD_COUNT",
    "PRE_COUNT",
    "POST_COUNT",
    "DOT_COUNT",
    "DEREF_COUNT",
    "SET_CHILD",
    "SET_ARRAY",
    "READ_ARRAY",
    "SIZE_ARRAY",
    "ADDRESS_COUNT",
];

/// Construct the C invocation used for both inspection and the native oracle, so
/// compiler-profile differences cannot explain a mismatch.
fn arguments() -> Vec<String> {
    let mut arguments = vec!["-std=c11".into(), "-fwrapv".into(), "-ffp-contract=off".into()];
    #[cfg(target_os = "macos")]
    {
        let sdk = rust_oracle::run_tool(
            std::process::Command::new("xcrun").arg("--show-sdk-path"),
            "locate the C oracle SDK",
        );
        arguments.extend(["-isysroot".into(), sdk.trim().into()]);
    }
    arguments
}

// Collect this fixture's actual bindgen fields. Production uses pgrx-bindgen's
// full collector; this independent fixture rejects unrecognized Rust storage.
/// Interpret fixture Rust field storage against target facts before validating it with the C
/// declaration catalog.
fn storage(ty: &syn::Type) -> Option<RustBindingType> {
    match ty {
        syn::Type::Path(ty) if ty.qself.is_none() => {
            let path = ty
                .path
                .segments
                .iter()
                .map(|segment| segment.ident.to_string())
                .collect::<Vec<_>>();
            let scalar = match path.as_slice() {
                [name] if name == "bool" => Some(RustBindingType::Bool),
                [name] if name == "u32" => {
                    Some(RustBindingType::Integer { signed: false, bits: 32 })
                }
                [name] if name == "i32" => {
                    Some(RustBindingType::Integer { signed: true, bits: 32 })
                }
                [name] if name == "u8" => Some(RustBindingType::Integer { signed: false, bits: 8 }),
                [name] if name == "i8" => Some(RustBindingType::Integer { signed: true, bits: 8 }),
                path if path.last().is_some_and(|name| name == "c_uint") => {
                    Some(RustBindingType::Integer { signed: false, bits: 32 })
                }
                path if path.last().is_some_and(|name| name == "c_ulonglong") => {
                    Some(RustBindingType::Integer { signed: false, bits: 64 })
                }
                path if path.last().is_some_and(|name| name == "c_int") => {
                    Some(RustBindingType::Integer { signed: true, bits: 32 })
                }
                path if path.last().is_some_and(|name| name == "c_uchar") => {
                    Some(RustBindingType::Integer { signed: false, bits: 8 })
                }
                path if path.last().is_some_and(|name| name == "c_char") => {
                    Some(RustBindingType::Integer { signed: true, bits: 8 })
                }
                _ => None,
            };
            scalar.or_else(|| {
                ty.path
                    .segments
                    .iter()
                    .all(|segment| segment.arguments.is_empty())
                    .then_some(RustBindingType::Named { path })
            })
        }
        syn::Type::Ptr(ty) => Some(RustBindingType::Pointer {
            pointee: Box::new(storage(&ty.elem)?),
            mutable: ty.mutability.is_some(),
        }),
        syn::Type::Array(ty) => {
            let syn::Expr::Lit(length) = &ty.len else { return None };
            let syn::Lit::Int(length) = &length.lit else { return None };
            Some(RustBindingType::Array {
                element: Box::new(storage(&ty.elem)?),
                length: length.base10_parse().ok()?,
            })
        }
        _ => None,
    }
}

/// Build the fixture's Rust binding catalog so field generation is tested against real storage
/// definitions.
fn catalog(rust_bindings: &str) -> BindingCatalog {
    let mut catalog = BindingCatalog::default();
    let file = syn::parse_file(rust_bindings).unwrap();
    for item in &file.items {
        let (name, fields, attributes, kind) = match item {
            syn::Item::Struct(item) => {
                (item.ident.clone(), item.fields.clone(), item.attrs.clone(), RecordKind::Struct)
            }
            syn::Item::Union(item) => (
                item.ident.clone(),
                syn::Fields::Named(item.fields.clone()),
                item.attrs.clone(),
                RecordKind::Union,
            ),
            _ => continue,
        };
        let fields = fields
            .into_iter()
            .filter_map(|field| {
                if !matches!(field.vis, syn::Visibility::Public(_)) {
                    return None;
                }
                let name = field.ident?.to_string();
                Some((name.clone(), FieldBinding { rust_name: name, ty: storage(&field.ty)? }))
            })
            .collect();
        let attributes = attributes
            .iter()
            .map(|attribute| quote::quote!(#attribute).to_string())
            .collect::<String>();
        let name = name.to_string();
        catalog.records.insert(
            name.clone(),
            RecordBinding {
                path: vec![name],
                kind,
                fields,
                packed: attributes.contains("packed"),
                copy: attributes.contains("Copy"),
            },
        );
    }
    for item in &file.items {
        let syn::Item::Impl(item) = item else { continue };
        let Some(RustBindingType::Named { path }) = storage(&item.self_ty) else { continue };
        for item in &item.items {
            let syn::ImplItem::Fn(method) = item else { continue };
            if method.sig.inputs.len() != 1 {
                continue;
            }
            let syn::ReturnType::Type(_, result) = &method.sig.output else { continue };
            let Some(ty) = storage(result) else { continue };
            let mut body = method.block.clone();
            let mut access = BitAccess::default();
            syn::visit_mut::VisitMut::visit_block_mut(&mut access, &mut body);
            if access.0.len() != 1 {
                continue;
            }
            let (storage_field, offset_bits, width) = access.0.pop().unwrap();
            let name = method.sig.ident.to_string();
            catalog.bitfields.insert(
                format!("{}::{name}", path.join("::")),
                BitfieldBinding {
                    record_path: path.clone(),
                    rust_name: name,
                    ty,
                    storage_field,
                    offset_bits,
                    width,
                },
            );
        }
    }
    catalog
}

/// Collect actual fixture bitfield access offsets and widths for comparison with compiler-owned
/// C field layout.
#[derive(Default)]
struct BitAccess(
    /// Recorded payload retained for exact semantic comparison.
    Vec<(String, u64, u32)>,
);
/// Traverse the parsed syntax so nested declarations participate in the same ABI or storage
/// checks.
impl syn::visit_mut::VisitMut for BitAccess {
    /// Inspect actual bindgen storage-unit reads while retaining recursive traversal of the
    /// getter expression.
    fn visit_expr_method_call_mut(&mut self, call: &mut syn::ExprMethodCall) {
        if call.method == "get"
            && call.args.len() == 2
            && let syn::Expr::Field(receiver) = call.receiver.as_ref()
            && let syn::Expr::Path(base) = receiver.base.as_ref()
            && base.path.is_ident("self")
            && let syn::Member::Named(field) = &receiver.member
            && let syn::Expr::Lit(offset) = &call.args[0]
            && let syn::Lit::Int(offset) = &offset.lit
            && let syn::Expr::Lit(width) = &call.args[1]
            && let syn::Lit::Int(width) = &width.lit
        {
            self.0.push((
                field.to_string(),
                offset.base10_parse().unwrap(),
                width.base10_parse().unwrap(),
            ));
        }
        syn::visit_mut::visit_expr_method_call_mut(self, call);
    }
}

/// Checks that public macro generation matches original C field values and evaluation.
#[test]
fn public_macro_generation_matches_original_c_field_values_and_evaluation() {
    let _lock = SCANNER_LOCK.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let directory = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let header = directory.join("tests/fixtures/field_oracle.h");
    let arguments = arguments();
    let scanner = MacroScanner::new().expect("libclang must be available");
    let frontend = inspect(&scanner, &header, &arguments, None).unwrap();
    let session = AnalysisSession::prepare(&scanner, &frontend, NAMES).unwrap();
    let rust_bindings = bindgen::Builder::default()
        .header(header.to_str().unwrap())
        .clang_args(&arguments)
        .allowlist_type("Child|Outer|Packed|Volatile|Value|Limitations")
        .derive_default(false)
        .layout_tests(false)
        .generate()
        .unwrap()
        .to_string();
    let bindings = catalog(&rust_bindings);
    let support = directory.join("../pgrx-pg-sys/src/c_macros/support.rs").canonicalize().unwrap();
    let mut rust = format!("#[path = {support:?}] pub mod __pgrx_c_macros;\n{rust_bindings}\n");
    rust.push_str(&emit_support_with_bindings(&session, NAMES, &bindings).unwrap());
    for emission in emit_batch_with_bindings(&session, NAMES, &bindings).unwrap() {
        let EmissionStatus::Emitted { rust: definition, .. } = emission.status else {
            panic!("{} must emit: {:?}", emission.analysis.name, emission.status);
        };
        rust.push_str(&definition);
    }
    rust.push_str(r#"
fn main() {
    let mut outer = core::mem::MaybeUninit::<Outer>::uninit();
    let p = outer.as_mut_ptr();
    let calls = core::cell::Cell::new(0);
    let next = || { calls.set(calls.get() + 1); p };
    unsafe {
        let assigned = SET_COUNT!(next(), -1_i32).get();
        let added = ADD_COUNT!(next(), 2_i32).get();
        let post = POST_COUNT!(next()).get();
        let pre = PRE_COUNT!(next()).get();
        let dot = DOT_COUNT!((*p)).get();
        let deref = DEREF_COUNT!(p).get();
        SET_CHILD!(p, 11_i32);
        core::ptr::addr_of_mut!((*p).next).write(core::ptr::addr_of_mut!((*p).child));
        let mut packed = core::mem::MaybeUninit::<Packed>::uninit();
        SET_COUNT!(packed.as_mut_ptr(), 641_i32);
        core::ptr::addr_of_mut!((*packed.as_mut_ptr()).signal).write_volatile(7_u8);
        let mut observable = core::mem::MaybeUninit::<Volatile>::uninit();
        core::ptr::addr_of_mut!((*observable.as_mut_ptr()).signal).write_volatile(19_u32);
        let mut value = core::mem::MaybeUninit::<Value>::uninit();
        SET_COUNT!(value.as_mut_ptr(), 23_i32);
        let mut array = core::mem::MaybeUninit::<Limitations>::uninit();
        SET_ARRAY!(array.as_mut_ptr(), -1_i32);
        let bytes = SIZE_ARRAY!({ calls.set(calls.get() + 1); array.as_mut_ptr() }).get();
        let addressed = ADDRESS_COUNT!(p).get().read();
        println!("{assigned},{added},{post},{pre},{dot},{deref},{},{},{},{},{},{},{bytes},{addressed},{}",
            READ_CHILD!(p).get(), READ_NEXT!(p).get(), READ_COUNT!(packed.as_mut_ptr()).get(),
            READ_SIGNAL!(packed.as_mut_ptr()).get(), READ_SIGNAL!(observable.as_mut_ptr()).get(),
            READ_COUNT!(value.as_mut_ptr()).get(), calls.get());
        println!("{}", READ_ARRAY!(array.as_mut_ptr()).get());
    }
}
"#);
    let expected = oracle::run_c(
        &frontend.profile().compiler.executable,
        &header,
        r#"
#include <stdio.h>
static struct Outer *p;
static unsigned int calls;
static struct Outer *next(void) { ++calls; return p; }
int main(void) {
    struct Outer outer;
    p = &outer;
    unsigned int assigned = SET_COUNT(next(), -1);
    unsigned int added = ADD_COUNT(next(), 2);
    unsigned int post = POST_COUNT(next());
    unsigned int pre = PRE_COUNT(next());
    unsigned int dot = DOT_COUNT(*p);
    unsigned int deref = DEREF_COUNT(p);
    SET_CHILD(p, 11);
    outer.next = &outer.child;
    struct Packed packed;
    SET_COUNT(&packed, 641);
    packed.signal = 7;
    struct Volatile observable;
    observable.signal = 19;
    union Value value;
    SET_COUNT(&value, 23);
    struct Limitations array;
    SET_ARRAY(&array, -1);
    size_t bytes = SIZE_ARRAY((++calls, &array));
    unsigned int addressed = *ADDRESS_COUNT(p);
    printf("%u,%u,%u,%u,%u,%u,%u,%u,%u,%u,%u,%u,%zu,%u,%u\n", assigned, added, post, pre, dot, deref,
        READ_CHILD(p), READ_NEXT(p), READ_COUNT(&packed), READ_SIGNAL(&packed),
        READ_SIGNAL(&observable), READ_COUNT(&value), bytes, addressed, calls);
    printf("%u\n", READ_ARRAY(&array));
    return 0;
}
"#,
        &arguments.iter().map(String::as_str).collect::<Vec<_>>(),
        true,
    );
    assert_eq!(
        rust_oracle::run_rust(&rust),
        expected,
        "public emission must preserve original C field conversions and evaluation counts"
    );
}

/// Checks that generated bitfield accessors match original C with uninitialized neighbor bits.
#[test]
fn generated_bitfield_accessors_match_original_c_with_uninitialized_neighbor_bits() {
    let _lock = SCANNER_LOCK.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let names = [
        "BIT_READ",
        "BIT_SET",
        "BIT_ADD",
        "BIT_PRE",
        "BIT_POST",
        "BIT_NOT",
        "BIT_POST_NOT",
        "BIT_SIGNED",
        "BIT_SIGNED_SET",
        "BIT_FULL",
        "BIT_FULL_SET",
        "BIT_WIDE",
        "BIT_WIDE_SET",
        "BIT_TRUTH",
        "BIT_TRUTH_SET",
        "BIT_LEFT",
        "BIT_LEFT_SET",
        "BIT_RIGHT",
        "BIT_RIGHT_SET",
        "BIT_SENTINEL",
        "BIT_SENTINEL_SET",
        "BIT_NESTED",
        "BIT_NESTED_SET",
        "BIT_SMALL",
        "BIT_SMALL_SET",
        "BIT_SMALL_COMMA_SIZE",
        "BIT_SMALL_ASSIGN_SIZE",
        "BIT_SMALL_PRE_SIZE",
        "BIT_SMALL_POST_SIZE",
        "BIT_SMALL_COMMA_NOT",
        "BIT_SMALL_ASSIGN_NOT",
        "BIT_SMALL_PRE_NOT",
        "BIT_SMALL_POST_NOT",
        "BIT_SIZE_INVALID",
        "BIT_ADDRESS_INVALID",
    ];
    let directory = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let header = directory.join("tests/fixtures/bitfield_oracle.h");
    let arguments = arguments();
    let scanner = MacroScanner::new().unwrap();
    let frontend = inspect(&scanner, &header, &arguments, None).unwrap();
    let session = AnalysisSession::prepare(&scanner, &frontend, &names).unwrap();
    let rust_bindings = bindgen::Builder::default()
        .header(header.to_str().unwrap())
        .clang_args(&arguments)
        .allowlist_type("BitFields|PackedBits|PackedContainer|VolatileBits")
        .derive_default(false)
        .layout_tests(false)
        .generate()
        .unwrap()
        .to_string();
    let bindings = catalog(&rust_bindings);
    let support = directory.join("../pgrx-pg-sys/src/c_macros/support.rs").canonicalize().unwrap();
    let mut rust = format!("#[path = {support:?}] pub mod __pgrx_c_macros;\n{rust_bindings}\n");
    let artifact =
        pgrx_c_macros::emit_support_artifact_with_bindings(&session, &names, &bindings).unwrap();
    rust.push_str(&artifact.rust);
    let c_support = artifact.c_source;
    assert!(!c_support.is_empty(), "bitfields must have original-C access primitives");
    assert!(!c_support.contains("BIT_READ"), "native support must not hand-port macro bodies");
    for emission in emit_batch_with_bindings(&session, &names, &bindings).unwrap() {
        let EmissionStatus::Emitted { rust: definition, .. } = emission.status else {
            panic!("{} must emit: {:?}", emission.analysis.name, emission.status);
        };
        rust.push_str(&definition);
    }
    let declarations = rust.clone();
    rust.push_str(r#"
fn main() {
    use __pgrx_c_macros::{CUnsignedLongLong,CValue};
    let mut partial = core::mem::MaybeUninit::<BitFields>::uninit();
    let p = partial.as_mut_ptr();
    let calls = core::cell::Cell::new(0_u32);
    let next = || { calls.set(calls.get()+1); p };
    unsafe {
        println!("partial,{},{},{}",BIT_SET!(next(),2147483648.0_f64).get(),BIT_SIGNED_SET!(p,-7_i32).get(),calls.get());
        BIT_LEFT_SET!(p,7_u32); BIT_RIGHT_SET!(p,0x003fffff_u32); BIT_SENTINEL_SET!(p,0x05abcdef_u32);
        let mut seed = 0x51e027a9_u32;
        for i in 0..64_i32 {
            seed=seed.wrapping_mul(1664525).wrapping_add(1013904223);
            let assigned = BIT_SET!(next(),seed).get();
            let added = BIT_ADD!(p,3_i32).get();
            let post = BIT_POST!(p).get();
            let pre = BIT_PRE!(p).get();
            let inverse = BIT_NOT!(p).get();
            let signed = BIT_SIGNED_SET!(p,i-32_i32).get();
            let full = BIT_FULL_SET!(p,seed).get();
            let wide = BIT_WIDE_SET!(p,CValue::<CUnsignedLongLong>::new((u64::from(seed)<<18)|(i as u64))).get();
            let truth = u32::from(BIT_TRUTH_SET!(p,i&1_i32).get());
            println!("{i},{assigned},{added},{post},{pre},{inverse},{signed},{full},{wide},{truth},{},{},{},{}",
                BIT_READ!(p).get(),BIT_LEFT!(p).get(),BIT_RIGHT!(p).get(),BIT_SENTINEL!(p).get());
        }
        BIT_SET!(p,127_u32);
        println!("postfix,{},{},{}",BIT_POST_NOT!(p).get(),BIT_READ!(p).get(),calls.get());
        BIT_SMALL_SET!(p,127_i32);
        let comma_size=BIT_SMALL_COMMA_SIZE!(next()).get();
        let assign_size=BIT_SMALL_ASSIGN_SIZE!(next()).get();
        let pre_size=BIT_SMALL_PRE_SIZE!(next()).get();
        let post_size=BIT_SMALL_POST_SIZE!(next()).get();
        let comma_not=BIT_SMALL_COMMA_NOT!(p).get();
        let assign_not=BIT_SMALL_ASSIGN_NOT!(p).get();
        let pre_not=BIT_SMALL_PRE_NOT!(p).get();
        let post_not=BIT_SMALL_POST_NOT!(p).get();
        println!("identity,{comma_size},{assign_size},{pre_size},{post_size},{comma_not},{assign_not},{pre_not},{post_not},{},{}",BIT_SMALL!(p).get(),calls.get());
        let mut packed = core::mem::MaybeUninit::<PackedBits>::uninit();
        let mut nested = core::mem::MaybeUninit::<PackedContainer>::uninit();
        let mut observable = core::mem::MaybeUninit::<VolatileBits>::uninit();
        let a=BIT_SET!(packed.as_mut_ptr(),255_u32).get();
        let b=BIT_NESTED_SET!(nested.as_mut_ptr(),129_u32).get();
        let c=BIT_SET!(observable.as_mut_ptr(),-1_i32).get();
        let d=u32::from(BIT_TRUTH_SET!(observable.as_mut_ptr(),2_i32).get());
        println!("packed,{a},{},{b},{},{c},{},{d},{},{}",BIT_READ!(packed.as_mut_ptr()).get(),BIT_NESTED!(nested.as_mut_ptr()).get(),BIT_READ!(observable.as_mut_ptr()).get(),u32::from(BIT_TRUTH!(observable.as_mut_ptr()).get()),calls.get());
    }
}
"#);
    let c = r#"
#include <stdio.h>
static struct BitFields *p;
static unsigned int calls;
static struct BitFields *next(void) { ++calls; return p; }
int main(void) {
    struct BitFields partial;
    p=&partial;
    int first = BIT_SET(next(),2147483648.0);
    int second = BIT_SIGNED_SET(p,-7);
    printf("partial,%d,%d,%u\n",first,second,calls);
    BIT_LEFT_SET(p,7u); BIT_RIGHT_SET(p,0x003fffffu); BIT_SENTINEL_SET(p,0x05abcdefu);
    unsigned int seed=0x51e027a9u;
    for (int i=0;i<64;++i) {
        seed=seed*1664525u+1013904223u;
        int assigned=BIT_SET(next(),seed);
        int added=BIT_ADD(p,3);
        int post=BIT_POST(p);
        int pre=BIT_PRE(p);
        int inverse=BIT_NOT(p);
        int signed_value=BIT_SIGNED_SET(p,i-32);
        unsigned int full=BIT_FULL_SET(p,seed);
        unsigned long long wide=BIT_WIDE_SET(p,((unsigned long long)seed<<18)|(unsigned long long)i);
        int truth=BIT_TRUTH_SET(p,i&1);
        printf("%d,%d,%d,%d,%d,%d,%d,%u,%llu,%d,%d,%d,%d,%d\n",i,assigned,added,post,pre,inverse,signed_value,full,wide,truth,
            BIT_READ(p),BIT_LEFT(p),BIT_RIGHT(p),BIT_SENTINEL(p));
    }
    BIT_SET(p,127u);
    unsigned int postfix_inverse = BIT_POST_NOT(p);
    printf("postfix,%u,%d,%u\n",postfix_inverse,BIT_READ(p),calls);
    BIT_SMALL_SET(p,127);
    size_t comma_size=BIT_SMALL_COMMA_SIZE(next());
    size_t assign_size=BIT_SMALL_ASSIGN_SIZE(next());
    size_t pre_size=BIT_SMALL_PRE_SIZE(next());
    size_t post_size=BIT_SMALL_POST_SIZE(next());
    int comma_not=BIT_SMALL_COMMA_NOT(p);
    int assign_not=BIT_SMALL_ASSIGN_NOT(p);
    int pre_not=BIT_SMALL_PRE_NOT(p);
    unsigned long long post_not=BIT_SMALL_POST_NOT(p);
    printf("identity,%zu,%zu,%zu,%zu,%d,%d,%d,%llu,%llu,%u\n",comma_size,assign_size,pre_size,post_size,comma_not,assign_not,pre_not,post_not,(unsigned long long)BIT_SMALL(p),calls);
    struct PackedBits packed;
    struct PackedContainer nested;
    struct VolatileBits observable;
    int a=BIT_SET(&packed,255u);
    int b=BIT_NESTED_SET(&nested,129u);
    int c=BIT_SET(&observable,-1);
    int d=BIT_TRUTH_SET(&observable,2);
    printf("packed,%d,%d,%d,%d,%d,%d,%d,%d,%u\n",a,BIT_READ(&packed),b,BIT_NESTED(&nested),c,BIT_READ(&observable),d,BIT_TRUTH(&observable),calls);
    return 0;
}
"#;
    let profile = frontend.profile();
    let args = arguments.iter().map(String::as_str).collect::<Vec<_>>();
    let expected = oracle::run_c(&profile.compiler.executable, &header, c, &args, true);
    let actual = rust_oracle::run_rust_linked(
        &rust,
        &profile.compiler.executable,
        &header,
        &c_support,
        &args,
    );
    assert_eq!(actual.lines().count(), expected.lines().count(), "bitfield corpus completeness");
    for (index, (actual, expected)) in actual.lines().zip(expected.lines()).enumerate() {
        assert_eq!(
            actual, expected,
            "bitfield conversions, access units and evaluation counts differ at oracle row {index}"
        );
    }
    for (name, code) in [("BIT_SIZE_INVALID", "SizeablePlace"), ("BIT_ADDRESS_INVALID", "Place")] {
        let source = format!(
            "{declarations}\nfn main() {{ let mut p=core::mem::MaybeUninit::<BitFields>::uninit(); unsafe {{ let _={name}!(p.as_mut_ptr()); }} }}"
        );
        let diagnostics = rust_oracle::reject_rust(&source);
        assert!(
            diagnostics.contains(code),
            "illegal C bitfield expression must fail with its place constraint: {diagnostics}"
        );
        rust_oracle::reject_c_invocation(
            &profile.compiler.executable,
            &header,
            &format!("void test(struct BitFields*p) {{ (void){name}(p); }}"),
            &args,
        );
    }
}
