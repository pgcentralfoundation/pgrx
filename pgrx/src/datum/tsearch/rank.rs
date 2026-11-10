//LICENSE Portions Copyright 2026 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.
//! Ranking of a [`TsVector`] against a [`TsQuery`].
use super::{PgFunction, TsQuery, TsVector};
use crate::datum::IntoDatum;
use crate::direct_function_call;
use crate::pg_sys;
use bitflags::bitflags;

bitflags! {
    /// How a rank is normalized by document length, as the `normalization` argument of
    /// `ts_rank`.
    #[derive(Clone, Copy, Default, Debug, PartialEq, Eq)]
    pub struct RankNormalization: i32 {
        /// Divides the rank by 1 + the logarithm of the document length.
        const LOG_LENGTH = 1;
        /// Divides the rank by the document length.
        const LENGTH = 2;
        /// Divides the rank by the mean harmonic distance between extents. Only for
        /// [`TsVector::rank_cd_with`].
        const MEAN_DISTANCE = 4;
        /// Divides the rank by the number of unique words in the document.
        const UNIQUE_WORDS = 8;
        /// Divides the rank by 1 + the logarithm of the number of unique words in the document.
        const LOG_UNIQUE_WORDS = 16;
        /// Divides the rank by itself + 1.
        const RANK_PLUS_ONE = 32;
    }
}

/// The weight given to each label of a lexeme position when ranking.
///
/// Postgres uses 0.1, 0.2, 0.4 and 1.0 for `D`, `C`, `B` and `A` by default. It raises an error for
/// a weight above 1.0, and uses the default for a negative one.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RankWeights {
    pub a: f32,
    pub b: f32,
    pub c: f32,
    pub d: f32,
}

impl RankWeights {
    fn to_array(self) -> [f32; 4] {
        [self.d, self.c, self.b, self.a]
    }
}

fn int_datum(value: i32) -> pg_sys::Datum {
    value.into_datum().expect("an integer is not null")
}

impl TsVector {
    fn call_rank(&self, f: PgFunction, query: &TsQuery, extra: &[pg_sys::Datum]) -> f32 {
        let mut args = vec![
            Some(pg_sys::Datum::from(self.as_varlena_ptr())),
            Some(pg_sys::Datum::from(query.as_varlena_ptr())),
        ];
        args.extend(extra.iter().copied().map(Some));
        // SAFETY: `f` takes `(tsvector, tsquery, ...)` with the extra arguments in the order its
        // SQL signature has them, and the datums are valid for the call.
        unsafe { direct_function_call::<f32>(f, &args) }.expect("rank function returned null")
    }

    fn call_rank_weighted(
        &self,
        f: PgFunction,
        query: &TsQuery,
        weights: RankWeights,
        normalization: RankNormalization,
    ) -> f32 {
        let array = weights.to_array().to_vec().into_datum().expect("an array is not null");
        let args = [
            Some(array),
            Some(pg_sys::Datum::from(self.as_varlena_ptr())),
            Some(pg_sys::Datum::from(query.as_varlena_ptr())),
            Some(int_datum(normalization.bits())),
        ];
        // SAFETY: `f` takes `(float4[], tsvector, tsquery, int4)`, and the datums are valid for
        // the call.
        let rank = unsafe { direct_function_call::<f32>(f, &args) };
        // SAFETY: `into_datum` allocated the array with `palloc`, and the call returned.
        unsafe { pg_sys::pfree(array.cast_mut_ptr()) };
        rank.expect("rank function returned null")
    }

    /// Ranks how well the vector matches `query` by how often the query lexemes occur, like
    /// `ts_rank(vector, query)`.
    pub fn rank(&self, query: &TsQuery) -> f32 {
        self.call_rank(pg_sys::ts_rank_tt, query, &[])
    }

    /// Like [`rank`](Self::rank), with weights for the position labels and a normalization, like
    /// `ts_rank([weights,] vector, query, normalization)`.
    ///
    /// Postgres raises an error for a weight above 1.0.
    pub fn rank_with(
        &self,
        query: &TsQuery,
        weights: Option<RankWeights>,
        normalization: RankNormalization,
    ) -> f32 {
        match weights {
            None => self.call_rank(pg_sys::ts_rank_ttf, query, &[int_datum(normalization.bits())]),
            Some(weights) => {
                self.call_rank_weighted(pg_sys::ts_rank_wttf, query, weights, normalization)
            }
        }
    }

    /// Ranks how well the vector matches `query` by the proximity of the query lexemes, like
    /// `ts_rank_cd(vector, query)`.
    pub fn rank_cd(&self, query: &TsQuery) -> f32 {
        self.call_rank(pg_sys::ts_rankcd_tt, query, &[])
    }

    /// Like [`rank_cd`](Self::rank_cd), with weights for the position labels and a normalization,
    /// like `ts_rank_cd([weights,] vector, query, normalization)`.
    ///
    /// Postgres raises an error for a weight above 1.0.
    pub fn rank_cd_with(
        &self,
        query: &TsQuery,
        weights: Option<RankWeights>,
        normalization: RankNormalization,
    ) -> f32 {
        match weights {
            None => {
                self.call_rank(pg_sys::ts_rankcd_ttf, query, &[int_datum(normalization.bits())])
            }
            Some(weights) => {
                self.call_rank_weighted(pg_sys::ts_rankcd_wttf, query, weights, normalization)
            }
        }
    }
}
