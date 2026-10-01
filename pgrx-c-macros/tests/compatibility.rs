//LICENSE Portions Copyright 2026 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.

#[path = "support/oracle.rs"]
mod oracle;
#[path = "support/rust_oracle.rs"]
mod rust_oracle;

use pgrx_c_macros::{AnalysisSession, EmissionStatus, MacroScanner, emit, inspect};
use std::fmt::Write;
use std::path::PathBuf;

struct Kind {
    c: &'static str,
    marker: &'static str,
    repr: &'static str,
    cast_macro: &'static str,
    bits: u32,
    signed: bool,
}

const KINDS: &[Kind] = &[
    Kind { c: "_Bool", marker: "CBool", repr: "bool", cast_macro: "BOOL", bits: 1, signed: false },
    Kind { c: "char", marker: "CChar", repr: "i8", cast_macro: "CHAR", bits: 8, signed: true },
    Kind {
        c: "signed char",
        marker: "CSignedChar",
        repr: "i8",
        cast_macro: "SCHAR",
        bits: 8,
        signed: true,
    },
    Kind {
        c: "unsigned char",
        marker: "CUnsignedChar",
        repr: "u8",
        cast_macro: "UCHAR",
        bits: 8,
        signed: false,
    },
    Kind { c: "short", marker: "CShort", repr: "i16", cast_macro: "SHORT", bits: 16, signed: true },
    Kind {
        c: "unsigned short",
        marker: "CUnsignedShort",
        repr: "u16",
        cast_macro: "USHORT",
        bits: 16,
        signed: false,
    },
    Kind { c: "int", marker: "CInt", repr: "i32", cast_macro: "INT", bits: 32, signed: true },
    Kind {
        c: "unsigned int",
        marker: "CUnsignedInt",
        repr: "u32",
        cast_macro: "UINT",
        bits: 32,
        signed: false,
    },
    Kind { c: "long", marker: "CLong", repr: "i64", cast_macro: "LONG", bits: 64, signed: true },
    Kind {
        c: "unsigned long",
        marker: "CUnsignedLong",
        repr: "u64",
        cast_macro: "ULONG",
        bits: 64,
        signed: false,
    },
    Kind {
        c: "long long",
        marker: "CLongLong",
        repr: "i64",
        cast_macro: "LLONG",
        bits: 64,
        signed: true,
    },
    Kind {
        c: "unsigned long long",
        marker: "CUnsignedLongLong",
        repr: "u64",
        cast_macro: "ULLONG",
        bits: 64,
        signed: false,
    },
    Kind {
        c: "__int128",
        marker: "CInt128",
        repr: "i128",
        cast_macro: "I128",
        bits: 128,
        signed: true,
    },
    Kind {
        c: "unsigned __int128",
        marker: "CUnsignedInt128",
        repr: "u128",
        cast_macro: "U128",
        bits: 128,
        signed: false,
    },
];

const BINARY: &[&str] = &[
    "ADD", "SUB", "MUL", "DIV", "REM", "AND_BITS", "OR_BITS", "XOR_BITS", "EQ", "NE", "LT", "LE",
    "GT", "GE",
];

const OTHER: &[&str] = &[
    "ID", "NOT_BITS", "NEG", "PLUS", "NOT", "SHL", "SHR", "AND", "OR", "CHOOSE", "REPEAT", "UNUSED",
];

#[derive(Clone, Copy)]
enum Sample {
    Zero,
    One,
    NegativeOne,
    Minimum,
    Maximum,
}

