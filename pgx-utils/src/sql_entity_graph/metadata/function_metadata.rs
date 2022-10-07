/*!

Function level metadata for Rust to SQL translation

> Like all of the [`sql_entity_graph`][crate::sql_entity_graph] APIs, this is considered **internal**
to the `pgx` framework and very subject to change between versions. While you may use this, please do it with caution.


*/
use super::{FunctionMetadataEntity, PhantomDataExt, SqlTranslatable};
use core::marker::PhantomData;

/**
Provide SQL generation related information on functions

```rust
use pgx_utils::sql_entity_graph::metadata::{FunctionMetadata, Returns, SqlMapping};
fn floof(i: i32) -> String { todo!() }

type FunctionPointer = fn(i32) -> String;
let marker: FunctionPointer = floof;
let metadata = pgx_utils::sql_entity_graph::metadata::FunctionMetadata::entity(&marker);
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
seq_macro::seq!(I in 0..=32 {
    #(
        seq_macro::seq!(N in 0..=I {
            impl<'output, #('input~N: 'output,)* #(Input~N,)* Output> FunctionMetadata<(#(Input~N,)*), Output> for fn(#(Input~N,)*) -> Output
            where
                #(
                    Input~N: SqlTranslatable + 'input~N,
                )*
                Output: SqlTranslatable + 'output,
            {
                fn entity(&self) -> FunctionMetadataEntity {
                    let mut arguments = Vec::new();
                    #(
                        arguments.push({
                            let marker: PhantomData<Input~N> = PhantomData;
                            marker.entity()
                        });
                    )*
                    FunctionMetadataEntity {
                        arguments,
                        retval: {
                            let marker: PhantomData<Output> = PhantomData;
                            Some(marker.entity())
                        },
                        path: self.path(),
                    }
                }
            }

            impl<#(Input~N,)* Output> FunctionMetadata<(#(Input~N,)*), Output> for unsafe fn(#(Input~N,)*) -> Output
            where
                #(
                    Input~N: SqlTranslatable,
                )*
                Output: SqlTranslatable,
            {
                fn entity(&self) -> FunctionMetadataEntity {
                    let mut arguments = Vec::new();
                    #(
                        arguments.push({
                            let marker: PhantomData<Input~N> = PhantomData;
                            marker.entity()
                        });
                    )*
                    FunctionMetadataEntity {
                        arguments,
                        retval: {
                            let marker: PhantomData<Output> = PhantomData;
                            Some(marker.entity())
                        },
                        path: self.path(),
                    }
                }
            }

            impl<#(Input~N,)*> FunctionMetadata<(#(Input~N,)*), ()> for fn(#(Input~N,)*)
            where
                #(
                    Input~N: SqlTranslatable,
                )*
            {
                fn entity(&self) -> FunctionMetadataEntity {
                    let mut arguments = Vec::new();
                    #(
                        arguments.push({
                            let marker: PhantomData<Input~N> = PhantomData;
                            marker.entity()
                        });
                    )*
                    FunctionMetadataEntity {
                        arguments,
                        retval: None,
                        path: self.path(),
                    }
                }
            }

            impl<#(Input~N,)*> FunctionMetadata<(#(Input~N,)*), ()> for unsafe fn(#(Input~N,)*)
            where
                #(
                    Input~N: SqlTranslatable,
                )*
            {
                fn entity(&self) -> FunctionMetadataEntity {
                    let mut arguments = Vec::new();
                    #(
                        arguments.push({
                            let marker: PhantomData<Input~N> = PhantomData;
                            marker.entity()
                        });
                    )*
                    FunctionMetadataEntity {
                        arguments,
                        retval: None,
                        path: self.path(),
                    }
                }
            }
        });
    )*
});
