//LICENSE Portions Copyright 2026 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.
//! Lexeme access for [`TsVector`].
//!
//! The walk reads the `TSVectorData` layout from `ts_type.h` through checked slicing, so a
//! corrupt value panics instead of reading out of bounds.
use super::{HEADER_SIZE, TsVector};
use crate::pg_sys;
use core::fmt;
use core::mem::size_of;
use core::ptr;
use core::str::Utf8Error;

const ENTRY_SIZE: usize = size_of::<pg_sys::WordEntry>();

/// The weight label of a lexeme position.
///
/// Declared in increasing order, so `D < C < B < A`: `A` outranks `D`. `D` is the weight of an
/// unlabeled position.
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub enum Weight {
    #[default]
    D = 0,
    C = 1,
    B = 2,
    A = 3,
}

/// The error returned when a character is not a weight label.
#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
#[error("invalid weight label {0:?}, expected A, B, C or D")]
pub struct InvalidWeight(pub char);

impl Weight {
    fn from_bits(bits: u8) -> Self {
        match bits {
            0 => Weight::D,
            1 => Weight::C,
            2 => Weight::B,
            3 => Weight::A,
            _ => unreachable!("a weight field has two bits"),
        }
    }
}

impl TryFrom<char> for Weight {
    type Error = InvalidWeight;

    /// Accepts `A` to `D` in either case, as `setweight` does.
    fn try_from(label: char) -> Result<Self, InvalidWeight> {
        match label.to_ascii_uppercase() {
            'A' => Ok(Weight::A),
            'B' => Ok(Weight::B),
            'C' => Ok(Weight::C),
            'D' => Ok(Weight::D),
            _ => Err(InvalidWeight(label)),
        }
    }
}

impl From<Weight> for char {
    fn from(weight: Weight) -> char {
        match weight {
            Weight::A => 'A',
            Weight::B => 'B',
            Weight::C => 'C',
            Weight::D => 'D',
        }
    }
}

/// A position of a lexeme in the source text, with its weight.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Position {
    /// The position, at most 16383.
    pub pos: u16,
    pub weight: Weight,
}

impl Position {
    fn from_raw(raw: u16) -> Self {
        let pos = pg_sys::WEP_GETPOS!(raw).get() as u16;
        let weight = pg_sys::WEP_GETWEIGHT!(raw).get() as u8;
        Position { pos, weight: Weight::from_bits(weight) }
    }
}

/// A lexeme of a [`TsVector`] with its positions.
#[derive(Clone, Copy)]
pub struct Lexeme<'a> {
    bytes: &'a [u8],
    positions: &'a [u8],
}

impl fmt::Debug for Lexeme<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Lexeme")
            .field("bytes", &String::from_utf8_lossy(self.bytes))
            .field("positions", &self.positions().collect::<Vec<_>>())
            .finish()
    }
}

impl<'a> Lexeme<'a> {
    /// Returns the lexeme in the database encoding.
    pub fn as_bytes(&self) -> &'a [u8] {
        self.bytes
    }

    /// Returns the lexeme as text, if the database encoding is compatible with UTF-8 here.
    pub fn to_str(&self) -> Result<&'a str, Utf8Error> {
        core::str::from_utf8(self.bytes)
    }

    /// Returns the positions in stored order, which is ascending. Empty if the lexeme has none.
    pub fn positions(&self) -> impl ExactSizeIterator<Item = Position> + 'a {
        self.positions
            .as_chunks::<{ size_of::<u16>() }>()
            .0
            .iter()
            .map(|&raw| Position::from_raw(u16::from_ne_bytes(raw)))
    }
}

/// An iterator over the lexemes of a [`TsVector`], in sorted order.
#[derive(Clone)]
pub struct Lexemes<'a> {
    bytes: &'a [u8],
    strings: usize,
    next: usize,
    len: usize,
}

impl fmt::Debug for Lexemes<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Lexemes").field("remaining", &(self.len - self.next)).finish()
    }
}

impl<'a> Iterator for Lexemes<'a> {
    type Item = Lexeme<'a>;

    fn next(&mut self) -> Option<Lexeme<'a>> {
        if self.next == self.len {
            return None;
        }
        let at = HEADER_SIZE + self.next * ENTRY_SIZE;
        self.next += 1;

        let raw = self.bytes.get(at..at + ENTRY_SIZE).expect("entries within the tsvector");
        // SAFETY: `raw` holds `size_of::<WordEntry>()` bytes, and any bit pattern is a valid
        // `WordEntry`.
        let entry = unsafe { ptr::read_unaligned(raw.as_ptr().cast::<pg_sys::WordEntry>()) };
        let (pos, len) = (entry.pos() as usize, entry.len() as usize);

