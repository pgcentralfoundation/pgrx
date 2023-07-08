//LICENSE Portions Copyright 2019-2021 ZomboDB, LLC.
//LICENSE
//LICENSE Portions Copyright 2021-2023 Technology Concepts & Design, Inc.
//LICENSE
//LICENSE Portions Copyright 2023-2023 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE All rights reserved.
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.
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