impl Kind {
    fn samples(&self) -> &'static [Sample] {
        if self.repr == "bool" {
            &[Sample::Zero, Sample::One]
        } else if self.signed {
            &[Sample::Zero, Sample::One, Sample::NegativeOne, Sample::Minimum, Sample::Maximum]
        } else {
            &[Sample::Zero, Sample::One, Sample::Maximum]
        }
    }

    fn c_value(&self, sample: Sample) -> String {
        let value = match sample {
            Sample::Zero => "0".to_owned(),
            Sample::One => "1".to_owned(),
            Sample::NegativeOne => "-1".to_owned(),
            Sample::Minimum if self.signed => {
                format!("-(__int128)((((unsigned __int128)1 << {}) - 1)) - 1", self.bits - 1)
            }
            Sample::Minimum => "0".to_owned(),
            Sample::Maximum if self.repr == "bool" => "1".to_owned(),
            Sample::Maximum if self.signed => {
                format!("((unsigned __int128)1 << {}) - 1", self.bits - 1)
            }
            Sample::Maximum => format!("~(unsigned __int128)0 >> {}", 128 - self.bits),
        };
        format!("({})({value})", self.c)
    }

    fn rust_value(&self, sample: Sample) -> String {
        let value = if self.repr == "bool" {
            match sample {
                Sample::Zero | Sample::Minimum => "false".to_owned(),
                _ => "true".to_owned(),
            }
        } else {
            match sample {
                Sample::Zero => format!("0_{}", self.repr),
                Sample::One => format!("1_{}", self.repr),
                Sample::NegativeOne => format!("-1_{}", self.repr),
                Sample::Minimum if self.signed => format!("{}::MIN", self.repr),
                Sample::Minimum => format!("0_{}", self.repr),
                Sample::Maximum => format!("{}::MAX", self.repr),
            }
        };
        format!("CValue::<{}>::new({value})", self.marker)
    }

    fn left_sample(&self) -> Sample {
        if self.signed { Sample::NegativeOne } else { Sample::Maximum }
    }
}

struct Sources {
    c: String,
    rust: String,
    rows: usize,
}

impl Sources {
    fn new(generated: String) -> Self {
        let mut c = String::from("#define KIND(value) _Generic((value), ");
        let mut rust = generated;
        rust.push_str(
            "use __pgrx_c_macros::*;\ntrait KindId { const ID: usize; }\n\
             fn record<K: CInteger + KindId>(label: &str, value: CValue<K>) {\n\
             println!(\"{} {} {:032x}\", label, K::ID, K::encode(value.get()));\n}\n",
        );
        for (index, kind) in KINDS.iter().enumerate() {
            if index != 0 {
                c.push_str(", ");
            }
            write!(c, "{}: {index}", kind.c).unwrap();
            writeln!(rust, "impl KindId for {} {{ const ID: usize = {index}; }}", kind.marker)
                .unwrap();
        }
        c.push_str(
            ")\n\
             static void record(const char *label, int kind, unsigned __int128 bits) {\n\
             printf(\"%s %d %016llx%016llx\\n\", label, kind,\n\
             (unsigned long long)(bits >> 64), (unsigned long long)bits);\n}\n\
             #define OUT(label, expr) do { __typeof__(expr) value = (expr); \
             record(label, KIND(value), (unsigned __int128)value); } while (0)\n\
             static int calls;\nstatic int bump(void) { calls++; return 7; }\n\
             int main(void) {\n",
        );
        rust.push_str("fn bump(calls: &mut i32) -> i32 { *calls += 1; 7 }\nfn main() {\n");
        Self { c, rust, rows: 0 }
    }

    fn record(&mut self, label: &str, c: &str, rust: &str) {
        writeln!(self.c, "OUT({label:?}, {c});").unwrap();
        writeln!(self.rust, "record({label:?}, {rust});").unwrap();
        self.rows += 1;
    }

    fn finish(mut self) -> Self {
        self.c.push_str("return 0;\n}\n");
        self.rust.push_str("}\n");
        self
    }
}

