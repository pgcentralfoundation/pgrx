//LICENSE Portions Copyright 2026 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.

//! Bridge immutable PostgreSQL declaration metadata through its original C macros.
//!
//! C owns initializer values and declaration expansion. Rust only receives records after
//! every field's storage and layout agree with the current Clang and bindgen catalogues.
//! These getters access immutable static data; they cannot raise PostgreSQL errors, invoke
//! callbacks, or access backend state, so they deliberately have no backend FFI guard.

use eyre::{WrapErr, eyre};
use pgrx_c_macros::{
    ActiveProvenance, ArrayKind, BindingCatalog, DeclarationCatalog, FrontendOutput, IntegerKind,
    MacroKind, RecordBinding, RecordKind, RustBindingType, TargetFacts, TypeCategory, TypeInfo,
    TypeShapeKind,
};
use std::collections::BTreeSet;
use std::fmt::Write;
use std::hash::{DefaultHasher, Hash, Hasher};

/// Rust data bridges and original-header declarations to append to one native translation unit.
pub(super) struct MetadataSupport {
    /// Layout witnesses and safe, hidden Rust accessors using actual bindgen paths.
    pub(super) rust: String,
    /// Original C metadata declarations and pure getters, without another header include.
    pub(super) c_source: String,
}

/// Generate metadata only after proving aggregate validity recursively against both catalogues.
pub(super) fn generate(
    frontend: &FrontendOutput,
    bindings: &BindingCatalog,
) -> eyre::Result<MetadataSupport> {
    let magic = frontend
        .environment()
        .active
        .get("PG_MODULE_MAGIC_DATA")
        .ok_or_else(|| eyre!("PG_MODULE_MAGIC_DATA is absent from the inspected headers"))?;
    let finfo = frontend
        .environment()
        .active
        .get("PG_FUNCTION_INFO_V1")
        .ok_or_else(|| eyre!("PG_FUNCTION_INFO_V1 is absent from the inspected headers"))?;
    if !matches!(magic.provenance, ActiveProvenance::Resolved)
        || !matches!(finfo.provenance, ActiveProvenance::Resolved)
        || finfo.definition.kind != MacroKind::FunctionLike
    {
        return Err(eyre!("metadata macro declarations require resolved original provenance"));
    }
    let magic_initializer = match magic.definition.kind {
        MacroKind::ObjectLike => "PG_MODULE_MAGIC_DATA",
        MacroKind::FunctionLike => "PG_MODULE_MAGIC_DATA()",
    };
    // Namespacing also covers the original finfo getter, whose PGDLLEXPORT explicitly
    // overrides the native translation unit's hidden default visibility. Include original
    // input fingerprints, macro definitions and both storage catalogues so different header
    // versions/configurations do not resolve one another's metadata symbols.
    let mut identity = DefaultHasher::new();
    serde_json::to_string(&(
        frontend.profile(),
        &magic.definition,
        &finfo.definition,
        frontend.declarations(),
        bindings,
    ))?
    .hash(&mut identity);
    let suffix = format!("{:016x}", identity.finish());
    let magic_getter = format!("pgrx_metadata_{suffix}_magic_get");
    let finfo_function = format!("pgrx_metadata_{suffix}_function_v1");
    let finfo_getter = format!("pg_finfo_{finfo_function}");
    let mut proof = StorageProof {
        declarations: frontend.declarations(),
        bindings,
        target: &frontend.profile().target,
        rust: pgrx_c_macros::support_abi_assertions(frontend.profile())
            .map_err(|error| eyre!(error))?,
        c_source: String::new(),
        active_records: BTreeSet::new(),
        witness: 0,
    };
    let magic_type = proof.named_record("Pg_magic_struct")?;
    let finfo_type = proof.named_record("Pg_finfo_record")?;
    writeln!(
        proof.c_source,
        "static const Pg_magic_struct pgrx_metadata_magic = {magic_initializer};"
    )?;
    writeln!(proof.c_source,
        "#if defined(__ELF__) || defined(__MACH__)
__attribute__((visibility(\"hidden\")))
#endif
const Pg_magic_struct *{magic_getter}(void) {{ return &pgrx_metadata_magic; }}
PG_FUNCTION_INFO_V1({finfo_function});
/* Verify the original declaration emitted exactly the symbol and return type Rust imports. */
_Static_assert(__builtin_types_compatible_p(__typeof__(&{finfo_getter}), const Pg_finfo_record *(*)(void)), \"original finfo getter signature changed\");")?;
    writeln!(proof.rust, "
/// Pure native getters for original immutable PostgreSQL declaration metadata.
mod __pgrx_metadata_native {{
    unsafe extern \"C\" {{
        /// Return the native static initialized by PG_MODULE_MAGIC_DATA.
        ///
        /// # Safety
        /// The matching native archive must remain loaded. Its returned static must not be mutated.
        #[link_name = \"{magic_getter}\"]
        pub(super) fn pgrx_metadata_magic_get() -> *const {magic_type};
        /// Return the native static declared by PG_FUNCTION_INFO_V1.
        ///
        /// # Safety
        /// The matching native archive must remain loaded. Its returned static must not be mutated.
        #[link_name = \"{finfo_getter}\"]
        pub(super) fn pg_finfo_pgrx_metadata_function_v1() -> *const {finfo_type};
    }}
}}
/// Copy this installation's original module magic data, including native initializer defaults.
#[doc(hidden)]
pub fn __pgrx_module_magic_data() -> {magic_type} {{
    // SAFETY: the C getter returns an aligned, immutable static initialized by the original
    // macro. Recursive storage witnesses prove every field is Rust-valid and both layouts agree.
    // Padding need not be initialized for a typed record copy. The getter has no backend effects.
    unsafe {{ __pgrx_metadata_native::pgrx_metadata_magic_get().read() }}
}}
/// Borrow the immutable V1 function-info record declared by this installation's original macro.
#[doc(hidden)]
pub fn __pgrx_function_info_v1() -> &'static {finfo_type} {{
    // SAFETY: the original declaration returns its immutable initialized C static, which lives
    // for the linked module's lifetime. Recursive witnesses prove layout and every field's validity.
    // No Rust or C writer exists for this record, and the getter has no backend effects.
    unsafe {{ &*__pgrx_metadata_native::pg_finfo_pgrx_metadata_function_v1() }}
}}
")?;
    Ok(MetadataSupport { rust: proof.rust, c_source: proof.c_source })
}

/// Accumulate layout assertions while proving only unrestricted initialized storage families.
struct StorageProof<'a> {
    /// Coherent original C declarations used for type and layout identity.
    declarations: &'a DeclarationCatalog,
    /// Fresh bindgen storage facts and accessible paths from the same build.
    bindings: &'a BindingCatalog,
    /// Compiler-established integer and pointer representation facts.
    target: &'a TargetFacts,
    /// Rust assertions and compile-time field type witnesses.
    rust: String,
    /// C assertions that reject changed headers at native compilation.
    c_source: String,
    /// Records on the current recursion path, rejecting cyclic by-value storage.
    active_records: BTreeSet<String>,
    /// Unique private field-witness function suffix within this generated fragment.
    witness: usize,
}

