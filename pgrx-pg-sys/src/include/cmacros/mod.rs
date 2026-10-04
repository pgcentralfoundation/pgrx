//! Expose the selected PostgreSQL version's generated C macro module tree.
//!
//! Ordinary builds include freshly generated modules from OUT_DIR, reflecting the
//! current installation, compiler flags, and target. Documentation builds use
//! versioned snapshots stored beside this selector. Their values describe the
//! release-generation installation and are not inputs to ordinary transpilation.
//! Reexports make macro definitions and their hidden adapters available through
//! pgrx-pg-sys and ultimately the pgrx crate root.

// PostgreSQL macros are generated from the selected installation for builds.
// Documentation uses the versioned snapshots alongside this module.
#![allow(nonstandard_style, unused_parens, unused_braces, unused_imports, clippy::all)]

/// Label Rust operands with their original C scalar identity before generated
/// macros apply C promotion and conversion rules. A Rust alias or binding
/// constant's storage type alone cannot establish that identity.
pub mod c {
    pub use crate::__pgrx_c_macros::expression::{CDouble, CFloat};
    pub use crate::__pgrx_c_macros::{
        CBool, CChar, CInt, CInt128, CLong, CLongLong, CPtrDiff, CShort, CSignedChar, CSize,
        CUnsignedChar, CUnsignedInt, CUnsignedInt128, CUnsignedLong, CUnsignedLongLong,
        CUnsignedShort, CValue,
    };
}

/// Include items only when this installation and compiler profile emitted the
/// named C macro. Availability can change across PostgreSQL versions or builds;
/// the body is removed before Rust resolves names when generation refused it.
///
/// ```ignore
/// pgrx::if_c_macro! { CHECK_FOR_INTERRUPTS {
///     unsafe fn check_interrupts() { pgrx::CHECK_FOR_INTERRUPTS!(); }
/// }}
/// ```
#[macro_export]
macro_rules! if_c_macro {
    ($name:ident { $($items:tt)* }) => {
        $crate::__pgrx_c_classify!(@if_available $name { $($items)* });
    };
}

pub use crate::if_c_macro;

/// Compile fresh PG15 macro tree from OUT_DIR for the selected installation and target.
#[cfg(all(feature = "pg15", not(docsrs)))]
mod pg15 {
    //! Compile fresh PG15 macro tree from OUT_DIR for the selected installation and target.
    //!
    //! Keep header modules and hidden native adapters in this versioned scope.
    //! The selector forwards their exports to pg_sys, while macro expansions
    //! retain the defining crate through `$crate`.

    include!(concat!(env!("OUT_DIR"), "/cmacros/pg15/mod.rs"));
}
/// Expose the stored PG15 macro snapshot solely for documentation builds.
#[cfg(all(feature = "pg15", docsrs))]
mod pg15;
#[cfg(feature = "pg15")]
pub use pg15::*;

/// Compile fresh PG16 macro tree from OUT_DIR for the selected installation and target.
#[cfg(all(feature = "pg16", not(docsrs)))]
mod pg16 {
    //! Compile fresh PG16 macro tree from OUT_DIR for the selected installation and target.
    //!
    //! Keep header modules and hidden native adapters in this versioned scope.
    //! The selector forwards their exports to pg_sys, while macro expansions
    //! retain the defining crate through `$crate`.

    include!(concat!(env!("OUT_DIR"), "/cmacros/pg16/mod.rs"));
}
/// Expose the stored PG16 macro snapshot solely for documentation builds.
#[cfg(all(feature = "pg16", docsrs))]
mod pg16;
#[cfg(feature = "pg16")]
pub use pg16::*;

/// Compile fresh PG17 macro tree from OUT_DIR for the selected installation and target.
#[cfg(all(feature = "pg17", not(docsrs)))]
mod pg17 {
    //! Compile fresh PG17 macro tree from OUT_DIR for the selected installation and target.
    //!
    //! Keep header modules and hidden native adapters in this versioned scope.
    //! The selector forwards their exports to pg_sys, while macro expansions
    //! retain the defining crate through `$crate`.

    include!(concat!(env!("OUT_DIR"), "/cmacros/pg17/mod.rs"));
}
/// Expose the stored PG17 macro snapshot solely for documentation builds.
#[cfg(all(feature = "pg17", docsrs))]
mod pg17;
#[cfg(feature = "pg17")]
pub use pg17::*;

/// Compile fresh PG18 macro tree from OUT_DIR for the selected installation and target.
#[cfg(all(feature = "pg18", not(docsrs)))]
mod pg18 {
    //! Compile fresh PG18 macro tree from OUT_DIR for the selected installation and target.
    //!
    //! Keep header modules and hidden native adapters in this versioned scope.
    //! The selector forwards their exports to pg_sys, while macro expansions
    //! retain the defining crate through `$crate`.

    include!(concat!(env!("OUT_DIR"), "/cmacros/pg18/mod.rs"));
}
/// Expose the stored PG18 macro snapshot solely for documentation builds.
#[cfg(all(feature = "pg18", docsrs))]
mod pg18;
#[cfg(feature = "pg18")]
pub use pg18::*;

/// Compile fresh PG19 macro tree from OUT_DIR for the selected installation and target.
#[cfg(all(feature = "pg19", not(docsrs)))]
mod pg19 {
    //! Compile fresh PG19 macro tree from OUT_DIR for the selected installation and target.
    //!
    //! Keep header modules and hidden native adapters in this versioned scope.
    //! The selector forwards their exports to pg_sys, while macro expansions
    //! retain the defining crate through `$crate`.

    include!(concat!(env!("OUT_DIR"), "/cmacros/pg19/mod.rs"));
}
/// Expose the stored PG19 macro snapshot solely for documentation builds.
#[cfg(all(feature = "pg19", docsrs))]
mod pg19;
#[cfg(feature = "pg19")]
pub use pg19::*;
