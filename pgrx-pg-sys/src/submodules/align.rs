//LICENSE Portions Copyright 2026 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.
//! `const fn` forms of PostgreSQL's alignment macros from `c.h`.
//!
//! The generated `MAXALIGN!` and its relatives follow the C types of their operands and run only
//! at runtime. These take and return `usize` and also work in constants, such as the size of an
//! on-disk structure. They use this build's alignment constants and PostgreSQL's formulas, which
//! require a power-of-two alignment.

use crate::{ALIGNOF_BUFFER, ALIGNOF_DOUBLE, ALIGNOF_INT, ALIGNOF_SHORT, MAXIMUM_ALIGNOF};

/// `TYPEALIGN(alignval, len)`: round `len` up to a multiple of `alignval`.
///
/// # Panics
///
/// Panics if `alignval` is not a power of two, or if the result overflows `usize`.
#[inline]
pub const fn typealign(alignval: usize, len: usize) -> usize {
    assert!(alignval.is_power_of_two(), "alignment must be a power of two");
    (len + (alignval - 1)) & !(alignval - 1)
}

/// `TYPEALIGN_DOWN(alignval, len)`: round `len` down to a multiple of `alignval`.
///
/// # Panics
///
/// Panics if `alignval` is not a power of two.
#[inline]
pub const fn typealign_down(alignval: usize, len: usize) -> usize {
    assert!(alignval.is_power_of_two(), "alignment must be a power of two");
    len & !(alignval - 1)
}

/// `SHORTALIGN(len)`: round `len` up to the alignment of C `short`.
#[inline]
pub const fn shortalign(len: usize) -> usize {
    typealign(ALIGNOF_SHORT as usize, len)
}

/// `INTALIGN(len)`: round `len` up to the alignment of C `int`.
#[inline]
pub const fn intalign(len: usize) -> usize {
    typealign(ALIGNOF_INT as usize, len)
}

/// `DOUBLEALIGN(len)`: round `len` up to the alignment of C `double`.
#[inline]
pub const fn doublealign(len: usize) -> usize {
    typealign(ALIGNOF_DOUBLE as usize, len)
}

/// `MAXALIGN(len)`: round `len` up to `MAXIMUM_ALIGNOF`, the alignment PostgreSQL gives every
/// palloc'd chunk and on-disk tuple.
#[inline]
pub const fn maxalign(len: usize) -> usize {
    typealign(MAXIMUM_ALIGNOF as usize, len)
}

/// `MAXALIGN_DOWN(len)`: round `len` down to a multiple of `MAXIMUM_ALIGNOF`.
#[inline]
pub const fn maxalign_down(len: usize) -> usize {
    typealign_down(MAXIMUM_ALIGNOF as usize, len)
}

/// `BUFFERALIGN(len)`: round `len` up to `ALIGNOF_BUFFER`, the alignment of shared buffers.
#[inline]
pub const fn bufferalign(len: usize) -> usize {
    typealign(ALIGNOF_BUFFER as usize, len)
}

#[cfg(test)]
mod tests {
    use super::*;

    // Each is usable in a constant.
    const HEADER: usize = maxalign(core::mem::offset_of!(crate::PageHeaderData, pd_linp));

    #[test]
    fn round_to_the_alignment() {
        assert_eq!(typealign(8, 0), 0);
        assert_eq!(typealign(8, 1), 8);
        assert_eq!(typealign(8, 8), 8);
        assert_eq!(typealign(8, 9), 16);
        assert_eq!(typealign_down(8, 15), 8);
        assert_eq!(maxalign(1), MAXIMUM_ALIGNOF as usize);
        assert_eq!(maxalign_down(MAXIMUM_ALIGNOF as usize + 1), MAXIMUM_ALIGNOF as usize);
        assert_eq!(bufferalign(1), ALIGNOF_BUFFER as usize);
        assert_eq!(HEADER % MAXIMUM_ALIGNOF as usize, 0);
        assert!(HEADER >= core::mem::offset_of!(crate::PageHeaderData, pd_linp));
    }

    #[test]
    #[should_panic(expected = "power of two")]
    fn reject_an_alignment_that_is_not_a_power_of_two() {
        typealign(6, 1);
    }
}
