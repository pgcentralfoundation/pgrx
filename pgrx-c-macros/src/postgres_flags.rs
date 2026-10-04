//LICENSE Portions Copyright 2026 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.

//! Decode recorded PostgreSQL build flags for the GNU-style Clang driver.
//!
//! MSVC fragments use Windows C argument quoting, preserving ordinary path
//! backslashes. Only explicitly understood options are translated; response
//! files and unknown options cannot hide additional semantic settings.

use crate::FrontendError;

/// Decode one recorded CFLAGS or CPPFLAGS fragment without changing its option order.
///
/// GNU fragments retain shell quoting through `shlex`. MSVC fragments follow
/// [Windows C argument quoting](https://learn.microsoft.com/en-us/cpp/c-language/parsing-c-command-line-arguments)
/// and map a bounded set of options to the GNU-style Clang driver. Translation
/// does not establish profile admission; compiler witnesses and the target gate
/// must still establish the actual declarations, layouts, and semantic modes.
/// Runtime selections remain intermediate `-fms-runtime-lib=...` markers. Call
/// [`lower_msvc_runtime_flags`] once after combining CFLAGS, CPPFLAGS and explicit
/// arguments: the CL driver selects the last runtime mode across the whole argv.
///
/// PostgreSQL's `src/common/config_info.c` returns `not recorded` when an MSVC
/// build does not provide VAL_CFLAGS or VAL_CPPFLAGS. That exact metadata value
/// contributes no flags. The selected profile then describes the installed
/// headers and explicit binding arguments; absent server flags are not recovered.
pub fn postgres_clang_flags(flags: &str, msvc: bool) -> Result<Vec<String>, FrontendError> {
    if flags.contains('\0') {
        return Err(FrontendError::Arguments("recorded PostgreSQL flags contain NUL".into()));
    }
    if !msvc {
        return Ok(crate::split_recorded_cflags(flags, false)?.unwrap_or_default());
    }
    let arguments = split_msvc_flags(flags)?;
    let mut arguments = arguments.iter();
    let mut translated = Vec::new();
    while let Some(argument) = arguments.next() {
        let Some(option) = argument.strip_prefix('/').or_else(|| argument.strip_prefix('-')) else {
            return Err(unsupported_msvc_flag(argument));
        };
        if let Some((prefix, output)) = [("FI", "-include"), ("D", "-D"), ("U", "-U"), ("I", "-I")]
            .into_iter()
            .find(|(prefix, _)| option.starts_with(prefix))
        {
            let attached = &option[prefix.len()..];
            let value = if attached.is_empty() {
                arguments.next().map(String::as_str).unwrap_or_default()
            } else {
                attached
            };
            if value.is_empty() {
                return Err(FrontendError::Arguments(format!(
                    "recorded MSVC option {argument:?} requires an operand"
                )));
            }
            if output == "-include" {
                translated.extend([output.into(), value.into()]);
            } else {
                // Attached Clang operands retain filenames/definitions verbatim
                // and cannot become additional driver options.
                translated.push(format!("{output}{value}"));
            }
            continue;
        }
        let mapped: &[&str] = match option {
            "MD" => &["-fms-runtime-lib=dll"],
            "MDd" => &["-fms-runtime-lib=dll_dbg"],
            "MT" => &["-fms-runtime-lib=static"],
            "MTd" => &["-fms-runtime-lib=static_dbg"],
            "J" => &["-funsigned-char"],
            "Zp" | "Zp1" => &["-fpack-struct=1"],
            "Zp2" => &["-fpack-struct=2"],
            "Zp4" => &["-fpack-struct=4"],
            "Zp8" => &["-fpack-struct=8"],
            "Zp16" => &["-fpack-struct=16"],
            "Od" => &["-O0"],
            "O1" => &["-Os", "-fomit-frame-pointer", "-ffunction-sections"],
            "O2" => &["-O3", "-fomit-frame-pointer", "-ffunction-sections"],
            "Ox" => &["-O3", "-fomit-frame-pointer"],
            "Os" => &["-Os"],
            "Ot" => &["-O3"],
            "Ob0" => &["-fno-inline"],
            "Ob1" => &["-finline-hint-functions"],
            "Ob2" | "Ob3" => &["-finline-functions"],
            "Oi" => &["-fbuiltin"],
            "Oi-" => &["-fno-builtin"],
            "Oy" => &["-fomit-frame-pointer"],
            "Oy-" => &["-fno-omit-frame-pointer"],
            "Gy" => &["-ffunction-sections"],
            "Gy-" => &["-fno-function-sections"],
            "GS" => &["-fstack-protector-strong"],
            "GS-" => &["-fno-stack-protector"],
            // Clang's MSVC driver implements disabled string pooling by making
            // literals writable. Preserve that mode for the profile to refuse.
            "GF" => &["-fno-writable-strings"],
            "GF-" => &["-fwritable-strings"],
            "Zi" | "Z7" => &["-gcodeview"],
            "TC" => &["-x", "c"],
            "std:c11" => &["-std=c11"],
            "std:c17" => &["-std=c17"],
            // Warning levels, severity, diagnostic format, and the startup
            // banner change reports only, not preprocessing or C semantics.
            "W0"
            | "W1"
            | "W2"
            | "W3"
            | "W4"
            | "Wall"
            | "WX"
            | "WX-"
            | "w"
            | "nologo"
            | "diagnostics:caret"
            | "diagnostics:column"
            | "diagnostics:classic" => &[],
            _ if ["wd", "we", "wo"].into_iter().any(|prefix| {
                option.strip_prefix(prefix).is_some_and(|number| {
                    !number.is_empty() && number.bytes().all(|byte| byte.is_ascii_digit())
                })
            }) =>
            {
                &[]
            }
            _ => return Err(unsupported_msvc_flag(argument)),
        };
        translated.extend(mapped.iter().map(|argument| (*argument).to_owned()));
    }
    Ok(translated)
}