        let strings = &self.bytes[self.strings..];
        let bytes = strings.get(pos..pos + len).expect("lexeme within the tsvector");
        let positions = if entry.haspos() != 0 {
            // The position vector follows the lexeme at the next 2-byte boundary.
            let at = (pos + len + 1) & !1;
            let npos = strings.get(at..at + 2).expect("position count within the tsvector");
            let npos = u16::from_ne_bytes([npos[0], npos[1]]) as usize;
            strings.get(at + 2..at + 2 + 2 * npos).expect("positions within the tsvector")
        } else {
            &[]
        };
        Some(Lexeme { bytes, positions })
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = self.len - self.next;
        (remaining, Some(remaining))
    }
}

impl ExactSizeIterator for Lexemes<'_> {}

impl TsVector {
    /// Returns the number of lexemes.
    pub fn len(&self) -> usize {
        usize::try_from(self.header_size()).expect("tsvector size is not negative")
    }

    /// Returns an iterator over the lexemes and their positions.
    ///
    /// # Panics
    ///
    /// Panics if the value is malformed.
    pub fn lexemes(&self) -> Lexemes<'_> {
        let len = self.len();
        let strings = len
            .checked_mul(ENTRY_SIZE)
            .and_then(|entries| entries.checked_add(HEADER_SIZE))
            .filter(|&strings| strings <= self.0.len())
            .expect("entries within the tsvector");
        Lexemes { bytes: &self.0, strings, next: 0, len }
    }
}