impl StorageProof<'_> {
    /// Resolve a requested C typedef through its actual binding rather than guessing a Rust path.
    fn named_record(&mut self, name: &str) -> eyre::Result<String> {
        let ty = self
            .declarations
            .types
            .get(name)
            .cloned()
            .ok_or_else(|| eyre!("metadata C type {name} is unavailable"))?;
        let binding = if let Some(record) = self.bindings.records.get(name) {
            RustBindingType::Named { path: record.path.clone() }
        } else if let Some(alias) = self.bindings.type_alias(name) {
            alias.target.clone()
        } else {
            return Err(eyre!("metadata Rust type {name} is unavailable"));
        };
        self.storage(&ty, &binding, name, 0)
            .wrap_err_with(|| format!("invalid metadata storage for {name}"))
    }

    /// Normalize only catalogue-owned aliases, with a bound that rejects unresolved or cyclic types.
    fn normalize(&self, binding: &RustBindingType, depth: usize) -> eyre::Result<RustBindingType> {
        if depth > 64 {
            return Err(eyre!("metadata alias recursion exceeds its bound"));
        }
        if let RustBindingType::Named { path } = binding
            && let Some(alias) = self.bindings.types.values().find(|alias| alias.path == *path)
        {
            return self.normalize(&alias.target, depth + 1);
        }
        Ok(binding.clone())
    }

    /// Prove unrestricted integers, fixed arrays, plain structs and raw scalar pointers recursively.
    /// Restricted Rust validity families, including enums, bools, unions and callbacks, fail closed.
    fn storage(
        &mut self,
        ty: &TypeInfo,
        binding: &RustBindingType,
        c_type: &str,
        depth: usize,
    ) -> eyre::Result<String> {
        if depth > 64 || ty.is_volatile {
            return Err(eyre!("metadata storage is recursive or volatile: {}", ty.spelling));
        }
        let actual = self.normalize(binding, depth)?;
        let shape = self.declarations.type_shapes.get(&ty.canonical_spelling);
        let rust_type = match (&ty.category, shape.map(|shape| &shape.kind), &actual) {
            (TypeCategory::Integer(IntegerKind::Char), _, RustBindingType::NativeChar) => {
                if self.target.char_bits != 8 || ty.size != Some(1) || ty.alignment != Some(1) {
                    return Err(eyre!(
                        "metadata c_char storage requires a verified plain-char byte: {}",
                        ty.spelling
                    ));
                }
                // Rust c_char is an i8/u8 alias: every initialized byte pattern is valid for
                // either storage type, even if a C char flag changes the inspected signedness.
                // This bridge copies immutable objects and preserves their bits; it performs
                // no char arithmetic or by-value char calls. Both languages' layout assertions
                // and the field type witness below still verify the actual binding storage.
                "::core::ffi::c_char".into()
            }
            (TypeCategory::Integer(kind), _, RustBindingType::Integer { signed, bits })
                if *kind != IntegerKind::Bool =>
            {
                let native =
                    self.target.integers.get(kind).ok_or_else(|| {
                        eyre!("metadata integer facts are absent: {}", ty.spelling)
                    })?;
                if native.bits != *bits
                    || native.signed != *signed
                    || ty.size.and_then(|size| size.checked_mul(u64::from(self.target.char_bits)))
                        != Some(u64::from(*bits))
                {
                    return Err(eyre!(
                        "metadata integer representation disagrees: {}",
                        ty.spelling
                    ));
                }
                format!("{}{bits}", if *signed { "i" } else { "u" })
            }
            (
                _,
                Some(TypeShapeKind::Array {
                    element,
                    length: Some(length),
                    array_kind: ArrayKind::Constant,
                }),
                RustBindingType::Array { element: actual_element, length: actual_length },
            ) if length == actual_length => {
                let element_type = self.storage(
                    element,
                    actual_element,
                    &format!("__typeof__((({c_type} *)0)[0][0])"),
                    depth + 1,
                )?;
                format!("[{element_type}; {length}]")
            }
            (
                TypeCategory::Pointer,
                Some(TypeShapeKind::Pointer { pointee }),
                RustBindingType::Pointer { pointee: actual_pointee, mutable },
            ) => {
                // Raw scalar pointers have no Rust enum/reference/function-pointer validity restriction.
                // Pointee storage is still checked; const qualification may not be discarded.
                if *mutable == pointee.is_const
                    || pointee.is_volatile
                    || !matches!(pointee.category, TypeCategory::Integer(kind) if kind != IntegerKind::Bool)
                {
                    return Err(eyre!(
                        "metadata pointer is not plain scalar storage: {}",
                        ty.spelling
                    ));
                }
                let pointee_type = self.storage(
                    pointee,
                    actual_pointee,
                    &format!("__typeof__(*((({c_type} *)0)[0]))"),
                    depth + 1,
                )?;
                format!("*{} {pointee_type}", if *mutable { "mut" } else { "const" })
            }
            (
                TypeCategory::Record,
                Some(TypeShapeKind::Record { .. }),
                RustBindingType::Named { path },
            ) => {
                let record =
                    self.declarations.records.get(&ty.canonical_spelling).cloned().ok_or_else(
                        || eyre!("metadata record layout is absent: {}", ty.spelling),
                    )?;
                let binding = self
                    .bindings
                    .records
                    .values()
                    .find(|record| record.path == *path)
                    .cloned()
                    .ok_or_else(|| {
                        eyre!("metadata record binding is absent: {}", path.join("::"))
                    })?;
                self.record(ty, &record, &binding, c_type, depth + 1)?
            }
            _ => {
                return Err(eyre!(
                    "metadata storage cannot establish Rust validity: {} as {actual:?}",
                    ty.spelling
                ));
            }
        };
        self.layout(ty, &rust_type, c_type)?;
        Ok(rust_type)
    }

    /// Match every named field exactly and prove each nested field before admitting a record load.
    fn record(
        &mut self,
        ty: &TypeInfo,
        record: &pgrx_c_macros::RecordInfo,
        binding: &RecordBinding,
        c_type: &str,
        depth: usize,
    ) -> eyre::Result<String> {
        if record.kind != RecordKind::Struct
            || binding.kind != RecordKind::Struct
            || binding.packed
            || !binding.copy
            || record.fields.len() != binding.fields.len()
            || record.size != ty.size
            || record.alignment != ty.alignment
            || !self.active_records.insert(record.identity.clone())
        {
            return Err(eyre!(
                "metadata aggregate is not a complete plain Copy struct: {}",
                ty.spelling
            ));
        }
        let rust_type = rust_path(&binding.path)?;
        let witness = self.witness;
        self.witness += 1;
        let mut fields = String::new();
        for field in &record.fields {
            let name = field
                .name
                .as_ref()
                .filter(|_| !field.is_anonymous && field.bit_width.is_none())
                .ok_or_else(|| eyre!("metadata anonymous or bit-field storage is unsupported"))?;
            c_identifier(name)?;
            let actual = binding
                .fields
                .get(name)
                .ok_or_else(|| eyre!("metadata field {name} is missing from Rust storage"))?;
            let rust_name = syn::parse_str::<syn::Ident>(&actual.rust_name)
                .wrap_err("metadata field has no valid Rust identifier")?;
            let offset = field
                .offset_bits
                .filter(|bits| bits % 8 == 0)
                .map(|bits| bits / 8)
                .ok_or_else(|| eyre!("metadata field {name} has no byte offset"))?;
            let field_type = self.storage(
                &field.ty,
                &actual.ty,
                &format!("__typeof__((({c_type} *)0)->{name})"),
                depth + 1,
            )?;
            writeln!(fields, "let _: &{field_type} = &value.{rust_name};")?;
            writeln!(
                self.rust,
                "const _: () = assert!(::core::mem::offset_of!({rust_type}, {rust_name}) == {offset});"
            )?;
            writeln!(
                self.c_source,
                "_Static_assert(__builtin_offsetof({c_type}, {name}) == {offset}, \"metadata field offset changed\");"
            )?;
        }
        writeln!(
            self.rust,
            "/// Compile-time storage witness for every initialized field of this metadata record.\n#[allow(dead_code)]\nfn __pgrx_metadata_fields_{witness}(value: &{rust_type}) {{ {fields} }}"
        )?;
        self.active_records.remove(&record.identity);
        Ok(rust_type)
    }

    /// Check both compilation languages against the inspected size and alignment, including fields.
    fn layout(&mut self, ty: &TypeInfo, rust_type: &str, c_type: &str) -> eyre::Result<()> {
        let size =
            ty.size.ok_or_else(|| eyre!("metadata storage is incomplete: {}", ty.spelling))?;
        let alignment = ty
            .alignment
            .filter(|alignment| *alignment != 0)
            .ok_or_else(|| eyre!("metadata alignment is absent: {}", ty.spelling))?;
        writeln!(
            self.rust,
            "const _: () = {{ assert!(::core::mem::size_of::<{rust_type}>() == {size}); assert!(::core::mem::align_of::<{rust_type}>() == {alignment}); }};"
        )?;
        writeln!(
            self.c_source,
            "_Static_assert(sizeof({c_type}) == {size} && _Alignof({c_type}) == {alignment}, \"metadata storage layout changed\");"
        )?;
        Ok(())
    }
}

