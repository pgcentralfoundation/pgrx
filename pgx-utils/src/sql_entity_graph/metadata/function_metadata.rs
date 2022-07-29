use super::{FunctionMetadataEntity, PhantomDataExt, SqlTranslatable};
use core::marker::PhantomData;

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
        FunctionMetadataEntity {
            arguments: vec![],
            retval: None,
            path: self.path(),
        }
    }
}

impl FunctionMetadata<(), ()> for unsafe fn() {
    fn entity(&self) -> FunctionMetadataEntity {
        FunctionMetadataEntity {
            arguments: vec![],
            retval: None,
            path: self.path(),
        }
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
