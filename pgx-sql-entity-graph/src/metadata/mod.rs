/*!

Function and type level metadata for Rust to SQL translation

> Like all of the [`sql_entity_graph`][crate::pgx_sql_entity_graph] APIs, this is considered **internal**
to the `pgx` framework and very subject to change between versions. While you may use this, please do it with caution.


*/

mod entity;
mod function_metadata;
mod phantomdata_ext;
mod return_variant;
mod sql_translatable;

pub use entity::{FunctionMetadataEntity, FunctionMetadataTypeEntity};
pub use function_metadata::FunctionMetadata;
pub use phantomdata_ext::PhantomDataExt;
pub use return_variant::{Returns, ReturnsError};
pub use sql_translatable::{ArgumentError, SqlMapping, SqlTranslatable};
