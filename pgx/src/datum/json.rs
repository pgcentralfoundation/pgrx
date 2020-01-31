use crate::{
    direct_function_call, direct_function_call_as_datum, pg_sys, rust_str_to_text_p, void_mut_ptr,
    DetoastedVarlenA, FromDatum, IntoDatum,
};
use serde::{Serialize, Serializer};
use serde_json::Value;

#[derive(Debug)]
pub struct Json(pub Value);

#[derive(Debug)]
pub struct JsonB(pub Value);

/// for json
impl FromDatum<Json> for Json {
    #[inline]
    unsafe fn from_datum(datum: pg_sys::Datum, is_null: bool, typoid: pg_sys::Oid) -> Option<Json> {
        if is_null {
            None
        } else if datum == 0 {
            panic!("a json Datum was flagged as non-null but the datum is zero");
        } else {
            let string = DetoastedVarlenA::from_datum(datum, is_null, typoid).unwrap();
            let value = serde_json::from_str(&string).expect("failed to parse Json value");
            Some(Json(value))
        }
    }
}

/// for jsonb
impl FromDatum<JsonB> for JsonB {
    unsafe fn from_datum(datum: pg_sys::Datum, is_null: bool, _: pg_sys::Oid) -> Option<JsonB> {
        if is_null {
            None
        } else if datum == 0 {
            panic!("a jsonb Datum was flagged as non-null but the datum is zero")
        } else {
            let cstr =
                direct_function_call::<&std::ffi::CStr>(pg_sys::jsonb_out, vec![Some(datum)])
                    .expect("failed to convert jsonb to a cstring");

            let value = serde_json::from_str(
                cstr.to_str()
                    .expect("text version of json is not valid UTF8"),
            )
            .expect("failed to parse JsonB value");

            // free the cstring returned from direct_function_call -- we don't need it anymore
            pg_sys::pfree(cstr.as_ptr() as void_mut_ptr);

            // return the parsed serde_json::Value
            Some(JsonB(value))
        }
    }
}

/// for json
impl IntoDatum<Json> for Json {
    fn into_datum(self) -> Option<pg_sys::Datum> {
        let string = serde_json::to_string(&self.0).expect("failed to serialize Json value");
        rust_str_to_text_p(string.as_str()).into_datum()
    }
}

/// for jsonb
impl IntoDatum<JsonB> for JsonB {
    fn into_datum(self) -> Option<pg_sys::Datum> {
        let string = serde_json::to_string(&self.0).expect("failed to serialize JsonB value");
        let cstring =
            std::ffi::CString::new(string).expect("string version of jsonb is not valid UTF8");

        direct_function_call_as_datum(
            pg_sys::jsonb_in,
            vec![Some(cstring.as_ptr() as pg_sys::Datum)],
        )
    }
}

impl Serialize for Json {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        self.0.serialize(serializer)
    }
}

impl Serialize for JsonB {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        self.0.serialize(serializer)
    }
}
