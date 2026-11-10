//LICENSE Portions Copyright 2026 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.
//! Item access for [`TsQuery`].
//!
//! The walk reads the `TSQueryData` layout from `ts_type.h` through checked slicing, so a
//! corrupt value panics instead of reading out of bounds.
use super::lexeme::Weight;
use super::{HEADER_SIZE, TsQuery};
use crate::pg_sys;
use core::fmt;
use core::mem::size_of;
use core::ptr;
use core::str::Utf8Error;

const ITEM_SIZE: usize = size_of::<pg_sys::QueryItem>();
const TYPE_OFFSET: usize = 0;
const PREFIX_OFFSET: usize = core::mem::offset_of!(pg_sys::QueryOperand, prefix);

/// A set of [`Weight`]s restricting which positions an operand matches.
///
/// The empty set means the operand matches any weight.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct WeightSet(u8);

impl WeightSet {
    const MASK: u8 = 0b1111;

    /// Returns true if `weight` is in the set. This is false for every weight if the set is empty.
    pub fn contains(self, weight: Weight) -> bool {
        self.0 & (1 << weight as u8) != 0
    }

    /// Returns true if the operand matches any weight, which is when the set is empty.
    pub fn is_any(self) -> bool {
        self.0 == 0
    }

    /// Returns the weights in the set, in increasing order.
    pub fn iter(self) -> impl Iterator<Item = Weight> {
        [Weight::D, Weight::C, Weight::B, Weight::A].into_iter().filter(move |&w| self.contains(w))
    }
}

/// An operator of a [`TsQuery`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Operator {
    /// `!`, with one operand.
    Not,
    /// `&`, with two operands.
    And,
    /// `|`, with two operands.
    Or,
    /// `<N>`, with two operands. `<->` is `distance: 1`.
    Phrase { distance: u16 },
}

/// A lexeme operand of a [`TsQuery`].
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Operand<'a> {
    bytes: &'a [u8],
    weights: WeightSet,
    prefix: bool,
}

impl fmt::Debug for Operand<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Operand")
            .field("bytes", &String::from_utf8_lossy(self.bytes))
            .field("weights", &self.weights)
            .field("prefix", &self.prefix)
            .finish()
    }
}

impl<'a> Operand<'a> {
    /// Returns the lexeme in the database encoding.
    pub fn as_bytes(&self) -> &'a [u8] {
        self.bytes
    }

    /// Returns the lexeme as text, if the database encoding is compatible with UTF-8 here.
    pub fn to_str(&self) -> Result<&'a str, Utf8Error> {
        core::str::from_utf8(self.bytes)
    }

    /// Returns the weights the operand is restricted to.
    pub fn weights(&self) -> WeightSet {
        self.weights
    }

    /// Returns true for a prefix match, written `:*`.
    pub fn is_prefix(&self) -> bool {
        self.prefix
    }
}

/// An item of a [`TsQuery`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Item<'a> {
    Operand(Operand<'a>),
    Operator(Operator),
}

/// An iterator over the items of a [`TsQuery`] in prefix order: an operator precedes its
/// operands, the right operand first.
#[derive(Clone)]
pub struct Items<'a> {
    bytes: &'a [u8],
    operands: usize,
    next: usize,
    len: usize,
}

impl fmt::Debug for Items<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Items").field("remaining", &(self.len - self.next)).finish()
    }
}

impl<'a> Iterator for Items<'a> {
    type Item = Item<'a>;

    fn next(&mut self) -> Option<Item<'a>> {
        if self.next == self.len {
            return None;
        }
        let at = HEADER_SIZE + self.next * ITEM_SIZE;
        self.next += 1;

