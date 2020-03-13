use crate::{direct_function_call_as_datum, pg_sys, IntoDatum};

pub struct Numeric(String);

impl Into<Numeric> for i8 {
    fn into(self) -> Numeric {
        Numeric(format!("{}", self))
    }
}

impl Into<Numeric> for i16 {
    fn into(self) -> Numeric {
        Numeric(format!("{}", self))
    }
}

impl Into<Numeric> for i32 {
    fn into(self) -> Numeric {
        Numeric(format!("{}", self))
    }
}

impl Into<Numeric> for i64 {
    fn into(self) -> Numeric {
        Numeric(format!("{}", self))
    }
}

impl Into<Numeric> for u8 {
    fn into(self) -> Numeric {
        Numeric(format!("{}", self))
    }
}

impl Into<Numeric> for u16 {
    fn into(self) -> Numeric {
        Numeric(format!("{}", self))
    }
}

impl Into<Numeric> for u32 {
    fn into(self) -> Numeric {
        Numeric(format!("{}", self))
    }
}

impl Into<Numeric> for u64 {
    fn into(self) -> Numeric {
        Numeric(format!("{}", self))
    }
}

impl Into<Numeric> for f32 {
    fn into(self) -> Numeric {
        Numeric(format!("{}", self))
    }
}

impl Into<Numeric> for f64 {
    fn into(self) -> Numeric {
        Numeric(format!("{}", self))
    }
}

impl IntoDatum<Numeric> for Numeric {
    fn into_datum(self) -> Option<pg_sys::Datum> {
        let cstring =
            std::ffi::CString::new(self.0).expect("failed to convert numeric string into CString");
        let cstr = cstring.as_c_str();
        direct_function_call_as_datum(pg_sys::numeric_in, vec![cstr.into_datum()])
    }
}
