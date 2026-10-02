//LICENSE Portions Copyright 2026 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.

//! Preserve generated C macro operands before Rust makes expression fragments opaque.
//!
//! Normalization changes tokens only. Each use of an operand requests its own C
//! context, so repeated operands and lazy branches retain C substitution semantics.

use super::{MAX_EMISSION_BYTES, macro_identifier, skip};
use crate::{MacroAnalysis, ParameterOrigin, ParameterRole, SkipReason, SkipReasonCode};
use std::borrow::Cow;
use std::collections::BTreeSet;
use std::fmt::Write;

const MAX_ARGUMENTS: usize = 64;
const TOKEN_BUDGET: usize = 64;

pub(super) struct ArgumentAdapter {
    pub normalizer: String,
    pub rust: String,
}

#[derive(Clone, Copy)]
pub(super) enum ArgumentContext {
    Value,
    Place,
    ReadPlace,
    Size,
    Discard,
}

/// Rust accepts keywords as metavariable names, including `self` and `_`.
/// `$crate` alone has special meaning and cannot bind a C argument.
pub(super) fn name(analysis: &MacroAnalysis, index: usize) -> Cow<'_, str> {
    let parameter = &analysis.parameters[index];
    if parameter.origin == ParameterOrigin::Formal
        || (parameter.name != "crate"
            && !analysis.parameters[..index].iter().any(|earlier| earlier.name == parameter.name))
    {
        return Cow::Borrowed(&parameter.name);
    }
    // An expanded macro may capture an identifier with the same spelling as an
    // outer formal. They remain distinct operands under C substitution rules.
    let mut capture = format!("__pgrx_c_capture_{index}");
    while analysis.parameters.iter().any(|parameter| parameter.name == capture) {
        capture.push('_');
    }
    Cow::Owned(capture)
}

/// Explicit-return arms bind an extra type fragment beside the C arguments.
pub(super) fn return_marker(analysis: &MacroAnalysis) -> String {
    let mut marker = String::from("__pgrx_c_return");
    while analysis.parameters.iter().any(|parameter| parameter.name == marker) {
        marker.push('_');
    }
    marker
}

/// Matcher for the internal context arms, after syntactic normalization.
pub(super) fn matcher(analysis: &MacroAnalysis) -> String {
    let mut matcher = String::new();
    for (index, parameter) in analysis.parameters.iter().enumerate() {
        if index != 0 {
            matcher.push_str(", ");
        }
        let fragment = match parameter.roles.as_slice() {
            [ParameterRole::Type] => "ty",
            [ParameterRole::Identifier] => "ident",
            _ => "tt",
        };
        write!(matcher, "${}:{fragment}", name(analysis, index)).expect("String output");
    }
    if !analysis.parameters.is_empty() {
        matcher.push_str(" $(,)?");
    }
    matcher
}