        let raw = self.bytes.get(at..at + ITEM_SIZE).expect("items within the tsquery");
        let kind = raw[TYPE_OFFSET] as u32;
        if kind == pg_sys::QI_OPR {
            // SAFETY: `raw` holds `size_of::<QueryItem>()` bytes and `QueryOperator` has no
            // invalid bit patterns.
            let operator =
                unsafe { ptr::read_unaligned(raw.as_ptr().cast::<pg_sys::QueryOperator>()) };
            let operator = match operator.oper as u32 {
                pg_sys::OP_NOT => Operator::Not,
                pg_sys::OP_AND => Operator::And,
                pg_sys::OP_OR => Operator::Or,
                pg_sys::OP_PHRASE => Operator::Phrase {
                    distance: u16::try_from(operator.distance)
                        .expect("phrase distance is not negative"),
                },
                _ => panic!("known operator in the tsquery"),
            };
            return Some(Item::Operator(operator));
        }
        assert!(kind == pg_sys::QI_VAL, "operand or operator item in the tsquery");
        assert!(raw[PREFIX_OFFSET] <= 1, "prefix flag is a boolean in the tsquery");
        // SAFETY: `raw` holds `size_of::<QueryItem>()` bytes, and the only field with invalid
        // bit patterns, `prefix`, was checked.
        let operand = unsafe { ptr::read_unaligned(raw.as_ptr().cast::<pg_sys::QueryOperand>()) };
        assert!(operand.weight & !WeightSet::MASK == 0, "weight mask within four bits");
        let (distance, length) = (operand.distance() as usize, operand.length() as usize);
        let bytes = self.bytes[self.operands..]
            .get(distance..distance + length)
            .expect("operand within the tsquery");
        Some(Item::Operand(Operand {
            bytes,
            weights: WeightSet(operand.weight),
            prefix: operand.prefix,
        }))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = self.len - self.next;
        (remaining, Some(remaining))
    }
}

impl ExactSizeIterator for Items<'_> {}

impl TsQuery {
    /// Returns an iterator over the items, in prefix order.
    ///
    /// # Panics
    ///
    /// Panics if the value is malformed.
    pub fn items(&self) -> Items<'_> {
        let len = usize::try_from(self.header_size()).expect("tsquery size is not negative");
        let operands = len
            .checked_mul(ITEM_SIZE)
            .and_then(|items| items.checked_add(HEADER_SIZE))
            .filter(|&operands| operands <= self.0.len())
            .expect("items within the tsquery");
        Items { bytes: &self.0, operands, next: 0, len }
    }

    /// Returns an iterator over the operands, in the order of [`items`](Self::items).
    pub fn operands(&self) -> impl Iterator<Item = Operand<'_>> {
        self.items().filter_map(|item| match item {
            Item::Operand(operand) => Some(operand),
            Item::Operator(_) => None,
        })
    }
}