fn append_type_and_cast_matrix(sources: &mut Sources) {
    for (from, kind) in KINDS.iter().enumerate() {
        for (sample, value) in kind.samples().iter().copied().enumerate() {
            let c = kind.c_value(value);
            let rust = kind.rust_value(value);
            sources.record(
                &format!("identity-{from}-{sample}"),
                &format!("COMP_ID({c})"),
                &format!("COMP_ID!({rust})"),
            );
            for (to, destination) in KINDS.iter().enumerate() {
                let name = destination.cast_macro;
                sources.record(
                    &format!("cast-{from}-{to}-{sample}"),
                    &format!("COMP_CAST_{name}({c})"),
                    &format!("COMP_CAST_{name}!({rust})"),
                );
            }
        }
        for (to, right) in KINDS.iter().enumerate() {
            let c_left = kind.c_value(kind.left_sample());
            let rust_left = kind.rust_value(kind.left_sample());
            let c_right = right.c_value(Sample::One);
            let rust_right = right.rust_value(Sample::One);
            for operator in BINARY {
                sources.record(
                    &format!("pair-{from}-{to}-{operator}"),
                    &format!("COMP_{operator}({c_left}, {c_right})"),
                    &format!("COMP_{operator}!({rust_left}, {rust_right})"),
                );
            }
            sources.record(
                &format!("conditional-{from}-{to}"),
                &format!("COMP_CHOOSE(1, {c_left}, {c_right})"),
                &format!("COMP_CHOOSE!(1_i32, {rust_left}, {rust_right})"),
            );
        }
        let c = kind.c_value(kind.left_sample());
        let rust = kind.rust_value(kind.left_sample());
        for operator in ["NOT_BITS", "NEG", "PLUS", "NOT"] {
            sources.record(
                &format!("unary-{from}-{operator}"),
                &format!("COMP_{operator}({c})"),
                &format!("COMP_{operator}!({rust})"),
            );
        }
    }
}

fn append_unsigned_byte_samples(sources: &mut Sources) {
    sources.c.push_str(
        "for (unsigned int i = 0; i < 64; i++) { for (unsigned int j = 0; j < 64; j++) {\n\
         unsigned char a = (unsigned char)(i * 4);\n\
         unsigned char b = (unsigned char)(j * 4 + 3);\n\
         printf(\"u8 %u %u %d %d %d %d %d %d %d %d %d %d %d\\n\", i, j,\n\
         COMP_ADD(a,b), COMP_SUB(a,b), COMP_MUL(a,b), COMP_DIV(a,b), COMP_REM(a,b),\n\
         COMP_AND_BITS(a,b), COMP_OR_BITS(a,b), COMP_XOR_BITS(a,b),\n\
         COMP_EQ(a,b), COMP_LT(a,b), COMP_NOT_BITS(a));\n\
         }}\n",
    );
    sources.rust.push_str(
        "for i in 0..64_u32 { for j in 0..64_u32 {\n\
         let a = (i * 4) as u8; let b = (j * 4 + 3) as u8;\n\
         println!(\"u8 {} {} {} {} {} {} {} {} {} {} {} {} {}\", i, j,\n\
         COMP_ADD!(a,b).get(), COMP_SUB!(a,b).get(), COMP_MUL!(a,b).get(),\n\
         COMP_DIV!(a,b).get(), COMP_REM!(a,b).get(), COMP_AND_BITS!(a,b).get(),\n\
         COMP_OR_BITS!(a,b).get(), COMP_XOR_BITS!(a,b).get(), COMP_EQ!(a,b).get(),\n\
         COMP_LT!(a,b).get(), COMP_NOT_BITS!(a).get());\n\
         }}\n",
    );
    sources.rows += 4096;
}