/// Render the actual crate-relative binding path only after Rust's parser accepts it.
fn rust_path(path: &[String]) -> eyre::Result<String> {
    let path = format!("crate::{}", path.join("::"));
    syn::parse_str::<syn::Path>(&path).wrap_err("metadata has no usable Rust binding path")?;
    Ok(path)
}

/// Reject C syntax in compiler-owned names before interpolating them into field assertions.
fn c_identifier(name: &str) -> eyre::Result<()> {
    let mut bytes = name.bytes();
    if !bytes.next().is_some_and(|byte| byte.is_ascii_alphabetic() || byte == b'_')
        || !bytes.all(|byte| byte.is_ascii_alphanumeric() || byte == b'_')
    {
        return Err(eyre!("metadata C field name is not an identifier"));
    }
    Ok(())
}

#[cfg(all(test, unix))]
mod tests {
    use super::*;
    use std::collections::BTreeMap;
    use std::path::PathBuf;

    /// Own native fixture sources and artifacts without touching generated PostgreSQL bindings.
    struct Directory(
        /// Unique temporary tree reserved by this fixture.
        PathBuf,
    );

    impl Directory {
        /// Reserve only a new directory, never reusing another test's compiler inputs.
        fn new() -> Self {
            for attempt in 0..64 {
                let path = std::env::temp_dir()
                    .join(format!("pgrx-native-metadata-{}-{attempt}", std::process::id()));
                match std::fs::create_dir(&path) {
                    Ok(()) => return Self(path),
                    Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => continue,
                    Err(error) => panic!("could not reserve metadata fixture: {error}"),
                }
            }
            panic!("could not reserve a unique metadata fixture");
        }
    }