/// Lower explicit CRT choices for the GNU-style driver on a selected MSVC target.
///
/// Clang 14/15 support the CL runtime options but lack `-fms-runtime-lib`. Use
/// their cc1 equivalents: runtime predefines precede every user `-D`/`-U`, while
/// dependent-library options become COFF linker directives in native objects.
/// The last runtime selector wins; earlier debug/dynamic modes leave no defines.
/// This is a C compilation translation, not a translation of a GNU link command.
/// No mode is inferred when PostgreSQL did not record one. Raw slash CRT selectors are
/// recognized after protecting option operands; GNU `-MD` retains its dependency-output meaning.
/// Non-MSVC argv is returned unchanged, and lowering an already lowered argv is idempotent.
///
/// Recognized operands are never interpreted as selectors, including forced
/// include filenames and opaque `-Xclang` arguments. When a selector is present,
/// unknown option arity is refused rather than guessing whether it consumes it.
pub fn lower_msvc_runtime_flags(
    arguments: &[String],
    msvc: bool,
) -> Result<Vec<String>, FrontendError> {
    if !msvc {
        return Ok(arguments.to_vec());
    }
    let mut retained = Vec::with_capacity(arguments.len());
    let mut selected = None;
    let mut ambiguous = None;
    let mut arguments = arguments.iter();
    while let Some(argument) = arguments.next() {
        if argument.contains('\0') {
            return Err(FrontendError::Arguments("an argument contains a NUL byte".into()));
        }
        if crate::frontend::VALUE_OPTIONS.contains(&argument.as_str())
            || matches!(argument.as_str(), "-x" | "-Xclang" | "-Xpreprocessor" | "-mllvm")
        {
            let value =
                arguments.next().filter(|value| !value.contains('\0')).ok_or_else(|| {
                    FrontendError::Arguments(format!(
                        "{argument} requires a value without NUL bytes"
                    ))
                })?;
            retained.extend([argument.clone(), value.clone()]);
            continue;
        }
        let mode = match argument.as_str() {
            "-fms-runtime-lib=dll" | "/MD" => Some((false, true, "msvcrt")),
            "-fms-runtime-lib=dll_dbg" | "/MDd" => Some((true, true, "msvcrtd")),
            "-fms-runtime-lib=static" | "/MT" => Some((false, false, "libcmt")),
            "-fms-runtime-lib=static_dbg" | "/MTd" => Some((true, false, "libcmtd")),
            _ => None,
        };
        if let Some(mode) = mode {
            selected = Some(mode);
        } else {
            if !runtime_companion_has_no_operand(argument) && ambiguous.is_none() {
                ambiguous = Some(argument.clone());
            }
            retained.push(argument.clone());
        }
    }
    let Some((debug, dynamic, library)) = selected else {
        return Ok(retained);
    };
    if let Some(argument) = ambiguous {
        return Err(FrontendError::Arguments(format!(
            "cannot lower an MSVC runtime selection with unknown option arity for {argument:?}"
        )));
    }
    let mut lowered = Vec::with_capacity(retained.len() + 9);
    if debug {
        lowered.push("-D_DEBUG".into());
    }
    lowered.push("-D_MT".into());
    if dynamic {
        lowered.push("-D_DLL".into());
    } else {
        // This is also emitted by Clang's CL driver for /MT and /MTd. It only
        // affects C++ standard-library visibility; this pipeline requires C.
        lowered.extend(["-Xclang".into(), "-flto-visibility-public-std".into()]);
    }
    lowered.extend([
        "-Xclang".into(),
        format!("--dependent-lib={library}"),
        "-Xclang".into(),
        "--dependent-lib=oldnames".into(),
    ]);
    lowered.extend(retained);
    Ok(lowered)
}