fn append_evaluation_cases(sources: &mut Sources) {
    for (label, c, rust) in [
        ("repeat", "COMP_REPEAT(bump())", "COMP_REPEAT!(bump(&mut calls))"),
        ("unused", "COMP_UNUSED(bump())", "COMP_UNUSED!(bump(&mut calls))"),
        ("and-lazy", "COMP_AND(0, bump())", "COMP_AND!(0_i32, bump(&mut calls))"),
        ("and-right", "COMP_AND(1, bump())", "COMP_AND!(1_i32, bump(&mut calls))"),
        ("or-lazy", "COMP_OR(1, bump())", "COMP_OR!(1_i32, bump(&mut calls))"),
        ("or-right", "COMP_OR(0, bump())", "COMP_OR!(0_i32, bump(&mut calls))"),
        (
            "choose-left",
            "COMP_CHOOSE(1, bump(), bump())",
            "COMP_CHOOSE!(1_i32, bump(&mut calls), bump(&mut calls))",
        ),
        (
            "choose-right",
            "COMP_CHOOSE(0, bump(), bump())",
            "COMP_CHOOSE!(0_i32, bump(&mut calls), bump(&mut calls))",
        ),
        (
            "condition-once",
            "COMP_CHOOSE(bump(), bump(), bump())",
            "COMP_CHOOSE!(bump(&mut calls), bump(&mut calls), bump(&mut calls))",
        ),
    ] {
        sources.c.push_str("calls = 0;\n");
        sources.rust.push_str("let mut calls = 0_i32;\n");
        sources.record(&format!("evaluation-{label}"), c, rust);
        sources.record(&format!("count-{label}"), "COMP_ID(calls)", "COMP_ID!(calls)");
    }
    // The unselected arm must never execute a division outside the defined C domain.
    sources.record(
        "choose-invalid-unselected",
        "COMP_CHOOSE(1, 7, COMP_DIV(1, calls = 0))",
        "COMP_CHOOSE!(1_i32, 7_i32, COMP_DIV!(1_i32, { calls = 0; calls }))",
    );
}

fn append_shift_and_wrap_cases(sources: &mut Sources, wrapping: bool) {
    for (index, kind) in KINDS.iter().enumerate() {
        let left = kind.c_value(Sample::One);
        let rust_left = kind.rust_value(Sample::One);
        // Narrow operands promote to int; keep both profiles inside C11's signed domain.
        let result_bits = kind.bits.max(32);
        let high_count =
            if kind.signed || kind.bits < 32 { result_bits - 2 } else { result_bits - 1 };
        for count in [0, 1, high_count] {
            sources.record(
                &format!("shift-left-{index}-{count}"),
                &format!("COMP_SHL({left}, {count})"),
                &format!("COMP_SHL!({rust_left}, {count}_i32)"),
            );
            let c = kind.c_value(kind.left_sample());
            let rust = kind.rust_value(kind.left_sample());
            sources.record(
                &format!("shift-right-{index}-{count}"),
                &format!("COMP_SHR({c}, {count})"),
                &format!("COMP_SHR!({rust}, {count}_i32)"),
            );
        }
        if wrapping && kind.signed && kind.bits >= 32 {
            let c_max = kind.c_value(Sample::Maximum);
            let rust_max = kind.rust_value(Sample::Maximum);
            let c_min = kind.c_value(Sample::Minimum);
            let rust_min = kind.rust_value(Sample::Minimum);
            for (label, c, rust) in [
                ("add", format!("COMP_ADD({c_max}, 1)"), format!("COMP_ADD!({rust_max}, 1_i32)")),
                ("sub", format!("COMP_SUB({c_min}, 1)"), format!("COMP_SUB!({rust_min}, 1_i32)")),
                ("mul", format!("COMP_MUL({c_max}, 2)"), format!("COMP_MUL!({rust_max}, 2_i32)")),
                ("neg", format!("COMP_NEG({c_min})"), format!("COMP_NEG!({rust_min})")),
            ] {
                sources.record(&format!("wrap-{index}-{label}"), &c, &rust);
            }
            // LLVM 21 Clang's EmitShl disables signed-base UB under -fwrapv,
            // emits a plain wrapping shl, and retains the shift-count domain.
            for (label, c, rust) in [
                (
                    "sign-bit",
                    format!("COMP_SHL({left}, {})", kind.bits - 1),
                    format!("COMP_SHL!({rust_left}, {}_i32)", kind.bits - 1),
                ),
                (
                    "negative",
                    format!("COMP_SHL({}, 1)", kind.c_value(Sample::NegativeOne)),
                    format!("COMP_SHL!({}, 1_i32)", kind.rust_value(Sample::NegativeOne)),
                ),
                (
                    "overflow",
                    format!("COMP_SHL({c_max}, 1)"),
                    format!("COMP_SHL!({rust_max}, 1_i32)"),
                ),
            ] {
                sources.record(&format!("wrap-shift-{index}-{label}"), &c, &rust);
            }
        }
    }
    // Clang's signed-right-shift implementation choice is tested with a negative value.
    sources.record("negative-div", "COMP_DIV(-7, 3)", "COMP_DIV!(-7_i32, 3_i32)");
    sources.record("negative-rem", "COMP_REM(-7, 3)", "COMP_REM!(-7_i32, 3_i32)");
}

