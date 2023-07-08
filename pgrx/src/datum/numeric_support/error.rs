use std::fmt;
use std::fmt::{Display, Formatter};

/// Represents some kind of conversion error when working with Postgres numerics
#[derive(Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum Error {
    /// A conversion to Numeric would produce a value outside the precision and scale constraints
    /// of the target Numeric
    OutOfRange(String),

    /// A provided value is not also a valid Numeric
    Invalid(String),

    /// Postgres versions less than 14 do not support `Infinity` and `-Infinity` values
    ConversionNotSupported(String),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Error::OutOfRange(s) => write!(f, "{}", s),
            Error::Invalid(s) => write!(f, "{}", s),
            Error::ConversionNotSupported(s) => write!(f, "{}", s),
        }
    }
}

impl std::error::Error for Error {}