    impl Drop for Directory {
        /// Delete only this test's successfully reserved temporary tree.
        fn drop(&mut self) {
            let _ = std::fs::remove_dir_all(&self.0);
        }
    }

    /// Inspect a header and reconcile independently generated bindgen storage under its C flags.
    fn fixture(
        directory: &Directory,
        function_like_magic: bool,
    ) -> (FrontendOutput, BindingCatalog, String) {
        fixture_with_arguments(directory, function_like_magic, &[])
    }

    /// Keep explicit plain-char modes in both independent inspection and binding invocations.
    fn fixture_with_arguments(
        directory: &Directory,
        function_like_magic: bool,
        arguments: &[String],
    ) -> (FrontendOutput, BindingCatalog, String) {
        let header = directory.0.join("metadata.h");
        let parameters = if function_like_magic { "(...)" } else { "" };
        std::fs::write(&header, format!(
            "typedef struct {{ int version; char abi_extra[5]; }} Pg_abi_values;\n\
             typedef struct {{ int len; Pg_abi_values abi_fields; const char *name; }} Pg_magic_struct;\n\
             typedef struct {{ int api_version; }} Pg_finfo_record;\n\
             #define PG_MODULE_MAGIC_DATA{parameters} {{ sizeof(Pg_magic_struct), {{ 29, \"\\377abi\" }}, 0 }}\n\
             #define PG_FUNCTION_INFO_V1(name) const Pg_finfo_record *pg_finfo_##name(void) {{ static const Pg_finfo_record info = {{ 37 }}; return &info; }}\n"
        )).unwrap();
        let scanner = pgrx_c_macros::MacroScanner::new().unwrap();
        let frontend = pgrx_c_macros::inspect(&scanner, &header, arguments, None).unwrap();
        let rust = bindgen::Builder::default()
            .header(header.to_str().unwrap())
            .clang_args(&frontend.profile().arguments)
            .layout_tests(false)
            .derive_copy(true)
            .generate()
            .unwrap()
            .to_string();
        let bindings = super::super::binding_symbols::collect_bindings(
            &syn::parse_file(&rust).unwrap(),
            &BTreeMap::new(),
            frontend.declarations(),
            &frontend.profile().target,
        );
        (frontend, bindings, rust)
    }

