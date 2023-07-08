//LICENSE Portions Copyright 2019-2021 ZomboDB, LLC.
//LICENSE
//LICENSE Portions Copyright 2021-2023 Technology Concepts & Design, Inc.
//LICENSE
//LICENSE Portions Copyright 2023-2023 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE All rights reserved.
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.
// Constants defined in PG13+
mod typalign {
    pub const TYPALIGN_CHAR: u8 = b'c';
    pub const TYPALIGN_SHORT: u8 = b's';
    pub const TYPALIGN_INT: u8 = b'i';
    pub const TYPALIGN_DOUBLE: u8 = b'd';
}

#[cfg(any(feature = "pg11", feature = "pg12"))]
pub use typalign::*;
