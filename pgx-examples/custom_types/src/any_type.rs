/*
Portions Copyright 2019-2021 ZomboDB, LLC.
Portions Copyright 2021-2022 Technology Concepts & Design, Inc. <support@tcdi.com>

All rights reserved.

Use of this source code is governed by the MIT license that can be found in the LICENSE file.
*/

use pgx::datum::Record;
use pgx::info;
use pgx::prelude::*;

/// Accepts a `record` type which is an anonymous composite type.
/// Parsing records is an exercise left to the reader.
#[pg_extern]
fn accept_record(_record: Record) {
    info!("I can accept a record type!");
}