    /// Standalone consumers use the same facade, alignment witnesses and cfg as production.
    fn runtime_fixture(frontend: &FrontendOutput) -> String {
        let support = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../pgrx-pg-sys/src/c_macros/support.rs")
            .canonicalize()
            .unwrap();
        let alignment = pgrx_c_macros::support_alignment_profile(frontend.profile()).unwrap();
        format!("#[path={support:?}] pub mod __pgrx_c_macros;\n{alignment}\n")
    }

    /// Compile and execute the real generated Rust/C bridge. Distinct fixture values prove both
    /// declarations supply their data; these values do not reproduce PostgreSQL's copied formulas.
    /// Both plain-char modes retain a high-bit byte through native-char nested storage.
    #[test]
    fn original_object_and_variadic_macros_supply_immutable_nested_metadata() {
        let _lock =
            super::super::MACRO_SCANNER.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        for (function_like_magic, char_mode) in
            [false, true].into_iter().flat_map(|function_like| {
                ["-fsigned-char", "-funsigned-char"].map(|mode| (function_like, mode))
            })
        {
            let directory = Directory::new();
            let (frontend, bindings, binding_source) =
                fixture_with_arguments(&directory, function_like_magic, &[char_mode.into()]);
            let RustBindingType::Array { element, .. } =
                &bindings.records["Pg_abi_values"].fields["abi_extra"].ty
            else {
                panic!("fixture char array disappeared")
            };
            assert_eq!(**element, RustBindingType::NativeChar);
            let support = generate(&frontend, &bindings).unwrap();
            let c_source = directory.0.join("metadata.c");
            std::fs::write(&c_source, format!("#include \"metadata.h\"\n{}", support.c_source))
                .unwrap();
            let object = directory.0.join("metadata.o");
            let archive = directory.0.join("libmetadata_fixture.a");
            pgrx_c_macros::compile_native_support(frontend.profile(), &c_source, &object, &archive)
                .unwrap();
            let rust_source = directory.0.join("metadata.rs");
            std::fs::write(&rust_source, format!(
                "#![allow(non_camel_case_types,dead_code)]\n{}\n{binding_source}\n{}\n\
                 fn main() {{\n\
                    let magic = __pgrx_module_magic_data();\n\
                    assert_eq!(magic.len as usize, core::mem::size_of::<Pg_magic_struct>());\n\
                    assert_eq!(magic.abi_fields.version, 29);\n\
                    assert_eq!(magic.abi_fields.abi_extra[0] as u8, 255);\n\
                    assert_eq!(magic.abi_fields.abi_extra[1] as u8, b'a');\n\
                    assert!(magic.name.is_null());\n\
                    assert_eq!(__pgrx_function_info_v1().api_version, 37);\n\
                    assert!(core::ptr::eq(__pgrx_function_info_v1(), __pgrx_function_info_v1()));\n\
                    assert_eq!(std::thread::spawn(|| __pgrx_function_info_v1().api_version).join().unwrap(), 37);\n\
                 }}\n", runtime_fixture(&frontend), support.rust
            )).unwrap();
            let executable = directory.0.join("metadata-test");
            let output = std::process::Command::new(
                std::env::var_os("RUSTC").unwrap_or_else(|| "rustc".into()),
            )
            .arg("--edition=2024")
            .args(
                pgrx_c_macros::support_rust_cfg(frontend.profile())
                    .unwrap()
                    .iter()
                    .flat_map(|cfg| ["--cfg", cfg.as_str()]),
            )
            .arg(&rust_source)
            .arg("-L")
            .arg(format!("native={}", directory.0.display()))
            .arg("-lstatic=metadata_fixture")
            .arg("-o")
            .arg(&executable)
            .output()
            .unwrap();
            assert!(output.status.success(), "{}", String::from_utf8_lossy(&output.stderr));
            assert!(std::process::Command::new(&executable).status().unwrap().success());
        }
    }