/// Generate linear stages for this macro's compiler-established parameter list.
pub(super) fn generate(analysis: &MacroAnalysis) -> Result<ArgumentAdapter, SkipReason> {
    if analysis.parameters.len() > MAX_ARGUMENTS {
        return Err(skip(
            analysis,
            SkipReasonCode::BudgetExceeded,
            "C macro argument normalization exceeds its 64-parameter bound",
            None,
        ));
    }
    for (index, parameter) in analysis.parameters.iter().enumerate() {
        let name = name(analysis, index);
        let mut bytes = name.bytes();
        if name == "crate"
            || !bytes.next().is_some_and(|byte| byte.is_ascii_alphabetic() || byte == b'_')
            || !bytes.all(|byte| byte.is_ascii_alphanumeric() || byte == b'_')
        {
            return Err(skip(
                analysis,
                SkipReasonCode::UnsupportedType,
                format!(
                    "C argument {:?} cannot retain its name as a Rust metavariable; $crate is reserved and only ASCII identifier spellings are modeled",
                    parameter.name
                ),
                None,
            ));
        }
    }
    let identifier = macro_identifier(&analysis.name).ok_or_else(|| {
        skip(analysis, SkipReasonCode::UnsupportedType, "macro has no Rust identifier", None)
    })?;
    let normalizer =
        format!("__pgrx_c_args_{}", identifier.strip_prefix("r#").unwrap_or(&identifier));
    let mut rust = format!(
        "#[doc(hidden)]\n#[macro_export]\nmacro_rules! {normalizer} {{\n\
         (@collect @$mode:ident [$($done:tt)*]; $($raw:tt)*) => {{ $crate::{normalizer}!(@p0 $mode [$($done)*]; $($raw)*) }};\n\
         (@collect $mode:ident [$($done:tt)*]; $($raw:tt)*) => {{ $crate::{normalizer}!(@p0 $mode [$($done)*]; $($raw)*) }};\n\
         (@classified [$next:ident $mode:ident [$($done:tt)*] [$($rest:tt)*]] $descriptor:tt) => {{ $crate::{normalizer}!(@$next $mode [$($done)* $descriptor,]; $($rest)*) }};\n"
    );
    for (index, parameter) in analysis.parameters.iter().enumerate() {
        let next = index + 1;
        let last = next == analysis.parameters.len();
        if parameter.roles == [ParameterRole::Unused] {
            writeln!(rust, "(@p{index} $mode:ident [$($done:tt)*]; $($raw:tt)*) => {{ $crate::{normalizer}!(@ignore{index} $mode [$($done)*] [{}]; $($raw)*) }};", "@ ".repeat(TOKEN_BUDGET)).expect("String output");
            writeln!(rust, "(@ignore{index} $mode:ident [$($done:tt)*] $budget:tt; , $($rest:tt)*) => {{ $crate::{normalizer}!(@p{next} $mode [$($done)* (@unused),]; $($rest)*) }};").expect("String output");
            if last {
                writeln!(rust, "(@ignore{index} $mode:ident [$($done:tt)*] $budget:tt;) => {{ $crate::{normalizer}!(@p{next} $mode [$($done)* (@unused),];) }};").expect("String output");
            }
            writeln!(rust, "(@ignore{index} $mode:ident [$($done:tt)*] [@ $($budget:tt)*]; $token:tt $($rest:tt)*) => {{ $crate::{normalizer}!(@ignore{index} $mode [$($done)*] [$($budget)*]; $($rest)*) }};").expect("String output");
            writeln!(rust, "(@ignore{index} $mode:ident [$($done:tt)*] []; $($raw:tt)+) => {{ compile_error!(\"unused C macro argument exceeds the 64-token normalization bound\") }};").expect("String output");
            continue;
        }
        if parameter.roles.contains(&ParameterRole::Type)
            && parameter.roles != [ParameterRole::Type]
        {
            return Err(skip(
                analysis,
                SkipReasonCode::UnsupportedType,
                "a macro parameter has incompatible type and operand roles",
                None,
            ));
        }
        if parameter.roles.contains(&ParameterRole::Identifier)
            && parameter.roles != [ParameterRole::Identifier]
        {
            return Err(skip(
                analysis,
                SkipReasonCode::UnsupportedType,
                "a macro parameter combines a field identifier with another operand role",
                None,
            ));
        }
        // A fragment parser's syntax failure is fatal to macro_rules matching.
        // Try the complete final structural argument before the comma form's
        // :expr fallback, which could otherwise try parsing a native escape.
        let endings = if last { vec!["", ", $($rest:tt)*"] } else { vec![", $($rest:tt)*"] };
        let atomic = parameter.uses.iter().any(|usage| !usage.grouped);
        if parameter.roles != [ParameterRole::Type]
            && parameter.roles != [ParameterRole::Identifier]
        {
            // A literal parser commits after '-'. Route other negative operands
            // through :expr after checking both complete literal ending forms.
            for ending in &endings {
                let rest = if ending.is_empty() { "" } else { "$($rest)*" };
                let negative = if atomic {
                    "compile_error!(\"an ungrouped C parameter requires a parenthesized negative literal\")".into()
                } else {
                    format!(
                        "$crate::{normalizer}!(@p{next} $mode [$($done)* (@literal [- $argument]),]; {rest})"
                    )
                };
                writeln!(rust, "(@p{index} $mode:ident [$($done:tt)*]; - $argument:literal{ending}) => {{ {negative} }};").expect("String output");
                writeln!(rust, "(@negative{index} $mode:ident [$($done:tt)*]; $argument:expr{ending}) => {{ $crate::{normalizer}!(@p{next} $mode [$($done)* (@native [$argument]),]; {rest}) }};").expect("String output");
            }
            let negative = if atomic {
                "compile_error!(\"an ungrouped C parameter requires one token tree\")".into()
            } else {
                format!("$crate::{normalizer}!(@negative{index} $mode [$($done)*]; - $($raw)*)")
            };
            writeln!(
                rust,
                "(@p{index} $mode:ident [$($done:tt)*]; - $($raw:tt)*) => {{ {negative} }};"
            )
            .expect("String output");
        }
        for ending in endings {
            let rest = if ending.is_empty() { "" } else { "$($rest)*" };
            let state = format!("[p{next} $mode [$($done)*] [{rest}]]");
            if parameter.roles == [ParameterRole::Type] {
                writeln!(rust, "(@p{index} $mode:ident [$($done:tt)*]; $argument:ty{ending}) => {{ $crate::{normalizer}!(@p{next} $mode [$($done)* $argument,]; {rest}) }};").expect("String output");
                continue;
            }
            if parameter.roles == [ParameterRole::Identifier] {
                writeln!(rust, "(@p{index} $mode:ident [$($done:tt)*]; $argument:ident{ending}) => {{ $crate::{normalizer}!(@p{next} $mode [$($done)* $argument,]; {rest}) }};").expect("String output");
                continue;
            }
            writeln!(rust, "(@p{index} $mode:ident [$($done:tt)*]; (@__pgrx_c_native [$argument:expr]){ending}) => {{ $crate::{normalizer}!(@p{next} $mode [$($done)* (@native [$argument]),]; {rest}) }};").expect("String output");
            // Match calls and groups before :expr capture. Other Rust expressions
            // remain native operands; they are never parsed as C by this helper.
            for prefix in ["", "::"] {
                writeln!(rust, "(@p{index} $mode:ident [$($done:tt)*]; {prefix}$head:ident $(::$tail:ident)* ! $group:tt{ending}) => {{ $crate::__pgrx_c_classify!(@argument [{normalizer}] {state} [{prefix}$head $(::$tail)* ! $group]) }};").expect("String output");
            }
            writeln!(rust, "(@p{index} $mode:ident [$($done:tt)*]; ($($inner:tt)*){ending}) => {{ $crate::__pgrx_c_classify!(@argument [{normalizer}] {state} [($($inner)*)]) }};").expect("String output");
            writeln!(rust, "(@p{index} $mode:ident [$($done:tt)*]; $argument:literal{ending}) => {{ $crate::{normalizer}!(@p{next} $mode [$($done)* (@literal [$argument]),]; {rest}) }};").expect("String output");
            let fragment = if atomic { "tt" } else { "expr" };
            writeln!(rust, "(@p{index} $mode:ident [$($done:tt)*]; $argument:{fragment}{ending}) => {{ $crate::{normalizer}!(@p{next} $mode [$($done)* (@native [$argument]),]; {rest}) }};").expect("String output");
        }
    }
    let count = analysis.parameters.len();
    writeln!(
        rust,
        "(@p{count} $mode:ident [$($done:tt)*];) => {{ $crate::{identifier}!(@$mode; $($done)*) }};"
    )
    .expect("String output");
    rust.push_str("($($invalid:tt)*) => { compile_error!(\"arguments do not satisfy this C macro's invocation contract\") };\n}\n");
    if rust.len() > MAX_EMISSION_BYTES {
        return Err(skip(
            analysis,
            SkipReasonCode::BudgetExceeded,
            "macro argument adapters exceed the output bound",
            None,
        ));
    }
    Ok(ArgumentAdapter { normalizer, rust })
}

