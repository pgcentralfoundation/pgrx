/*!

Function level metadata for Rust to SQL translation

> Like all of the [`sql_entity_graph`][crate::pgx_sql_entity_graph] APIs, this is considered **internal**
to the `pgx` framework and very subject to change between versions. While you may use this, please do it with caution.


*/
use super::{FunctionMetadataEntity, PhantomDataExt, SqlTranslatable};
use core::marker::PhantomData;

/**
Provide SQL generation related information on functions

```rust
use pgx_sql_entity_graph::metadata::{FunctionMetadata, Returns, SqlMapping};
fn floof(i: i32) -> String { todo!() }

type FunctionPointer = fn(i32) -> String;
let marker: FunctionPointer = floof;
let metadata = pgx_sql_entity_graph::metadata::FunctionMetadata::entity(&marker);
assert_eq!(
    metadata.retval.unwrap().return_sql,
    Ok(Returns::One(SqlMapping::As("TEXT".to_string()))),
);
```
 */
pub trait FunctionMetadata<Inputs, Output> {
    fn path(&self) -> &'static str {
        core::any::type_name::<Self>()
    }
    fn entity(&self) -> FunctionMetadataEntity;
}

impl<Output> FunctionMetadata<(), Output> for fn() -> Output
where
    Output: SqlTranslatable,
{
    fn entity(&self) -> FunctionMetadataEntity {
        FunctionMetadataEntity {
            arguments: vec![],
            retval: {
                let marker: PhantomData<Output> = PhantomData;
                Some(marker.entity())
            },
            path: self.path(),
        }
    }
}

impl<Output> FunctionMetadata<(), Output> for unsafe fn() -> Output
where
    Output: SqlTranslatable,
{
    fn entity(&self) -> FunctionMetadataEntity {
        FunctionMetadataEntity {
            arguments: vec![],
            retval: {
                let marker: PhantomData<Output> = PhantomData;
                Some(marker.entity())
            },
            path: self.path(),
        }
    }
}

impl FunctionMetadata<(), ()> for fn() {
    fn entity(&self) -> FunctionMetadataEntity {
        FunctionMetadataEntity { arguments: vec![], retval: None, path: self.path() }
    }
}

impl FunctionMetadata<(), ()> for unsafe fn() {
    fn entity(&self) -> FunctionMetadataEntity {
        FunctionMetadataEntity { arguments: vec![], retval: None, path: self.path() }
    }
}

macro_rules! impl_fn {
    ($($T:ident),* $(,)?) => {
        impl<$($T: SqlTranslatable,)* Output: SqlTranslatable> FunctionMetadata<($($T,)*), Output> for fn($($T,)*) -> Output {
            fn entity(&self) -> FunctionMetadataEntity {
                FunctionMetadataEntity {
                    arguments: vec![$(PhantomData::<$T>.entity()),+],
                    retval: Some(PhantomData::<Output>.entity()),
                    path: self.path(),
                }
            }
        }
        impl<$($T: SqlTranslatable,)* Output: SqlTranslatable> FunctionMetadata<($($T,)*), Output> for unsafe fn($($T,)*) -> Output {
            fn entity(&self) -> FunctionMetadataEntity {
                FunctionMetadataEntity {
                    arguments: vec![$(PhantomData::<$T>.entity()),+],
                    retval: Some(PhantomData::<Output>.entity()),
                    path: self.path(),
                }
            }
        }
        impl<$($T: SqlTranslatable,)*> FunctionMetadata<($($T,)*), ()> for fn($($T,)*) {
            fn entity(&self) -> FunctionMetadataEntity {
                FunctionMetadataEntity {
                    arguments: vec![$(PhantomData::<$T>.entity()),+],
                    retval: None,
                    path: self.path(),
                }
            }
        }
        impl<$($T: SqlTranslatable,)*> FunctionMetadata<($($T,)*), ()> for unsafe fn($($T,)*) {
            fn entity(&self) -> FunctionMetadataEntity {
                FunctionMetadataEntity {
                    arguments: vec![$(PhantomData::<$T>.entity()),+],
                    retval: None,
                    path: self.path(),
                }
            }
        }
    };
}
// empty tuples are above
impl_fn!(T0);
impl_fn!(T0, T1);
impl_fn!(T0, T1, T2);
impl_fn!(T0, T1, T2, T3);
impl_fn!(T0, T1, T2, T3, T4);
impl_fn!(T0, T1, T2, T3, T4, T5);
impl_fn!(T0, T1, T2, T3, T4, T5, T6);
impl_fn!(T0, T1, T2, T3, T4, T5, T6, T7);
impl_fn!(T0, T1, T2, T3, T4, T5, T6, T7, T8);
impl_fn!(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9);
impl_fn!(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10);
impl_fn!(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11);
impl_fn!(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12);
impl_fn!(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13);
impl_fn!(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14);
impl_fn!(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15);
impl_fn!(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16);
impl_fn!(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17);
impl_fn!(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18);
impl_fn!(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19);
impl_fn!(
    T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20
);
impl_fn!(
    T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20,
    T21
);
impl_fn!(
    T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20,
    T21, T22
);
impl_fn!(
    T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20,
    T21, T22, T23
);
impl_fn!(
    T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20,
    T21, T22, T23, T24
);
impl_fn!(
    T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20,
    T21, T22, T23, T24, T25
);
impl_fn!(
    T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20,
    T21, T22, T23, T24, T25, T26
);
impl_fn!(
    T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20,
    T21, T22, T23, T24, T25, T26, T27
);
impl_fn!(
    T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20,
    T21, T22, T23, T24, T25, T26, T27, T28
);
impl_fn!(
    T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20,
    T21, T22, T23, T24, T25, T26, T27, T28, T29
);
impl_fn!(
    T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20,
    T21, T22, T23, T24, T25, T26, T27, T28, T29, T30
);
impl_fn!(
    T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20,
    T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31
);
