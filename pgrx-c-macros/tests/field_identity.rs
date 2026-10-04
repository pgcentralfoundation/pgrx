//LICENSE Portions Copyright 2026 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.

#![cfg(all(target_pointer_width = "64", not(target_os = "windows")))]

#[path = "../../pgrx-bindgen/src/build/binding_symbols.rs"]
mod binding_symbols;
#[path = "support/oracle.rs"]
mod oracle;
#[path = "support/rust_oracle.rs"]
#[allow(dead_code)]
mod rust_oracle;

use pgrx_c_macros::{
    AnalysisSession, EmissionStatus, MacroGeneration, MacroScanner, generate_with_bindings, inspect,
};
use proc_macro2::{Delimiter, TokenTree};
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

fn is_builtin_oid(name: &str) -> bool {
    name.ends_with("OID") && name != "HEAP_HASOID"
        || name.ends_with("RelationId")
        || name == "TemplateDbOid"
}

const NAMES: &[&str] = &[
    "IDENTITY_READ",
    "IDENTITY_WRITE",
    "IDENTITY_OFFSET",
    "IDENTITY_PRIMARY",
    "IDENTITY_OTHER",
    "IDENTITY_PROMOTED",
    "IDENTITY_BITS_READ",
    "IDENTITY_BITS_WRITE",
    "IDENTITY_KEYWORD",
    "IDENTITY_SKIPPED",
];

// Read the actual emitted token routes. The test deliberately does not reproduce
// the private registry's ID allocation or derive IDs from the fixture's names.
fn field_routes(source: &str) -> BTreeMap<String, String> {
    let file = syn::parse_file(source).unwrap();
    let registry = file
        .items
        .iter()
        .find_map(|item| match item {
            syn::Item::Macro(item)
                if item.ident.as_ref().is_some_and(|name| name == "__pgrx_c_field_marker") =>
            {
                Some(item)
            }
            _ => None,
        })
        .expect("generated field selector");
    let tokens = registry.mac.tokens.clone().into_iter().collect::<Vec<_>>();
    let mut routes = BTreeMap::new();
    for window in tokens.windows(4) {
        let [
            TokenTree::Group(matcher),
            TokenTree::Punct(equal),
            TokenTree::Punct(arrow),
            TokenTree::Group(body),
        ] = window
        else {
            continue;
        };
        if matcher.delimiter() != Delimiter::Parenthesis
            || equal.as_char() != '='
            || arrow.as_char() != '>'
        {
            continue;
        }
        let matcher = matcher.stream().into_iter().collect::<Vec<_>>();
        let [TokenTree::Ident(name)] = matcher.as_slice() else { continue };
        let name = name.to_string().trim_start_matches("r#").to_owned();
        let route = body.stream().to_string();
        if let Some(previous) = routes.insert(name.clone(), route.clone()) {
            assert_eq!(previous, route, "raw and ordinary {name} tokens must have one identity");
        }
    }
    routes
}

fn local_marker(route: &str) -> String {
    let tokens = route.parse::<proc_macro2::TokenStream>().unwrap().into_iter().collect::<Vec<_>>();
    let [TokenTree::Punct(dollar), TokenTree::Ident(root), ..] = tokens.as_slice() else {
        panic!("field route must start with $crate: {route}")
    };
    assert_eq!(dollar.as_char(), '$');
    assert_eq!(root.to_string(), "crate");
    let path = syn::parse2::<syn::Path>(tokens.into_iter().skip(1).collect()).unwrap();
    assert_eq!(path.segments.len(), 3, "the marker must reside in the defining generated module");
    assert_eq!(path.segments[1].ident, "__pgrx_c_generated");
    assert!(
        path.segments.iter().all(|segment| segment.arguments.is_empty()),
        "field markers must be nominal nongeneric types: {route}"
    );
    path.segments.last().unwrap().ident.to_string()
}

fn definitions(generation: &MacroGeneration) -> String {
    let mut rust = String::new();
    for emission in &generation.macros {
        match &emission.status {
            EmissionStatus::Emitted { rust: definition, .. } => rust.push_str(definition),
            EmissionStatus::Skipped { reason } if emission.analysis.name == "IDENTITY_SKIPPED" => {
                assert!(!reason.message.is_empty());
            }
            _ => panic!("fixture macro must emit: {emission:?}"),
        }
    }
    rust
}