/// One export classifier for the entire generated artifact, rather than one per macro.
/// Names are the actual Rust exports, including mappings for reserved C identifiers.
pub(super) fn shared(exports: &BTreeSet<String>) -> Result<String, String> {
    let mut rust = format!(
        "#[doc(hidden)]\n#[macro_export]\nmacro_rules! __pgrx_c_classify {{\n\
         (@argument [$callback:ident] $state:tt [$($original:tt)*]) => {{ $crate::__pgrx_c_classify!(@walk [$callback] $state [$($original)*] [{}]; [$($original)*]) }};\n\
         (@walk [$callback:ident] $state:tt $original:tt [@ $($budget:tt)*]; [($($inner:tt)*)]) => {{ $crate::__pgrx_c_classify!(@walk [$callback] $state $original [$($budget)*]; [$($inner)*]) }};\n\
         (@walk [$callback:ident] $state:tt $original:tt $budget:tt; [@__pgrx_c_native [$expression:expr]]) => {{ $crate::$callback!(@classified $state (@native [$expression])) }};\n\
         (@walk [$callback:ident] $state:tt $original:tt $budget:tt; [- $argument:literal]) => {{ $crate::$callback!(@classified $state (@literal [- $argument])) }};\n\
         (@walk [$callback:ident] $state:tt [$($original:tt)*] $budget:tt; [- $($negative:tt)*]) => {{ $crate::$callback!(@classified $state (@native [$($original)*])) }};\n\
         (@walk [$callback:ident] $state:tt $original:tt $budget:tt; [$argument:literal]) => {{ $crate::$callback!(@classified $state (@literal [$argument])) }};\n\
         (@walk [$callback:ident] $state:tt $original:tt $budget:tt; [$head:ident $(::$tail:ident)* ! $group:tt]) => {{ $crate::__pgrx_c_classify!(@path [$callback] $state $original [$head $(::$tail)*] $group $budget; $head $(::$tail)*) }};\n\
         (@walk [$callback:ident] $state:tt $original:tt $budget:tt; [::$head:ident $(::$tail:ident)* ! $group:tt]) => {{ $crate::__pgrx_c_classify!(@path [$callback] $state $original [::$head $(::$tail)*] $group $budget; $head $(::$tail)*) }};\n\
         (@walk [$callback:ident] $state:tt [$($original:tt)*] []; [$($raw:tt)*]) => {{ compile_error!(\"C macro argument exceeds the 64-group normalization bound\") }};\n\
         (@walk [$callback:ident] $state:tt [$($original:tt)*] $budget:tt; [$($raw:tt)*]) => {{ $crate::$callback!(@classified $state (@native [$($original)*])) }};\n\
         (@path [$callback:ident] $state:tt $original:tt $path:tt $group:tt [@ $($budget:tt)*]; $head:ident :: $($tail:tt)+) => {{ $crate::__pgrx_c_classify!(@path [$callback] $state $original $path $group [$($budget)*]; $($tail)+) }};\n",
        "@ ".repeat(TOKEN_BUDGET)
    );
    for name in exports {
        if super::rust_identifier(name.strip_prefix("r#").unwrap_or(name)).as_deref() != Some(name)
        {
            return Err(format!("{name}: invalid generated Rust macro export"));
        }
        writeln!(rust, "(@path [$callback:ident] $state:tt $original:tt [$($path:tt)*] $group:tt $budget:tt; {name}) => {{ $crate::__pgrx_c_classify!(@known [$callback] $state [$($path)*]; $group) }};").expect("String output");
        writeln!(rust, "(@if_available {name} {{ $($items:tt)* }}) => {{ $($items)* }};")
            .expect("String output");
        if rust.len() > MAX_EMISSION_BYTES {
            return Err("generated macro export classifier exceeds the output bound".into());
        }
    }
    rust.push_str(
        "(@if_available $unknown:ident { $($items:tt)* }) => {};\n\
         (@path [$callback:ident] $state:tt [$($original:tt)*] $path:tt $group:tt $budget:tt; $last:ident) => { $crate::$callback!(@classified $state (@native [$($original)*])) };\n\
         (@path [$callback:ident] $state:tt $original:tt $path:tt $group:tt []; $($raw:tt)+) => { compile_error!(\"C macro path exceeds the 64-component normalization bound\") };\n\
         (@known [$callback:ident] $state:tt [$($path:tt)*]; ($($inner:tt)*)) => { $crate::$callback!(@classified $state (@macro [$($path)*] [$($inner)*])) };\n\
         (@known [$callback:ident] $state:tt [$($path:tt)*]; [$($inner:tt)*]) => { $crate::$callback!(@classified $state (@macro [$($path)*] [$($inner)*])) };\n\
         (@known [$callback:ident] $state:tt [$($path:tt)*]; {$($inner:tt)*}) => { $crate::$callback!(@classified $state (@macro [$($path)*] [$($inner)*])) };\n\
         ($($invalid:tt)*) => { compile_error!(\"invalid generated C macro argument descriptor\") };\n}\n\
         #[doc(hidden)]\n#[macro_export]\nmacro_rules! __pgrx_c_operand {\n\
         (@value [$floats:tt]; (@native [$expression:expr])) => { $crate::__pgrx_c_macros::expression::profile_input::<$floats, _>($expression) };\n\
         (@place; (@native [$expression:expr])) => { $crate::__pgrx_c_macros::expression::native_place(::core::ptr::addr_of_mut!($expression)) };\n\
         (@read_place; (@native [$expression:expr])) => { $crate::__pgrx_c_macros::expression::native_const_place(::core::ptr::addr_of!($expression)) };\n\
         (@size; (@native [$expression:expr])) => { $crate::__pgrx_c_macros::expression::size_of_value_type(if false { Some(unsafe { $crate::__pgrx_c_macros::expression::input($expression) }) } else { None }) };\n\
         (@discard [$floats:tt]; (@native [$expression:expr])) => { { let _ = $crate::__pgrx_c_macros::expression::profile_input::<$floats, _>($expression); } };\n\
         (@value [$floats:tt]; (@literal [$argument:expr])) => { $crate::__pgrx_c_macros::expression::profile_value::<$floats, _>($crate::__pgrx_c_macros::expression::literal::value::<{ let __pgrx_c_literal = $argument; (__pgrx_c_literal as u128) == 0 }, _>($argument)) };\n\
         (@place; (@literal [$argument:expr])) => { compile_error!(\"a C literal is not an object place\") };\n\
         (@read_place; (@literal [$argument:expr])) => { compile_error!(\"a C literal is not an object place\") };\n\
         (@size; (@literal [$argument:expr])) => { $crate::__pgrx_c_macros::expression::size_of_value_type(if false { Some($crate::__pgrx_c_macros::expression::literal::value::<{ let __pgrx_c_literal = $argument; (__pgrx_c_literal as u128) == 0 }, _>($argument)) } else { None }) };\n\
         (@discard [$floats:tt]; (@literal [$argument:expr])) => { { let _ = $crate::__pgrx_c_operand!(@value [$floats]; (@literal [$argument])); } };\n\
         (@value [$floats:tt]; (@macro [$($path:tt)*] [$($arguments:tt)*])) => { $($path)* !(@__pgrx_c_value; $($arguments)*) };\n\
         (@place; (@macro [$($path:tt)*] [$($arguments:tt)*])) => { $($path)* !(@__pgrx_c_place; $($arguments)*) };\n\
         (@read_place; (@macro [$($path:tt)*] [$($arguments:tt)*])) => { $($path)* !(@__pgrx_c_read_place; $($arguments)*) };\n\
         (@size; (@macro [$($path:tt)*] [$($arguments:tt)*])) => { $($path)* !(@__pgrx_c_size; $($arguments)*) };\n\
         (@discard [$floats:tt]; (@macro [$($path:tt)*] [$($arguments:tt)*])) => { $($path)* !(@__pgrx_c_discard; $($arguments)*) };\n\
         (@value [$floats:tt]; (@compiled [$($value:tt)*] $place:tt $read:tt $size:tt)) => { $($value)* };\n\
         (@place; (@compiled $value:tt [$($place:tt)*] $read:tt $size:tt)) => { $($place)* };\n\
         (@read_place; (@compiled $value:tt $place:tt [$($read:tt)*] $size:tt)) => { $($read)* };\n\
         (@size; (@compiled $value:tt $place:tt $read:tt [$($size:tt)*])) => { $($size)* };\n\
         (@discard [$floats:tt]; (@compiled [$($value:tt)*] $place:tt $read:tt $size:tt)) => { { let _ = $($value)*; } };\n\
         ($($invalid:tt)*) => { compile_error!(\"invalid C macro operand context\") };\n}\n"
    );
    if rust.len() > MAX_EMISSION_BYTES {
        return Err("generated macro argument support exceeds the output bound".into());
    }
    Ok(rust)
}

pub(super) fn render_operand(
    analysis: &MacroAnalysis,
    parameter: usize,
    context: ArgumentContext,
    floats: bool,
    rust: &mut String,
) {
    let mode = match context {
        ArgumentContext::Value => format!("@value [{floats}]"),
        ArgumentContext::Place => "@place".into(),
        ArgumentContext::ReadPlace => "@read_place".into(),
        ArgumentContext::Size => "@size".into(),
        ArgumentContext::Discard => format!("@discard [{floats}]"),
    };
    write!(rust, "$crate::__pgrx_c_operand!({mode}; ${})", name(analysis, parameter))
        .expect("String output");
}