    /// Reject restricted validity and mismatched nested storage before generating typed loads.
    #[test]
    fn metadata_rejects_boolean_missing_field_and_changed_nested_array_storage() {
        let _lock =
            super::super::MACRO_SCANNER.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let directory = Directory::new();
        let (frontend, bindings, _) = fixture(&directory, false);
        let mut restricted = bindings.clone();
        restricted
            .records
            .get_mut("Pg_finfo_record")
            .unwrap()
            .fields
            .get_mut("api_version")
            .unwrap()
            .ty = RustBindingType::Bool;
        assert!(generate(&frontend, &restricted).is_err());
        let mut false_char = bindings.clone();
        false_char
            .records
            .get_mut("Pg_finfo_record")
            .unwrap()
            .fields
            .get_mut("api_version")
            .unwrap()
            .ty = RustBindingType::NativeChar;
        assert!(generate(&frontend, &false_char).is_err());
        let mut missing = bindings.clone();
        missing.records.get_mut("Pg_magic_struct").unwrap().fields.remove("name");
        assert!(generate(&frontend, &missing).is_err());
        let mut nested = bindings.clone();
        let array = &mut nested
            .records
            .get_mut("Pg_abi_values")
            .unwrap()
            .fields
            .get_mut("abi_extra")
            .unwrap()
            .ty;
        let RustBindingType::Array { length, .. } = array else {
            panic!("fixture array disappeared")
        };
        *length += 1;
        assert!(generate(&frontend, &nested).is_err());
    }

