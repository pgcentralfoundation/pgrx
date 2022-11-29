/*
Portions Copyright 2019-2021 ZomboDB, LLC.
Portions Copyright 2021-2022 Technology Concepts & Design, Inc. <support@tcdi.com>

All rights reserved.

Use of this source code is governed by the MIT license that can be found in the LICENSE file.
*/

use std::any::TypeId;

/// A serializable equivalent to [`TypeId`]. Should not be used for downcasting.
#[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize)]
pub struct TyId {
    #[serde(skip)]
    full: Option<TypeId>,
    /// Two hash codes. Not particularly good, so if [TypeId]
    hash: [u64; 2],
}

impl TyId {
    pub fn of<T: 'static>() -> Self {
        Self::from(TypeId::of::<T>())
    }

    pub fn id(&self) -> Option<TypeId> {
        self.full
    }

    /// NB: don't use for downcasting!
    pub fn same(&self, t: &TypeId) -> bool {
        self.hash == Self::hash_bits(t)
    }

    fn hash_bits(t: &TypeId) -> [u64; 2] {
        fn extract_hash<V: core::hash::Hash, H: core::hash::Hasher>(val: &V, mut hasher: H) -> u64 {
            val.hash(&mut hasher);
            hasher.finish()
        }
        // Intentionally going through `RandomState`!
        let sip_code = extract_hash(t, std::collections::hash_map::DefaultHasher::default());
        let fx_code = extract_hash(t, rustc_hash::FxHasher::default());
        [sip_code, fx_code]
    }
}

impl Eq for TyId {}
impl Ord for TyId {
    fn cmp(&self, o: &Self) -> core::cmp::Ordering {
        self.hash.cmp(&o.hash)
    }
}
impl PartialOrd for TyId {
    fn partial_cmp(&self, o: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(&o))
    }
}
impl PartialEq for TyId {
    fn eq(&self, o: &Self) -> bool {
        let same = self.hash == o.hash;
        if same && self.full.is_some() && o.full.is_some() && self.full != o.full {
            assert_eq!(self.full, o.full, "hash collision between: {:?}", (self, o));
        }
        same
    }
}
impl core::hash::Hash for TyId {
    fn hash<H: core::hash::Hasher>(&self, h: &mut H) {
        self.hash.hash(h);
    }
}
impl From<TypeId> for TyId {
    fn from(t: TypeId) -> Self {
        Self { hash: Self::hash_bits(&t), full: Some(t) }
    }
}

impl PartialEq<TypeId> for TyId {
    fn eq(&self, o: &TypeId) -> bool {
        self.same(o)
    }
}
impl PartialEq<TyId> for TypeId {
    fn eq(&self, o: &TyId) -> bool {
        o.same(self)
    }
}