/// Hashes what `Eq` compares: the lexemes, their positions and weights.
impl core::hash::Hash for TsVector {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        self.len().hash(state);
        for lexeme in self.lexemes() {
            lexeme.as_bytes().hash(state);
            let positions = lexeme.positions();
            positions.len().hash(state);
            positions.for_each(|position| position.hash(state));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::TsVectorBuf;
    use super::super::test_support::XorShift;
    use super::super::varlena_words;
    use super::*;

    type Image<'a> = [(&'a [u8], &'a [(u16, Weight)])];
    type Owned = Vec<(Vec<u8>, Vec<(u16, Weight)>)>;

    /// Writes the `ts_type.h` layout: header, entries, then each lexeme followed by its
    /// 2-byte-aligned position vector.
    fn image_bytes(lexemes: &Image<'_>) -> Vec<u8> {
        let mut strings = Vec::new();
        let mut entries = Vec::new();
        for (bytes, positions) in lexemes {
            let pos = strings.len() as u32;
            strings.extend_from_slice(bytes);
            if !positions.is_empty() {
                if strings.len() % 2 == 1 {
                    strings.push(0);
                }
                strings.extend_from_slice(&(positions.len() as u16).to_ne_bytes());
                for &(pos, weight) in *positions {
                    strings.extend_from_slice(&(((weight as u16) << 14) | pos).to_ne_bytes());
                }
            }
            let bits = pg_sys::WordEntry::new_bitfield_1(
                !positions.is_empty() as u32,
                bytes.len() as u32,
                pos,
            );
            let entry = pg_sys::WordEntry { _bindgen_align: [], _bitfield_1: bits };
            // SAFETY: `WordEntry` is four initialized bytes.
            entries.extend_from_slice(unsafe {
                core::slice::from_raw_parts(ptr::addr_of!(entry).cast::<u8>(), ENTRY_SIZE)
            });
        }

        let mut out = vec![0; HEADER_SIZE];
        out[4..8].copy_from_slice(&(lexemes.len() as i32).to_ne_bytes());
        out.extend_from_slice(&entries);
        out.extend_from_slice(&strings);
        out
    }

    fn buf(bytes: Vec<u8>) -> TsVectorBuf {
        TsVectorBuf(varlena_words(&bytes))
    }

    fn image(lexemes: &Image<'_>) -> TsVectorBuf {
        buf(image_bytes(lexemes))
    }

    fn collect(vector: &TsVector) -> Owned {
        vector
            .lexemes()
            .map(|lexeme| {
                (
                    lexeme.as_bytes().to_vec(),
                    lexeme.positions().map(|p| (p.pos, p.weight)).collect(),
                )
            })
            .collect()
    }

    fn owned(lexemes: &Image<'_>) -> Owned {
        lexemes.iter().map(|&(bytes, positions)| (bytes.to_vec(), positions.to_vec())).collect()
    }

    #[test]
    fn empty() {
        let v = image(&[]);
        assert_eq!(v.len(), 0);
        assert!(v.is_empty());
        assert_eq!(v.lexemes().count(), 0);
    }

    #[test]
    fn small_images() {
        use Weight::*;
        let cases: [&Image<'_>; 4] = [
            &[(b"a", &[(1, D), (3, B)]), (b"b", &[(2, C)])],
            &[(b"a", &[]), (b"bc", &[]), (b"d", &[(16383, A)])],
            &[("h\u{e9}llo".as_bytes(), &[(1, A)]), ("\u{65e5}\u{672c}".as_bytes(), &[])],
            &[(b"\xff\x00", &[(1, D)])],
        ];
        for case in cases {
            let v = image(case);
            assert_eq!(v.len(), case.len());
            assert_eq!(v.lexemes().len(), case.len());
            assert_eq!(collect(&v), owned(case));
        }
    }

    #[test]
    fn random_images() {
        let mut rng = XorShift(0x9e37_79b9_7f4a_7c15);
        let weights = [Weight::A, Weight::B, Weight::C, Weight::D];
        for _ in 0..300 {
            let count = rng.below(33);
            let mut lexemes: Owned = Vec::new();
            for _ in 0..count {
                let len = 1 + rng.below(40);
                let bytes = (0..len).map(|_| rng.next() as u8).collect();
                let positions = (0..rng.below(9))
                    .map(|_| ((1 + rng.below(16383)) as u16, weights[rng.below(4)]))
                    .collect();
                lexemes.push((bytes, positions));
            }
            let input: Vec<(&[u8], &[(u16, Weight)])> =
                lexemes.iter().map(|(b, p)| (b.as_slice(), p.as_slice())).collect();
            let v = image(&input);
            assert_eq!(v.lexemes().len(), count);
            assert_eq!(collect(&v), lexemes);
        }
    }

    #[test]
    fn maximum_sizes() {
        let longest = vec![b'x'; pg_sys::MAXSTRLEN as usize];
        let positions: Vec<_> = (1..=pg_sys::MAXNUMPOS as u16).map(|p| (p, Weight::B)).collect();
        let v = image(&[(&longest, &positions)]);
        assert_eq!(collect(&v), owned(&[(&longest, &positions)]));
    }

    #[test]
    fn maximum_string_offset() {
        let filler = vec![b'f'; pg_sys::MAXSTRLEN as usize];
        let last = *b"z";
        let last_positions = [(1, Weight::A)];
        let mut input: Vec<(&[u8], &[(u16, Weight)])> = vec![(&filler, &[]); 512];
        let rest = vec![b'r'; pg_sys::MAXSTRPOS as usize - 512 * filler.len()];
        input.push((&rest, &[]));
        input.push((&last, &last_positions));
        let v = image(&input);
        let lexeme = v.lexemes().last().unwrap();
        assert_eq!(lexeme.as_bytes(), b"z");
        assert_eq!(
            lexeme.positions().collect::<Vec<_>>(),
            [Position { pos: 1, weight: Weight::A }]
        );
        assert_eq!(v.lexemes().count(), 514);
    }

    #[test]
    fn lexeme_text() {
        let v = image(&[("h\u{e9}llo".as_bytes(), &[]), (b"\xff", &[])]);
        let mut lexemes = v.lexemes();
        assert_eq!(lexemes.next().unwrap().to_str(), Ok("h\u{e9}llo"));
        assert!(lexemes.next().unwrap().to_str().is_err());
    }

    #[test]
    #[should_panic(expected = "entries within the tsvector")]
    fn size_larger_than_data() {
        let mut bytes = image_bytes(&[(b"a", &[])]);
        bytes[4..8].copy_from_slice(&100_i32.to_ne_bytes());
        buf(bytes).lexemes();
    }

    #[test]
    #[should_panic(expected = "tsvector size is not negative")]
    fn negative_size() {
        let mut bytes = image_bytes(&[(b"a", &[])]);
        bytes[4..8].copy_from_slice(&(-1_i32).to_ne_bytes());
        buf(bytes).lexemes();
    }

    #[test]
    #[should_panic(expected = "lexeme within the tsvector")]
    fn truncated_lexeme() {
        let mut bytes = image_bytes(&[(b"abc", &[])]);
        bytes.truncate(bytes.len() - 1);
        buf(bytes).lexemes().count();
    }

    #[test]
    #[should_panic(expected = "positions within the tsvector")]
    fn positions_past_the_end() {
        let mut bytes = image_bytes(&[(b"ab", &[(1, Weight::D)])]);
        let npos_at = bytes.len() - 4;
        bytes[npos_at..npos_at + 2].copy_from_slice(&1000_u16.to_ne_bytes());
        buf(bytes).lexemes().count();
    }

    #[test]
    #[should_panic(expected = "position count within the tsvector")]
    fn truncated_position_count() {
        let mut bytes = image_bytes(&[(b"ab", &[(1, Weight::D)])]);
        bytes.truncate(bytes.len() - 4);
        buf(bytes).lexemes().count();
    }
}