#[cfg(target_os = "macos")]
fn native_include_arguments() -> Vec<String> {
    let sdk = rust_oracle::run_tool(
        std::process::Command::new("xcrun").arg("--show-sdk-path"),
        "Apple SDK lookup",
    );
    let sdk = sdk.trim();
    assert!(
        !sdk.is_empty() && std::path::Path::new(sdk).is_dir(),
        "Apple SDK lookup did not return an installed SDK directory"
    );
    vec!["-isysroot".to_owned(), sdk.to_owned()]
}

#[cfg(not(target_os = "macos"))]
fn native_include_arguments() -> Vec<String> {
    Vec::new()
}

#[test]
fn original_c_and_emitted_rust_match_integer_types_values_and_evaluation() {
    let scanner = MacroScanner::new().expect("libclang must be available");
    let header = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/compatibility.h");
    let runtime = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../pgrx-pg-sys/src/c_macros/support.rs")
        .canonicalize()
        .unwrap();
    let mut names =
        BINARY.iter().chain(OTHER).map(|name| format!("COMP_{name}")).collect::<Vec<_>>();
    names.extend(KINDS.iter().map(|kind| format!("COMP_CAST_{}", kind.cast_macro)));
    let references = names.iter().map(String::as_str).collect::<Vec<_>>();
    let includes = native_include_arguments();
    for wrapping in [false, true] {
        let mut arguments = vec![
            "-std=gnu11".to_owned(),
            "-O2".to_owned(),
            if wrapping { "-fwrapv" } else { "-fno-wrapv" }.to_owned(),
        ];
        arguments.extend(includes.iter().cloned());
        let frontend =
            inspect(&scanner, &header, &arguments, None).expect("inspect original C corpus");
        let session = AnalysisSession::prepare(&scanner, &frontend, &references).unwrap();
        let mut generated =
            format!("#[path = {:?}]\npub mod __pgrx_c_macros;\n", runtime.to_str().unwrap());
        for name in &names {
            let emission = emit(&session, name);
            let EmissionStatus::Emitted { rust, .. } = &emission.status else {
                panic!("corpus macro must emit: {emission:?}");
            };
            generated.push_str(rust);
        }
        let mut sources = Sources::new(generated);
        append_type_and_cast_matrix(&mut sources);
        append_unsigned_byte_samples(&mut sources);
        append_evaluation_cases(&mut sources);
        append_shift_and_wrap_cases(&mut sources, wrapping);
        let sources = sources.finish();
        let c = oracle::run_c(
            &frontend.profile().compiler.executable,
            &header,
            &sources.c,
            &frontend.profile().arguments.iter().map(String::as_str).collect::<Vec<_>>(),
            true,
        );
        let rust = rust_oracle::run_rust(&sources.rust);
        assert_eq!(c.lines().count(), sources.rows);
        assert_eq!(rust.lines().count(), sources.rows);
        for (index, (c, rust)) in c.lines().zip(rust.lines()).enumerate() {
            assert_eq!(rust, c, "original C vs emitted Rust row {index}, wrapping={wrapping}");
        }
    }
}