/// Recognize the operand-free flags emitted by the decoder and ordinary explicit profiles.
/// Unknown flags remain usable without CRT lowering; they cannot hide selector operands.
fn runtime_companion_has_no_operand(argument: &str) -> bool {
    matches!(
        argument,
        "-funsigned-char"
            | "-fsigned-char"
            | "-O0"
            | "-O1"
            | "-O2"
            | "-O3"
            | "-Os"
            | "-Oz"
            | "-Og"
            | "-Ofast"
            | "-fomit-frame-pointer"
            | "-fno-omit-frame-pointer"
            | "-ffunction-sections"
            | "-fno-function-sections"
            | "-fdata-sections"
            | "-fno-data-sections"
            | "-fno-inline"
            | "-finline-hint-functions"
            | "-finline-functions"
            | "-fbuiltin"
            | "-fno-builtin"
            | "-fstack-protector-strong"
            | "-fno-stack-protector"
            | "-fno-writable-strings"
            | "-fwritable-strings"
            | "-gcodeview"
            | "-g"
            | "-g0"
            | "-fwrapv"
            | "-fno-wrapv"
            | "-fPIC"
            | "-fpic"
            | "-fPIE"
            | "-fpie"
            | "-fno-PIC"
            | "-fno-PIE"
            | "-fno-lto"
            | "-flto"
            | "-fms-extensions"
            | "-fno-ms-extensions"
            | "-fms-compatibility"
            | "-fno-ms-compatibility"
            | "-fno-strict-aliasing"
            | "-fstrict-aliasing"
            | "-pthread"
            | "-nostdinc"
            | "-nobuiltininc"
            | "-Qunused-arguments"
            | "-xc"
            | "-xc-header"
    ) || ["-D", "-U", "-I", "-F"]
        .iter()
        .any(|prefix| argument.strip_prefix(prefix).is_some_and(|value| !value.is_empty()))
        || [
            "--target=",
            "-target=",
            "-ccc-gcc-name=",
            "--gcc-triple=",
            "--sysroot=",
            "-resource-dir=",
            "--gcc-toolchain=",
            "-gcc-toolchain=",
            "--gcc-install-dir=",
            "-std=",
            "-fpack-struct=",
            "-fvisibility=",
            "-ferror-limit=",
            "-fms-compatibility-version=",
        ]
        .iter()
        .any(|prefix| argument.starts_with(prefix))
        || (argument.starts_with("-W") && argument.len() > 2 && !argument.starts_with("-Wp,"))
}

/// Explain a refused option without silently discarding an unknown compiler mode.
fn unsupported_msvc_flag(argument: &str) -> FrontendError {
    FrontendError::Arguments(format!(
        "recorded MSVC option {argument:?} has no verified GNU-style Clang translation"
    ))
}