    /// Distinct inspected original definitions get distinct native symbol identities, while the
    /// same coherent catalogues produce matching C symbols and Rust link names on every call.
    #[test]
    fn original_profile_namespaces_metadata_symbols() {
        let _lock =
            super::super::MACRO_SCANNER.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let directory = Directory::new();
        let (object, object_bindings, _) = fixture(&directory, false);
        let object_support = generate(&object, &object_bindings).unwrap();
        let repeated = generate(&object, &object_bindings).unwrap();
        assert_eq!(object_support.rust, repeated.rust);
        assert_eq!(object_support.c_source, repeated.c_source);
        let (variadic, variadic_bindings, _) = fixture(&directory, true);
        let variadic_support = generate(&variadic, &variadic_bindings).unwrap();
        let link_names = |source: &str| {
            source
                .lines()
                .filter(|line| line.contains("#[link_name"))
                .map(str::to_owned)
                .collect::<Vec<_>>()
        };
        let object_names = link_names(&object_support.rust);
        let variadic_names = link_names(&variadic_support.rust);
        assert_eq!(object_names.len(), 2);
        assert_eq!(variadic_names.len(), 2);
        assert!(object_names.iter().all(|name| !variadic_names.contains(name)));
        assert!(object_support.c_source.contains("visibility(\"hidden\")"));
        assert!(object_support.c_source.contains("__builtin_types_compatible_p"));
    }

    /// Retain both independent layout witnesses: stale C fields and Rust aggregate alignment
    /// fail compilation instead of allowing an unchecked aggregate read.
    #[test]
    fn generated_layout_assertions_reject_drift_in_either_language() {
        let _lock =
            super::super::MACRO_SCANNER.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let directory = Directory::new();
        let (frontend, bindings, binding_source) = fixture(&directory, false);
        let support = generate(&frontend, &bindings).unwrap();
        let c_source = directory.0.join("invalid-layout.c");
        // The extra leading C field changes offsets without editing the inspected input header.
        let changed_header = std::fs::read_to_string(frontend.profile().header.clone())
            .unwrap()
            .replace("int api_version;", "int prefix; int api_version;");
        std::fs::write(&c_source, format!("{changed_header}\n{}", support.c_source)).unwrap();
        let object = directory.0.join("invalid-layout.o");
        let archive = directory.0.join("libinvalid_layout.a");
        assert!(
            pgrx_c_macros::compile_native_support(frontend.profile(), &c_source, &object, &archive)
                .is_err()
        );
        assert!(!archive.exists());
        let rust_source = directory.0.join("invalid-layout.rs");
        std::fs::write(
            &rust_source,
            format!(
                "#![allow(non_camel_case_types,dead_code)]\n{}\n{}\n{}\nfn main() {{}}\n",
                runtime_fixture(&frontend),
                binding_source.replace("#[repr(C)]", "#[repr(C, align(64))]"),
                support.rust
            ),
        )
        .unwrap();
        let output =
            std::process::Command::new(std::env::var_os("RUSTC").unwrap_or_else(|| "rustc".into()))
                .arg("--edition=2024")
                .args(
                    pgrx_c_macros::support_rust_cfg(frontend.profile())
                        .unwrap()
                        .iter()
                        .flat_map(|cfg| ["--cfg", cfg.as_str()]),
                )
                .arg("--emit=metadata")
                .arg("--crate-name=invalid_layout")
                .arg(&rust_source)
                .arg("-o")
                .arg(directory.0.join("invalid-layout.rmeta"))
                .output()
                .unwrap();
        assert!(!output.status.success(), "Rust aggregate layout drift was accepted");
        assert!(
            String::from_utf8_lossy(&output.stderr).contains("evaluation"),
            "expected layout assertion failure: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }
}
