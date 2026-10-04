// PostgreSQL macros are generated from the selected installation for builds.
// Documentation uses the versioned snapshots alongside this module.
#![allow(nonstandard_style, unused_parens, unused_braces, unused_imports, clippy::all)]

#[cfg(all(feature = "pg15", not(docsrs)))]
mod pg15 {
    include!(concat!(env!("OUT_DIR"), "/cmacros/pg15/mod.rs"));
}
#[cfg(all(feature = "pg15", docsrs))]
mod pg15;
#[cfg(feature = "pg15")]
pub use pg15::*;

#[cfg(all(feature = "pg16", not(docsrs)))]
mod pg16 {
    include!(concat!(env!("OUT_DIR"), "/cmacros/pg16/mod.rs"));
}
#[cfg(all(feature = "pg16", docsrs))]
mod pg16;
#[cfg(feature = "pg16")]
pub use pg16::*;

#[cfg(all(feature = "pg17", not(docsrs)))]
mod pg17 {
    include!(concat!(env!("OUT_DIR"), "/cmacros/pg17/mod.rs"));
}
#[cfg(all(feature = "pg17", docsrs))]
mod pg17;
#[cfg(feature = "pg17")]
pub use pg17::*;

#[cfg(all(feature = "pg18", not(docsrs)))]
mod pg18 {
    include!(concat!(env!("OUT_DIR"), "/cmacros/pg18/mod.rs"));
}
#[cfg(all(feature = "pg18", docsrs))]
mod pg18;
#[cfg(feature = "pg18")]
pub use pg18::*;

#[cfg(all(feature = "pg19", not(docsrs)))]
mod pg19 {
    include!(concat!(env!("OUT_DIR"), "/cmacros/pg19/mod.rs"));
}
#[cfg(all(feature = "pg19", docsrs))]
mod pg19;
#[cfg(feature = "pg19")]
pub use pg19::*;