/// Decode Windows metadata through the shared quoting contract, requiring balanced fragments.
/// Recorded MSVC flags occupy one command-line fragment; embedded control characters are refused
/// before splitting so no hidden command-line records can enter the selected profile.
fn split_msvc_flags(flags: &str) -> Result<Vec<String>, FrontendError> {
    if flags.chars().any(|character| character.is_control() && character != '\t') {
        return Err(FrontendError::Arguments(
            "recorded MSVC flags must be one command-line fragment without control characters"
                .into(),
        ));
    }
    Ok(crate::split_recorded_cflags(flags, true)?.unwrap_or_default())
}

/// Cover recorded GNU/MSVC quoting, known semantic translations, and exact refusals.
#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeMap;
    use std::path::{Path, PathBuf};

    /// Preserve GNU shell quoting and its explicit malformed-fragment failure.
    #[test]
    fn gnu_flags_keep_shell_semantics() {
        assert_eq!(
            postgres_clang_flags("-I'/path with spaces' -DVALUE=\"a b\" -fwrapv", false).unwrap(),
            ["-I/path with spaces", "-DVALUE=a b", "-fwrapv"]
        );
        assert!(postgres_clang_flags("-DVALUE='", false).is_err());
        for msvc in [false, true] {
            for missing in ["", "  ", "not recorded"] {
                assert!(postgres_clang_flags(missing, msvc).unwrap().is_empty());
            }
        }
    }

    /// Preserve Windows path separators, quoted spaces, escaped quotes, and empty operands.
    #[test]
    fn msvc_fragments_follow_windows_c_quoting() {
        assert_eq!(
            split_msvc_flags(
                r#"/I"C:\Program Files\PostgreSQL\include" /I C:\plain\include /DNAME=\"value\""#
            )
            .unwrap(),
            [
                r"/IC:\Program Files\PostgreSQL\include",
                r"/I",
                r"C:\plain\include",
                r#"/DNAME="value""#
            ]
        );
        for (input, expected) in [
            (r#""a b c" d e"#, vec!["a b c", "d", "e"]),
            (r#""ab\"c" "\\" d"#, vec![r#"ab"c"#, r"\", "d"]),
            (r#"a\\\b d"e f"g h"#, vec![r"a\\\b", "de fg", "h"]),
            (r#"a\\\"b c d"#, vec![r#"a\"b"#, "c", "d"]),
            (r#"a\\\\"b c" d e"#, vec![r"a\\b c", "d", "e"]),
            (r#""" a"#, vec!["", "a"]),
        ] {
            assert_eq!(split_msvc_flags(input).unwrap(), expected, "{input}");
        }
        for malformed in [r#"a"b"" c d"#, r#"/I"unterminated"#] {
            assert!(split_msvc_flags(malformed).is_err(), "{malformed}");
        }
    }

    /// Keep preprocessing operands verbatim and emit only known semantic counterparts.
    #[test]
    fn msvc_flags_preserve_order_and_preprocessor_operands() {
        assert_eq!(
            postgres_clang_flags(r#"/DVALUE=1 /U VALUE /I"C:\Program Files\include" /FI C:\forced.h /MDd /J /Zp8 /Od /O2 /W3 /wd4996 /nologo"#, true).unwrap(),
            ["-DVALUE=1", "-UVALUE", r"-IC:\Program Files\include", "-include", r"C:\forced.h", "-fms-runtime-lib=dll_dbg", "-funsigned-char", "-fpack-struct=8", "-O0", "-O3", "-fomit-frame-pointer", "-ffunction-sections"]
        );
        assert!(postgres_clang_flags(" not recorded ", true).unwrap().is_empty());
        for input in [
            "/D",
            "/I\"\"",
            "/Zp3",
            "/fp:fast",
            "/Gz",
            "/FC",
            "/unknown",
            "@options.rsp",
            "\"\"",
            "/wdabc",
            "/DVALUE=1\n/O2",
        ] {
            assert!(postgres_clang_flags(input, true).is_err(), "{input}");
        }
        assert!(postgres_clang_flags("/DVALUE=1\0/O2", true).is_err());
    }

    /// Resolve the complete recorded/explicit argv once, keeping user overrides last.
    #[test]
    fn runtime_lowering_combines_fragments_and_preserves_user_overrides() {
        let mut combined = postgres_clang_flags("/D_DEBUG=7 /MDd", true).unwrap();
        combined.extend(postgres_clang_flags("/MT /U_DLL", true).unwrap());
        combined.extend(["-fms-runtime-lib=dll".into(), "-U_MT".into()]);
        let lowered = lower_msvc_runtime_flags(&combined, true).unwrap();
        assert_eq!(
            lowered,
            [
                "-D_MT",
                "-D_DLL",
                "-Xclang",
                "--dependent-lib=msvcrt",
                "-Xclang",
                "--dependent-lib=oldnames",
                "-D_DEBUG=7",
                "-U_DLL",
                "-U_MT",
            ]
        );
        assert_eq!(lower_msvc_runtime_flags(&lowered, true).unwrap(), lowered);
        assert_eq!(lower_msvc_runtime_flags(&combined, false).unwrap(), combined);
        let gnu = postgres_clang_flags("-fwrapv -Werror -I'/path with spaces'", false).unwrap();
        assert_eq!(lower_msvc_runtime_flags(&gnu, false).unwrap(), gnu);
    }

    /// Recognize raw retained MSVC runtime tails while preserving filenames and GNU dependency flags.
    #[test]
    fn raw_runtime_tails_are_detected_without_reinterpreting_operands() {
        for (raw, marker) in [
            ("/MD", "-fms-runtime-lib=dll"),
            ("/MDd", "-fms-runtime-lib=dll_dbg"),
            ("/MT", "-fms-runtime-lib=static"),
            ("/MTd", "-fms-runtime-lib=static_dbg"),
        ] {
            let tail = [raw.into()];
            let lowered = lower_msvc_runtime_flags(&tail, true).unwrap();
            assert_ne!(lowered, tail, "retained bindgen tails must expose {raw}");
            assert_eq!(lowered, lower_msvc_runtime_flags(&[marker.into()], true).unwrap());
            assert_eq!(lower_msvc_runtime_flags(&tail, false).unwrap(), tail);
            for option in crate::frontend::VALUE_OPTIONS.iter().copied().chain([
                "-x",
                "-Xclang",
                "-Xpreprocessor",
                "-mllvm",
            ]) {
                let operands = [option.into(), raw.into()];
                assert_eq!(
                    lower_msvc_runtime_flags(&operands, true).unwrap(),
                    operands,
                    "{option} {raw}"
                );
                let mixed = [option.into(), raw.into(), "/MT".into()];
                let lowered = lower_msvc_runtime_flags(&mixed, true).unwrap();
                assert_eq!(&lowered[lowered.len() - 2..], &operands, "{option} {raw}");
                assert!(lowered.contains(&"--dependent-lib=libcmt".into()));
            }
        }
        let dependency = ["-MD".into()];
        assert_eq!(lower_msvc_runtime_flags(&dependency, true).unwrap(), dependency);
        assert!(lower_msvc_runtime_flags(&["-MD".into(), "/MT".into()], true).is_err());
        assert!(
            lower_msvc_runtime_flags(&["/MDd".into(), "/MT".into()], true)
                .unwrap()
                .contains(&"--dependent-lib=libcmt".into())
        );
    }

    /// A selector spelling used as an operand is data, even beside a genuine selector.
    #[test]
    fn runtime_lowering_never_reinterprets_operands() {
        for option in crate::frontend::VALUE_OPTIONS.iter().copied().chain([
            "-x",
            "-Xclang",
            "-Xpreprocessor",
            "-mllvm",
        ]) {
            let argv = [option.into(), "-fms-runtime-lib=dll".into()];
            assert_eq!(lower_msvc_runtime_flags(&argv, true).unwrap(), argv, "{option}");
            let with_mode =
                [option.into(), "-fms-runtime-lib=dll".into(), "-fms-runtime-lib=static".into()];
            let lowered = lower_msvc_runtime_flags(&with_mode, true).unwrap();
            assert_eq!(&lowered[lowered.len() - 2..], &argv, "{option}");
            assert!(lowered.contains(&"--dependent-lib=libcmt".into()));
            assert!(!lowered.contains(&"--dependent-lib=msvcrt".into()));
        }
        let forced = postgres_clang_flags(r#"/FI "-fms-runtime-lib=dll" /MT"#, true).unwrap();
        let lowered = lower_msvc_runtime_flags(&forced, true).unwrap();
        assert_eq!(&lowered[lowered.len() - 2..], &["-include", "-fms-runtime-lib=dll"]);
        for argv in [
            vec!["-unknown-option", "-fms-runtime-lib=dll"],
            vec!["-include"],
            vec!["-Xclang"],
            vec!["-include", "bad\0operand"],
        ] {
            let argv = argv.into_iter().map(str::to_owned).collect::<Vec<_>>();
            assert!(lower_msvc_runtime_flags(&argv, true).is_err(), "{argv:?}");
        }
    }

    /// Run a header-free Windows target probe through the frontend's bounded driver runner.
    fn run_windows_driver(flags: &str, cl: bool, source: &Path, dump: bool) -> String {
        let compiler = std::env::var_os("CLANG_PATH")
            .map(PathBuf::from)
            .unwrap_or_else(|| PathBuf::from(if cfg!(windows) { "clang.exe" } else { "clang" }));
        let mut arguments = vec!["--target=x86_64-pc-windows-msvc".into()];
        if cl {
            arguments.push("--driver-mode=cl".into());
            arguments.extend(split_msvc_flags(flags).unwrap());
            if dump {
                arguments.extend(["/E".into(), "/clang:-dM".into()]);
            } else {
                arguments.push("/Zs".into());
            }
            arguments.push("/TC".into());
        } else {
            arguments.extend(
                lower_msvc_runtime_flags(&postgres_clang_flags(flags, true).unwrap(), true)
                    .unwrap(),
            );
            if dump {
                arguments.extend(["-E".into(), "-dM".into()]);
            } else {
                arguments.push("-fsyntax-only".into());
            }
            arguments.extend(["-x".into(), "c".into()]);
        }
        arguments.push(source.to_str().expect("temporary fixture path must be UTF-8").into());
        crate::frontend::run_compiler(&compiler, &arguments)
            .expect("actual Clang Windows target probe must succeed")
            .stdout
    }

    /// Compare observable predefines from explicit MSVC options against their translations.
    #[test]
    fn translated_flags_preserve_windows_driver_predefines() {
        let directory = tempfile::tempdir().unwrap();
        let source = directory.path().join("flags.c");
        std::fs::write(&source, "/* No platform SDK is needed for predefined macros. */\n")
            .unwrap();
        let selected = |output: String| {
            output
                .lines()
                .filter_map(|line| {
                    let rest = line.strip_prefix("#define ")?;
                    let (name, value) = rest.split_once(' ')?;
                    [
                        "_MT",
                        "_DLL",
                        "_DEBUG",
                        "__CHAR_UNSIGNED__",
                        "__OPTIMIZE__",
                        "__OPTIMIZE_SIZE__",
                        "__STDC_VERSION__",
                        "DECODED_VALUE",
                    ]
                    .contains(&name)
                    .then(|| (name.to_owned(), value.to_owned()))
                })
                .collect::<BTreeMap<_, _>>()
        };
        for flags in [
            "/MD",
            "/MDd",
            "/MT",
            "/MTd",
            "/MDd /MT",
            "/MT /MDd",
            "/D_DEBUG=7 /MT",
            "/MDd /U_DEBUG",
            "/D_DLL=7 /MT",
            "/U_MT /MD",
            "/MD /U_MT",
            "/MD /J",
            "/MD /O1",
            "/MD /O2",
            "/MD /Od",
            "/MD /O2 /Od",
            "/MD /std:c11",
            "/MD /std:c17",
            r#"/MD /DDECODED_VALUE="\"a b\"" /U_DEBUG"#,
        ] {
            let original = selected(run_windows_driver(flags, true, &source, true));
            let translated = selected(run_windows_driver(flags, false, &source, true));
            assert_eq!(translated, original, "{flags}");
        }
    }

    /// Compile real COFF objects without Windows headers, a Windows linker, or llvm-lib.
    fn windows_object(flags: &str, cl: bool, source: &Path, object: &Path) -> Vec<u8> {
        let compiler = std::env::var_os("CLANG_PATH")
            .map(PathBuf::from)
            .unwrap_or_else(|| PathBuf::from(if cfg!(windows) { "clang.exe" } else { "clang" }));
        let mut arguments = vec!["--target=x86_64-pc-windows-msvc19.20.0".into()];
        if cl {
            arguments.extend(["--driver-mode=cl".into(), "/TC".into(), "/c".into()]);
            arguments.extend(split_msvc_flags(flags).unwrap());
            arguments.push(format!("/Fo{}", object.to_str().unwrap()));
        } else {
            arguments.extend(
                lower_msvc_runtime_flags(&postgres_clang_flags(flags, true).unwrap(), true)
                    .unwrap(),
            );
            arguments.extend([
                "-x".into(),
                "c".into(),
                "-c".into(),
                "-o".into(),
                object.to_str().unwrap().into(),
            ]);
        }
        arguments.push(source.to_str().unwrap().into());
        crate::frontend::run_compiler(&compiler, &arguments)
            .expect("SDK-free COFF compilation must succeed");
        std::fs::read(object).unwrap()
    }

    /// Read the compiler's COFF linker directive section, separate from symbols or string tables.
    fn coff_directives(object: &[u8]) -> Vec<String> {
        assert_eq!(&object[..2], &[0x64, 0x86], "expected AMD64 COFF machine code");
        let sections = u16::from_le_bytes(object[2..4].try_into().unwrap()) as usize;
        let optional = u16::from_le_bytes(object[16..18].try_into().unwrap()) as usize;
        let headers = &object[20 + optional..20 + optional + sections * 40];
        let section = headers
            .chunks_exact(40)
            .find(|section| &section[..8] == b".drectve")
            .expect("explicit CRT choices must emit COFF linker directives");
        let length = u32::from_le_bytes(section[16..20].try_into().unwrap()) as usize;
        let offset = u32::from_le_bytes(section[20..24].try_into().unwrap()) as usize;
        std::str::from_utf8(object.get(offset..offset + length).unwrap())
            .unwrap()
            .split_whitespace()
            .map(|directive| directive.trim_matches('"').to_owned())
            .collect()
    }

    /// Match /MD[d] and /MT[d] libraries in actual objects, including last-wins runtime modes.
    #[test]
    fn translated_runtime_preserves_coff_linker_directives() {
        let directory = tempfile::tempdir().unwrap();
        let source = directory.path().join("runtime.c");
        std::fs::write(&source, "int runtime_probe(void) { return 42; }\n").unwrap();
        for (flags, library) in [
            ("/MD", "msvcrt"),
            ("/MDd", "msvcrtd"),
            ("/MT", "libcmt"),
            ("/MTd", "libcmtd"),
            ("/MDd /MT", "libcmt"),
            ("/MT /MDd", "msvcrtd"),
            ("/MDd /U_DEBUG /D_DLL=7", "msvcrtd"),
        ] {
            let original = windows_object(flags, true, &source, &directory.path().join("cl.obj"));
            let translated =
                windows_object(flags, false, &source, &directory.path().join("gnu.obj"));
            let expected =
                vec![format!("/DEFAULTLIB:{library}.lib"), "/DEFAULTLIB:oldnames.lib".into()];
            assert_eq!(coff_directives(&original), expected, "CL driver: {flags}");
            assert_eq!(coff_directives(&translated), expected, "translated driver: {flags}");
        }
    }

    /// Verify translated packing preserves actual Windows record offsets, size, and alignment.
    #[test]
    fn translated_packing_preserves_windows_driver_layout() {
        let directory = tempfile::tempdir().unwrap();
        let source = directory.path().join("packing.c");
        for (flags, alignment, size) in [
            ("/Zp", 1, 9),
            ("/Zp1", 1, 9),
            ("/Zp2", 2, 10),
            ("/Zp4", 4, 12),
            ("/Zp8", 8, 16),
            ("/Zp16", 8, 16),
        ] {
            std::fs::write(&source, format!(
                "struct FlagRecord {{ char tag; unsigned long long value; }};\n\
                 _Static_assert(sizeof(struct FlagRecord) == {size}, \"record size\");\n\
                 _Static_assert(_Alignof(struct FlagRecord) == {alignment}, \"record alignment\");\n\
                 _Static_assert(__builtin_offsetof(struct FlagRecord, value) == {alignment}, \"member offset\");\n"
            )).unwrap();
            run_windows_driver(flags, true, &source, false);
            run_windows_driver(flags, false, &source, false);
        }
    }
}