/// Hashes what `Eq` compares: the structure, operators, phrase distances and operand text. `Eq`
/// ignores weights and prefix flags, so the hash does too.
impl core::hash::Hash for TsQuery {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        self.items().len().hash(state);
        for item in self.items() {
            match item {
                Item::Operand(operand) => operand.as_bytes().hash(state),
                Item::Operator(operator) => operator.hash(state),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::TsQueryBuf;
    use super::super::test_support::XorShift;
    use super::super::varlena_words;
    use super::*;

    fn weights(bits: u8) -> WeightSet {
        WeightSet(bits)
    }

    fn operand(bytes: &[u8]) -> Item<'_> {
        Item::Operand(Operand { bytes, weights: weights(0), prefix: false })
    }

    /// Writes the `ts_type.h` layout: header, items, then the NUL-terminated operands.
    fn image_bytes(items: &[Item<'_>]) -> Vec<u8> {
        let mut strings = Vec::new();
        let mut out = vec![0; HEADER_SIZE];
        out[4..8].copy_from_slice(&(items.len() as i32).to_ne_bytes());
        for (at, item) in items.iter().enumerate() {
            let mut raw = [0_u8; ITEM_SIZE];
            match *item {
                Item::Operator(operator) => {
                    let (oper, distance) = match operator {
                        Operator::Not => (pg_sys::OP_NOT, 0),
                        Operator::And => (pg_sys::OP_AND, 0),
                        Operator::Or => (pg_sys::OP_OR, 0),
                        Operator::Phrase { distance } => (pg_sys::OP_PHRASE, distance),
                    };
                    let item = pg_sys::QueryOperator {
                        type_: pg_sys::QI_OPR as i8,
                        oper: oper as i8,
                        distance: distance as i16,
                        left: 1 + at as u32,
                    };
                    // SAFETY: `QueryOperator` is eight initialized bytes, no padding.
                    raw[..8].copy_from_slice(unsafe {
                        core::slice::from_raw_parts(ptr::addr_of!(item).cast::<u8>(), 8)
                    });
                }
                Item::Operand(operand) => {
                    let mut item = pg_sys::QueryOperand {
                        type_: pg_sys::QI_VAL as i8,
                        weight: operand.weights.0,
                        prefix: operand.prefix,
                        valcrc: 7,
                        _bitfield_1: Default::default(),
                    };
                    item.set_length(operand.bytes.len() as u32);
                    item.set_distance(strings.len() as u32);
                    strings.extend_from_slice(operand.bytes);
                    strings.push(0);
                    let start = core::mem::offset_of!(pg_sys::QueryOperand, _bitfield_1);
                    // SAFETY: the bitfield unit is four initialized bytes, no padding.
                    raw[start..start + 4].copy_from_slice(unsafe {
                        core::slice::from_raw_parts(ptr::addr_of!(item._bitfield_1).cast::<u8>(), 4)
                    });
                    raw[TYPE_OFFSET] = item.type_ as u8;
                    raw[core::mem::offset_of!(pg_sys::QueryOperand, weight)] = item.weight;
                    raw[PREFIX_OFFSET] = item.prefix as u8;
                    let valcrc = core::mem::offset_of!(pg_sys::QueryOperand, valcrc);
                    raw[valcrc..valcrc + 4].copy_from_slice(&item.valcrc.to_ne_bytes());
                }
            }
            out.extend_from_slice(&raw);
        }
        out.extend_from_slice(&strings);
        out
    }

    fn buf(bytes: Vec<u8>) -> TsQueryBuf {
        TsQueryBuf(varlena_words(&bytes))
    }

    fn image(items: &[Item<'_>]) -> TsQueryBuf {
        buf(image_bytes(items))
    }

    fn expect_items(query: &TsQueryBuf, expected: &[Item<'_>]) {
        assert_eq!(query.items().len(), expected.len());
        assert_eq!(query.items().collect::<Vec<_>>(), expected);
    }

    #[test]
    fn empty() {
        let q = image(&[]);
        assert!(q.is_empty());
        assert_eq!(q.items().count(), 0);
        assert_eq!(q.operands().count(), 0);
    }

    #[test]
    fn small_images() {
        let items = [
            Item::Operator(Operator::Or),
            Item::Operator(Operator::Phrase { distance: 2 }),
            operand(b"d"),
            Item::Operand(Operand { bytes: b"c", weights: weights(0b0101), prefix: true }),
            Item::Operator(Operator::And),
            Item::Operator(Operator::Not),
            operand("h\u{e9}llo".as_bytes()),
            operand(b"\xff"),
        ];
        let q = image(&items);
        expect_items(&q, &items);
        assert_eq!(q.operands().count(), 4);
    }

    #[test]
    fn operand_accessors() {
        let q = image(&[Item::Operand(Operand {
            bytes: "h\u{e9}llo".as_bytes(),
            weights: weights(0b1001),
            prefix: true,
        })]);
        let operand = q.operands().next().unwrap();
        assert_eq!(operand.to_str(), Ok("h\u{e9}llo"));
        assert!(operand.is_prefix());
        assert!(!operand.weights().is_any());
        assert_eq!(operand.weights().iter().collect::<Vec<_>>(), [Weight::D, Weight::A]);
        assert!(operand.weights().contains(Weight::A) && !operand.weights().contains(Weight::B));
        assert!(WeightSet::default().is_any());
    }

    fn tree<'a>(rng: &mut XorShift, depth: usize, words: &'a [Vec<u8>], out: &mut Vec<Item<'a>>) {
        if depth == 0 || rng.below(3) == 0 {
            out.push(Item::Operand(Operand {
                bytes: &words[rng.below(words.len())],
                weights: weights(rng.below(16) as u8),
                prefix: rng.below(2) == 0,
            }));
            return;
        }
        let operator = match rng.below(4) {
            0 => Operator::Not,
            1 => Operator::And,
            2 => Operator::Or,
            _ => Operator::Phrase { distance: rng.below(16384) as u16 },
        };
        out.push(Item::Operator(operator));
        tree(rng, depth - 1, words, out);
        if operator != Operator::Not {
            tree(rng, depth - 1, words, out);
        }
    }

    #[test]
    fn random_trees() {
        let mut rng = XorShift(0x9e37_79b9_7f4a_7c15);
        let words: Vec<Vec<u8>> = (0..16)
            .map(|_| {
                let len = 1 + rng.below(40);
                (0..len).map(|_| 1 + rng.below(255) as u8).collect()
            })
            .collect();
        for _ in 0..300 {
            let mut items = Vec::new();
            tree(&mut rng, 5, &words, &mut items);
            expect_items(&image(&items), &items);
        }
    }

    #[test]
    fn maximum_operand() {
        let longest = vec![b'x'; pg_sys::MAXSTRLEN as usize];
        let items = [Item::Operator(Operator::And), operand(&longest), operand(b"y")];
        expect_items(&image(&items), &items);
    }

    #[test]
    #[should_panic(expected = "items within the tsquery")]
    fn size_larger_than_data() {
        let mut bytes = image_bytes(&[operand(b"a")]);
        bytes[4..8].copy_from_slice(&100_i32.to_ne_bytes());
        buf(bytes).items();
    }

    #[test]
    #[should_panic(expected = "tsquery size is not negative")]
    fn negative_size() {
        let mut bytes = image_bytes(&[operand(b"a")]);
        bytes[4..8].copy_from_slice(&(-1_i32).to_ne_bytes());
        buf(bytes).items();
    }

    #[test]
    #[should_panic(expected = "operand or operator item in the tsquery")]
    fn stop_word_item() {
        let mut bytes = image_bytes(&[operand(b"a")]);
        bytes[HEADER_SIZE + TYPE_OFFSET] = pg_sys::QI_VALSTOP as u8;
        buf(bytes).items().count();
    }

    #[test]
    #[should_panic(expected = "known operator in the tsquery")]
    fn unknown_operator() {
        let mut bytes = image_bytes(&[Item::Operator(Operator::And)]);
        bytes[HEADER_SIZE + core::mem::offset_of!(pg_sys::QueryOperator, oper)] = 9;
        buf(bytes).items().count();
    }

    #[test]
    #[should_panic(expected = "phrase distance is not negative")]
    fn negative_distance() {
        let mut bytes = image_bytes(&[Item::Operator(Operator::Phrase { distance: 1 })]);
        let at = HEADER_SIZE + core::mem::offset_of!(pg_sys::QueryOperator, distance);
        bytes[at..at + 2].copy_from_slice(&(-1_i16).to_ne_bytes());
        buf(bytes).items().count();
    }

    #[test]
    #[should_panic(expected = "prefix flag is a boolean in the tsquery")]
    fn invalid_prefix_flag() {
        let mut bytes = image_bytes(&[operand(b"a")]);
        bytes[HEADER_SIZE + PREFIX_OFFSET] = 2;
        buf(bytes).items().count();
    }

    #[test]
    #[should_panic(expected = "weight mask within four bits")]
    fn invalid_weight_mask() {
        let mut bytes = image_bytes(&[operand(b"a")]);
        bytes[HEADER_SIZE + core::mem::offset_of!(pg_sys::QueryOperand, weight)] = 0x10;
        buf(bytes).items().count();
    }

    #[test]
    #[should_panic(expected = "operand within the tsquery")]
    fn truncated_operand() {
        let mut bytes = image_bytes(&[operand(b"abc")]);
        bytes.truncate(bytes.len() - 3);
        buf(bytes).items().count();
    }
}
