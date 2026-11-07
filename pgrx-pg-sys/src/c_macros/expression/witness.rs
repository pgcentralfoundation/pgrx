//LICENSE Portions Copyright 2026 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.

//! Result types of documentation shells, without evaluating anything.
//!
//! A documentation shell renders the real macro's value expression, but replaces
//! each helper call whose result type its signature fixes with [`type_witness`].
//! The proofs below compile in every build, so changing one of those signatures
//! stops this crate from compiling until the generator's collapse rule changes.

use super::super::{CInt, CLong, CSize};
use super::*;

/// A never-evaluated value of `M`'s C expression type, used by documentation shells.
///
/// Shells call this only on branches that cannot execute. Calling it panics.
#[doc(hidden)]
pub fn type_witness<M: CType>() -> M::Value {
    unreachable!("C type witnesses are never evaluated")
}

/// Require two expressions to have one type.
fn same<T>(_: T, _: T) {}

/// The generator collapses a call to each helper here to `type_witness` of the
/// marker shown, so each helper must return exactly that marker's value type.
#[expect(dead_code, reason = "compiled only for its type checks")]
#[allow(clippy::too_many_arguments)] // One argument per helper call keeps each proof independent.
fn collapse_rules<S, M, V, W, L, R, T, F, X, P, O>(
    cast_value: V,
    cast_as_value: V,
    implicit_value: W,
    storage: S,
    equality: [(L, R); 2],
    ordering: [(T, T); 4],
    negated: V,
    sized_value: Option<X>,
    sized_place: Option<P>,
    member: Option<O>,
) where
    M: CType<Storage = S> + CompleteObject,
    V: CastTo<M> + Truth,
    W: ImplicitTo<M>,
    L: Equality<R>,
    R: CExprValue,
    T: Compare<T>,
    X: CExprValue,
    X::Marker: CompleteObject,
    P: SizeablePlace,
    O: RecordExpression,
    CRecord<O::Record>: Field<F, ReadOnly>,
    <CRecord<O::Record> as Field<F, ReadOnly>>::Output: SizeablePlace,
    M: OffsetPath<F>,
{
    same(cast::<M, V>(cast_value), type_witness::<M>());
    same(cast_as::<S, M, V>(cast_as_value), type_witness::<M>());
    same(implicit::<M, W>(implicit_value), type_witness::<M>());
    same(<M as CType>::from_storage(storage), type_witness::<M>());

    let [(left, right), (other_left, other_right)] = equality;
    same(eq(left, right), type_witness::<CInt>());
    same(ne(other_left, other_right), type_witness::<CInt>());
    let [lesser, at_most, greater, at_least] = ordering;
    same(lt(lesser.0, lesser.1), type_witness::<CInt>());
    same(le(at_most.0, at_most.1), type_witness::<CInt>());
    same(gt(greater.0, greater.1), type_witness::<CInt>());
    same(ge(at_least.0, at_least.1), type_witness::<CInt>());
    same(logical_not(negated), type_witness::<CInt>());
    same(super::super::CValue::<CInt>::new(0), type_witness::<CInt>());

    same(size_of::<M>(), type_witness::<CSize>());
    same(align_of::<M>(), type_witness::<CSize>());
    same(offset_of::<M, F>(), type_witness::<CSize>());
    same(size_of_value_type(sized_value), type_witness::<CSize>());
    same(size_of_place_type(sized_place), type_witness::<CSize>());
    same(record::size_of_member_type::<F, O>(member), type_witness::<CSize>());

    same(expect(type_witness::<CLong>(), type_witness::<CLong>()), type_witness::<CLong>());
}