#[test]
fn local_field_identities_survive_selection_skips_and_cross_crate_expansion() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let header = root.join("tests/fixtures/field_identity.h");
    let support = root.join("../pgrx-pg-sys/src/c_macros/support.rs").canonicalize().unwrap();
    let mut arguments = vec!["-std=c17".into(), "-O2".into(), "-ffp-contract=off".into()];
    #[cfg(target_os = "macos")]
    {
        let sdk = rust_oracle::run_tool(
            Command::new("xcrun").arg("--show-sdk-path"),
            "field_identity_sdk",
        );
        arguments.extend(["-isysroot".into(), sdk.trim().into()]);
    }
    let scanner = MacroScanner::new().unwrap();
    let frontend = inspect(&scanner, &header, &arguments, None).unwrap();
    let session = AnalysisSession::prepare(&scanner, &frontend, NAMES).unwrap();
    let native = bindgen::Builder::default()
        .rust_target(bindgen::RustTarget::stable(85, 0).unwrap())
        .rust_edition(bindgen::RustEdition::Edition2024)
        .header(header.to_str().unwrap())
        .clang_args(&frontend.profile().arguments)
        .allowlist_type("Identity(Ordinary|Other|Promoted|Bits|Keyword)")
        .derive_copy(false)
        .wrap_unsafe_ops(true)
        .layout_tests(false)
        .generate_comments(false)
        .formatter(bindgen::Formatter::None)
        .generate()
        .unwrap()
        .to_string();
    let catalog = binding_symbols::collect_bindings(
        &syn::parse_file(&native).unwrap(),
        session.integer_constants(),
        frontend.declarations(),
        &frontend.profile().target,
    );
    assert!(!catalog.records.contains_key("IdentityDiscarded"));
    let full = generate_with_bindings(&session, NAMES, &catalog).unwrap();
    assert!(matches!(full.macros.last().unwrap().status, EmissionStatus::Skipped { .. }));
    let macros = definitions(&full);
    let routes = field_routes(&full.support.rust);
    assert!(
        !routes.contains_key("aa_discarded"),
        "rejected roots must not retain field capabilities"
    );
    for name in ["value", "frozen", "type"] {
        assert!(routes.contains_key(name));
    }
    let parsed = syn::parse_file(&full.support.rust).unwrap();
    let generated = parsed
        .items
        .iter()
        .find_map(|item| match item {
            syn::Item::Mod(module) if module.ident == "__pgrx_c_generated" => {
                module.content.as_ref().map(|(_, items)| items)
            }
            _ => None,
        })
        .unwrap();
    let markers = routes.values().map(|route| local_marker(route)).collect::<BTreeSet<_>>();
    assert_eq!(
        markers.len(),
        routes.len(),
        "different C field names must have distinct local identities"
    );
    for marker in &markers {
        let definitions = generated
            .iter()
            .filter_map(|item| match item {
                syn::Item::Struct(item) if item.ident == marker.as_str() => Some(item),
                _ => None,
            })
            .collect::<Vec<_>>();
        assert_eq!(definitions.len(), 1, "field route {marker} must resolve to one local struct");
        assert!(definitions[0].generics.params.is_empty());
    }
    let value_marker = local_marker(&routes["value"]);
    let owners = generated
        .iter()
        .filter_map(|item| {
            let syn::Item::Impl(item) = item else { return None };
            let (_, trait_path, _) = item.trait_.as_ref()?;
            let capability = trait_path.segments.last()?;
            if capability.ident != "OrdinaryField" && capability.ident != "Field" {
                return None;
            }
            let syn::PathArguments::AngleBracketed(arguments) = &capability.arguments else {
                return None;
            };
            let syn::GenericArgument::Type(syn::Type::Path(marker)) = arguments.args.first()?
            else {
                return None;
            };
            if marker.path.segments.last()?.ident != value_marker {
                return None;
            }
            let syn::Type::Path(owner) = item.self_ty.as_ref() else { return None };
            let syn::PathArguments::AngleBracketed(arguments) =
                &owner.path.segments.last()?.arguments
            else {
                return None;
            };
            let syn::GenericArgument::Type(syn::Type::Path(owner)) = arguments.args.first()? else {
                return None;
            };
            Some(owner.path.segments.last()?.ident.to_string())
        })
        .collect::<BTreeSet<_>>();
    for owner in ["IdentityOrdinary", "IdentityOther", "IdentityPromoted", "IdentityBits"] {
        assert!(owners.contains(owner), "{owner}.value must use the same selected local identity");
    }
    assert!(!full.support.c_source.is_empty(), "bitfields must retain original-C accessors");

    let selected =
        generate_with_bindings(&session, &["IDENTITY_PRIMARY", "IDENTITY_SKIPPED"], &catalog)
            .unwrap();
    let primary = generate_with_bindings(&session, &["IDENTITY_PRIMARY"], &catalog).unwrap();
    // CLI selection prepares only selected roots; library selection can reuse a
    // broader session. Both must resolve the same compiler-owned field identity.
    let isolated_session =
        AnalysisSession::prepare(&scanner, &frontend, &["IDENTITY_PRIMARY"]).unwrap();
    let isolated =
        generate_with_bindings(&isolated_session, &["IDENTITY_PRIMARY"], &catalog).unwrap();
    let profile = frontend.profile();
    let c_arguments = profile.arguments.iter().map(String::as_str).collect::<Vec<_>>();
    let primary_expected = oracle::run_c(
        &profile.compiler.executable,
        &header,
        "#include <stdio.h>\nint main(void) { IdentityOrdinary record = {23, 0, 0}; printf(\"%u\\n\", IDENTITY_PRIMARY(&record)); }",
        &c_arguments,
        true,
    );
    for generation in [&selected, &primary, &isolated] {
        let selected_routes = field_routes(&generation.support.rust);
        assert_eq!(selected_routes["value"], routes["value"]);
        assert!(!selected_routes.contains_key("aa_discarded"));
    }
    for generation in [&selected, &isolated] {
        let actual = rust_oracle::run_rust(&format!(
            r#"#[path={support:?}] pub mod __pgrx_c_macros;
{native} {} {}
fn main() {{
    let mut record = core::mem::MaybeUninit::<IdentityOrdinary>::zeroed();
    // SAFETY: The live aligned record allocation has initialized unsigned fields.
    unsafe {{ (*record.as_mut_ptr()).value = 23; println!("{{}}", IDENTITY_PRIMARY!(record.as_mut_ptr()).get()); }}
}}"#,
            generation.support.rust,
            definitions(generation)
        ));
        assert_eq!(
            actual, primary_expected,
            "support rebuilt after rejection must match its retained macro"
        );
    }

    let runtime_source = format!(
        "#![allow(non_snake_case, non_camel_case_types, dead_code, unused_parens)]\n#![deny(unsafe_op_in_unsafe_fn)]\n#[path={support:?}] pub mod __pgrx_c_macros;\n{native}\n{}\n{macros}",
        full.support.rust
    );
    let directory = TemporaryDirectory::new();
    let runtime = directory.0.join("runtime.rs");
    let library = directory.0.join("libpgrx_field_identity.rlib");
    fs::write(&runtime, &runtime_source).unwrap();
    rust_oracle::run_tool(
        Command::new("rustc")
            .args(["--edition=2024", "--crate-type=rlib", "--crate-name", "pgrx_field_identity"])
            .arg(&runtime)
            .arg("-o")
            .arg(&library),
        "field_identity_library",
    );
    let object = directory.0.join("native.o");
    let native_c = directory.0.join("native.c");
    fs::write(&native_c, &full.support.c_source).unwrap();
    rust_oracle::run_tool(
        Command::new(&profile.compiler.executable)
            .args(&profile.arguments)
            .args(["-x", "c", "-include"])
            .arg(&header)
            .arg("-c")
            .arg(&native_c)
            .arg("-o")
            .arg(&object),
        "field_identity_native",
    );
    let consumer = directory.0.join("consumer.rs");
    fs::write(
        &consumer,
        format!(
            "extern crate pgrx_field_identity as renamed;\nuse renamed::*;\n{RUST_OBSERVATIONS}"
        ),
    )
    .unwrap();
    let executable = directory.0.join("consumer");
    rust_oracle::run_tool(
        Command::new("rustc")
            .args(["--edition=2024", "--extern"])
            .arg(format!("pgrx_field_identity={}", library.display()))
            .arg("-C")
            .arg(format!("link-arg={}", object.display()))
            .arg(&consumer)
            .arg("-o")
            .arg(&executable),
        "field_identity_consumer",
    );
    let actual = rust_oracle::run_tool(&mut Command::new(&executable), "field_identity_execute");
    let expected =
        oracle::run_c(&profile.compiler.executable, &header, C_OBSERVATIONS, &c_arguments, true);
    assert_eq!(
        actual, expected,
        "ordinary, promoted, keyword, bitfield and offset routes must match C"
    );

    for (owner, member, readonly_base) in [
        ("IdentityOrdinary", "frozen", false),
        ("IdentityOrdinary", "value", true),
        ("IdentityPromoted", "frozen", false),
        ("IdentityBits", "frozen", false),
    ] {
        let pointer = if readonly_base { "null" } else { "null_mut" };
        let diagnostic = rust_oracle::reject_rust(&format!(
            "{runtime_source}\nfn main() {{ unsafe {{ IDENTITY_WRITE!(core::ptr::{pointer}::<{owner}>(), {member}, 1u32); }} }}"
        ));
        assert!(
            diagnostic.contains("E0277")
                && diagnostic.contains("ReadOnly")
                && (diagnostic.contains("store") || diagnostic.contains("WritePlace")),
            "{owner}.{member}: {diagnostic}"
        );
        let c_pointer =
            if readonly_base { format!("(const {owner} *)0") } else { format!("({owner} *)0") };
        let diagnostic = rust_oracle::reject_c_invocation(
            &profile.compiler.executable,
            &header,
            &format!("void rejected(void) {{ IDENTITY_WRITE({c_pointer}, {member}, 1); }}"),
            &c_arguments,
        );
        assert!(
            diagnostic.contains("read-only") || diagnostic.contains("const-qualified"),
            "{owner}.{member} must reject C write qualification: {diagnostic}"
        );
    }

    verify_foreign_runtime_field_impls(&directory, &library, &native, &full.support.rust, &macros);
}

