use super::{FunctionMetadataEntity, PhantomDataExt, SqlTranslatable};
use core::marker::PhantomData;

pub trait FunctionMetadata<Inputs, Output> {
    fn arguments(&self) -> Vec<Box<dyn PhantomDataExt>>;
    fn retval(&self) -> Option<Box<dyn PhantomDataExt>>;
    fn path(&self) -> &'static str {
        core::any::type_name::<Self>()
    }
    fn entity(&self) -> FunctionMetadataEntity {
        FunctionMetadataEntity {
            arguments: self.arguments().into_iter().map(|v| v.entity()).collect(),
            retval: self.retval().map(|v| v.entity()),
            path: self.path(),
        }
    }
}

impl<Output> FunctionMetadata<(), Output> for fn() -> Output
where
    Output: SqlTranslatable + 'static,
{
    fn arguments(&self) -> Vec<Box<dyn PhantomDataExt>> {
        Vec::new()
    }
    fn retval(&self) -> Option<Box<dyn PhantomDataExt>> {
        Some(Box::new({
            let marker: PhantomData<Output> = PhantomData;
            marker
        }) as Box<dyn PhantomDataExt>)
    }
}

impl<Output> FunctionMetadata<(), Output> for unsafe fn() -> Output
where
    Output: SqlTranslatable + 'static,
{
    fn arguments(&self) -> Vec<Box<dyn PhantomDataExt>> {
        Vec::new()
    }
    fn retval(&self) -> Option<Box<dyn PhantomDataExt>> {
        Some(Box::new({
            let marker: PhantomData<Output> = PhantomData;
            marker
        }) as Box<dyn PhantomDataExt>)
    }
}

impl FunctionMetadata<(), ()> for fn() {
    fn arguments(&self) -> Vec<Box<dyn PhantomDataExt>> {
        Vec::new()
    }
    fn retval(&self) -> Option<Box<dyn PhantomDataExt>> {
        None
    }
}

impl FunctionMetadata<(), ()> for unsafe fn() {
    fn arguments(&self) -> Vec<Box<dyn PhantomDataExt>> {
        Vec::new()
    }
    fn retval(&self) -> Option<Box<dyn PhantomDataExt>> {
        None
    }
}

seq_macro::seq!(I in 0..=32 {
    #(
        seq_macro::seq!(N in 0..=I {
            impl<#(Input~N,)* Output> FunctionMetadata<(#(Input~N,)*), Output> for fn(#(Input~N,)*) -> Output
            where
                #(
                    Input~N: SqlTranslatable + 'static,
                )*
                Output: SqlTranslatable + 'static,
            {
                fn arguments(&self) -> Vec<Box<dyn PhantomDataExt>> {
                    let mut vec = Vec::new();
                    #(
                        vec.push(Box::new({
                            let marker: PhantomData<Input~N> = PhantomData;
                            marker
                        }) as Box<dyn PhantomDataExt>);
                    )*
                    vec
                }
                fn retval(&self) -> Option<Box<dyn PhantomDataExt>> {
                    Some(Box::new({
                        let marker: PhantomData<Output> = PhantomData;
                        marker
                    }) as Box<dyn PhantomDataExt>)
                }
            }

            impl<#(Input~N,)* Output> FunctionMetadata<(#(Input~N,)*), Output> for unsafe fn(#(Input~N,)*) -> Output
            where
                #(
                    Input~N: SqlTranslatable + 'static,
                )*
                Output: SqlTranslatable + 'static,
            {
                fn arguments(&self) -> Vec<Box<dyn PhantomDataExt>> {
                    let mut vec = Vec::new();
                    #(
                        vec.push(Box::new({
                            let marker: PhantomData<Input~N> = PhantomData;
                            marker
                        }) as Box<dyn PhantomDataExt>);
                    )*
                    vec
                }
                fn retval(&self) -> Option<Box<dyn PhantomDataExt>> {
                    Some(Box::new({
                        let marker: PhantomData<Output> = PhantomData;
                        marker
                    }) as Box<dyn PhantomDataExt>)
                }
            }

            impl<#(Input~N,)*> FunctionMetadata<(#(Input~N,)*), ()> for fn(#(Input~N,)*)
            where
                #(
                    Input~N: SqlTranslatable + 'static,
                )*
            {
                fn arguments(&self) -> Vec<Box<dyn PhantomDataExt>> {
                    let mut vec = Vec::new();
                    #(
                        vec.push(Box::new({
                            let marker: PhantomData<Input~N> = PhantomData;
                            marker
                        }) as Box<dyn PhantomDataExt>);
                    )*
                    vec
                }
                fn retval(&self) -> Option<Box<dyn PhantomDataExt> >{
                    None
                }
            }

            impl<#(Input~N,)*> FunctionMetadata<(#(Input~N,)*), ()> for unsafe fn(#(Input~N,)*)
            where
                #(
                    Input~N: SqlTranslatable + 'static,
                )*
            {
                fn arguments(&self) -> Vec<Box<dyn PhantomDataExt>> {
                    let mut vec = Vec::new();
                    #(
                        vec.push(Box::new({
                            let marker: PhantomData<Input~N> = PhantomData;
                            marker
                        }) as Box<dyn PhantomDataExt>);
                    )*
                    vec
                }
                fn retval(&self) -> Option<Box<dyn PhantomDataExt> >{
                    None
                }
            }
        });
    )*
});
