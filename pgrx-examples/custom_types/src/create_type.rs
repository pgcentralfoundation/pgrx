use pgrx::pg_sys::{Datum, Oid};
use pgrx::pgrx_sql_entity_graph::metadata::{
    ArgumentError, Returns, ReturnsError, SqlMapping, SqlTranslatable,
};
use pgrx::prelude::*;
use pgrx::{rust_regtypein, StringInfo};
use std::error::Error;
use std::ffi::CStr;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

#[repr(transparent)]
#[derive(
    Copy,
    Clone,
    Debug,
    Ord,
    PartialOrd,
    Eq,
    PartialEq,
    Hash,
    PostgresEq,
    PostgresOrd,
    PostgresHash
)]
struct CustomInteger {
    value: i64,
}

impl Display for CustomInteger {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

unsafe impl SqlTranslatable for CustomInteger {
    fn argument_sql() -> Result<SqlMapping, ArgumentError> {
        Ok(SqlMapping::As("custominteger".into()))
    }

    fn return_sql() -> Result<Returns, ReturnsError> {
        Ok(Returns::One(SqlMapping::As("custominteger".into())))
    }
}

impl FromDatum for CustomInteger {
    unsafe fn from_polymorphic_datum(datum: Datum, is_null: bool, _: Oid) -> Option<Self>
    where
        Self: Sized,
    {
        if is_null {
            None
        } else {
            Some(CustomInteger { value: datum.value() as _ })
        }
    }
}

impl IntoDatum for CustomInteger {
    fn into_datum(self) -> Option<Datum> {
        Some(Datum::from(self.value))
    }

    fn type_oid() -> Oid {
        rust_regtypein::<Self>()
    }
}

#[pg_extern(immutable, parallel_safe, requires = [ "shell_type" ])]
fn custominteger_in(input: &CStr) -> Result<CustomInteger, Box<dyn Error>> {
    Ok(CustomInteger { value: i64::from_str(input.to_str()?)? })
}

#[pg_extern(immutable, parallel_safe, create_or_replace)]
fn custominteger_out<'a>(value: CustomInteger) -> &'a CStr {
    let mut s = StringInfo::new();
    s.push_str(&value.to_string());
    s.into()
}

extension_sql!(
    r#"
CREATE TYPE custominteger; -- shell type

-- hack in the output function -- it'll get replaced later, but it's necessary in order to appease the
-- sql entity graph which doesn't quite have a full understand of all our handwritten code as it relates
-- to the #[derive(PostgresEq/Ord/Hash)] macros on CustomInteger
CREATE FUNCTION custominteger_out(custominteger) RETURNS cstring IMMUTABLE STRICT PARALLEL SAFE LANGUAGE C AS 'MODULE_PATHNAME', 'custominteger_out_wrapper';
"#,
    name = "shell_type",
    bootstrap
);

extension_sql!(
    r#"
CREATE TYPE custominteger (
    INPUT = custominteger_in,
    OUTPUT = custominteger_out,
    LIKE = int8
);
"#,
    name = "concrete_type",
    creates = [Type(CustomInteger)],
    requires = [custominteger_in, "shell_type"],
);