// This separate consumer checks local trait-argument orphan legality. It does
// not attempt third-party NativeRecord registration: that private registration
// remains in the complete defining library tested above. Every generated field,
// bitfield and offset implementation and eager layout/type witness is retained.
fn verify_foreign_runtime_field_impls(
    directory: &TemporaryDirectory,
    library: &Path,
    native: &str,
    support: &str,
    macros: &str,
) {
    let mut file = syn::parse_file(support).unwrap();
    let mut registrations = 0;
    for item in &mut file.items {
        if let syn::Item::Mod(module) = item
            && module.ident == "__pgrx_c_generated"
        {
            module.content.as_mut().unwrap().1.retain(|item| {
                let registration = matches!(item, syn::Item::Impl(item) if item.trait_.as_ref().is_some_and(|(_, path, _)| path.segments.last().is_some_and(|segment| segment.ident == "NativeRecord")));
                registrations += usize::from(registration);
                !registration
            });
        }
    }
    assert!(registrations > 0, "the complete defining library must register its native records");
    let source = directory.0.join("foreign_runtime.rs");
    fs::write(
        &source,
        format!(
            r#"#![allow(non_camel_case_types, non_snake_case, dead_code)]
extern crate pgrx_field_identity as upstream;
pub use upstream::__pgrx_c_macros;
{native}
{} {macros}
use __pgrx_c_macros::expression::{{Pointer, CRecord}};
/// # Safety
/// The pointer must designate a live aligned record whose value field is
/// initialized and readable, with allocation bounds and aliasing permissions.
unsafe fn caller(pointer: *mut IdentityOrdinary) {{
    let tagged = Pointer::<CRecord<IdentityOrdinary>>::new(pointer);
    // SAFETY: The caller establishes readable initialized field storage.
    unsafe {{ let _ = IDENTITY_READ!(tagged, value); }}
    let _ = __pgrx_c_macros::expression::offset_of::<
        CRecord<IdentityOrdinary>, __pgrx_c_field_marker!(@path; frozen)
    >();
}}
fn main() {{}}
"#,
            quote::quote!(#file)
        ),
    )
    .unwrap();
    rust_oracle::run_tool(
        Command::new("rustc")
            .args(["--edition=2024", "--emit=metadata", "--extern"])
            .arg(format!("pgrx_field_identity={}", library.display()))
            .arg(&source)
            .arg("-o")
            .arg(directory.0.join("foreign_runtime.rmeta")),
        "field_identity_foreign_runtime",
    );
}

const RUST_OBSERVATIONS: &str = r#"
fn main() {
    let mut ordinary = core::mem::MaybeUninit::<IdentityOrdinary>::zeroed();
    let mut other = core::mem::MaybeUninit::<IdentityOther>::zeroed();
    let mut promoted = core::mem::MaybeUninit::<IdentityPromoted>::zeroed();
    let mut bits = core::mem::MaybeUninit::<IdentityBits>::zeroed();
    let mut keyword = core::mem::MaybeUninit::<IdentityKeyword>::zeroed();
    // SAFETY: Each zeroed allocation is live, aligned and exclusively accessible;
    // all accessed unsigned fields and bitfields are initialized. C accessors
    // handle bitfield storage without Rust reads of the complete record.
    unsafe {
        IDENTITY_WRITE!(ordinary.as_mut_ptr(), value, 3u32);
        IDENTITY_WRITE!(ordinary.as_mut_ptr(), type, 11u32);
        IDENTITY_WRITE!(other.as_mut_ptr(), value, 5u32);
        IDENTITY_WRITE!(promoted.as_mut_ptr(), value, 13u32);
        IDENTITY_WRITE!(keyword.as_mut_ptr(), r#type, 17u32);
        let assigned = IDENTITY_BITS_WRITE!(bits.as_mut_ptr(), 63u32).get();
        println!("{} {} {} {} {} {} {} {} {} {} {}", IDENTITY_PRIMARY!(ordinary.as_mut_ptr()).get(), IDENTITY_OTHER!(other.as_mut_ptr()).get(), IDENTITY_PROMOTED!(promoted.as_mut_ptr()).get(), IDENTITY_READ!(promoted.as_mut_ptr(), value).get(), IDENTITY_READ!(ordinary.as_mut_ptr(), frozen).get(), IDENTITY_READ!(ordinary.as_mut_ptr(), r#type).get(), IDENTITY_KEYWORD!(keyword.as_mut_ptr()).get(), assigned, IDENTITY_READ!(bits.as_mut_ptr(), value).get(), IDENTITY_READ!(bits.as_mut_ptr(), frozen).get(), IDENTITY_BITS_READ!(bits.as_mut_ptr()).get());
    }
    println!("{} {} {} {} {}", IDENTITY_OFFSET!(IdentityOrdinary, value).get(), IDENTITY_OFFSET!(IdentityOrdinary, frozen).get(), IDENTITY_OFFSET!(IdentityOrdinary, type).get(), IDENTITY_OFFSET!(IdentityOther, value).get(), IDENTITY_OFFSET!(IdentityPromoted, frozen).get());
}
"#;

const C_OBSERVATIONS: &str = r#"
#include <stdio.h>
int main(void) {
    IdentityOrdinary ordinary = {0}; IdentityOther other = {0};
    IdentityPromoted promoted = {0}; IdentityBits bits = {0}; IdentityKeyword keyword = {0};
    IDENTITY_WRITE(&ordinary, value, 3); IDENTITY_WRITE(&ordinary, type, 11);
    IDENTITY_WRITE(&other, value, 5); IDENTITY_WRITE(&promoted, value, 13);
    IDENTITY_WRITE(&keyword, type, 17);
    unsigned int assigned = IDENTITY_BITS_WRITE(&bits, 63);
    printf("%u %u %u %u %u %u %u %u %u %u %u\n", (unsigned int)IDENTITY_PRIMARY(&ordinary), (unsigned int)IDENTITY_OTHER(&other), (unsigned int)IDENTITY_PROMOTED(&promoted), (unsigned int)IDENTITY_READ(&promoted,value), (unsigned int)IDENTITY_READ(&ordinary,frozen), (unsigned int)IDENTITY_READ(&ordinary,type), (unsigned int)IDENTITY_KEYWORD(&keyword), assigned, (unsigned int)IDENTITY_READ(&bits,value), (unsigned int)IDENTITY_READ(&bits,frozen), (unsigned int)IDENTITY_BITS_READ(&bits));
    printf("%zu %zu %zu %zu %zu\n", IDENTITY_OFFSET(IdentityOrdinary,value), IDENTITY_OFFSET(IdentityOrdinary,frozen), IDENTITY_OFFSET(IdentityOrdinary,type), IDENTITY_OFFSET(IdentityOther,value), IDENTITY_OFFSET(IdentityPromoted,frozen));
}
"#;

struct TemporaryDirectory(PathBuf);
impl TemporaryDirectory {
    fn new() -> Self {
        let nonce = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();
        let path = std::env::temp_dir()
            .join(format!("pgrx-field-identity-{}-{nonce}", std::process::id()));
        fs::create_dir(&path).unwrap();
        Self(path)
    }
}
impl Drop for TemporaryDirectory {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}
