/*
Portions Copyright 2019-2021 ZomboDB, LLC.
Portions Copyright 2021-2022 Technology Concepts & Design, Inc. <support@tcdi.com>

All rights reserved.

Use of this source code is governed by the MIT license that can be found in the LICENSE file.
*/

mod aggregate_tests;
mod anyarray_tests;
mod array_tests;
mod attributes_tests;
mod bgworker_tests;
mod bytea_tests;
mod cfg_tests;
mod datetime_tests;
mod default_arg_value_tests;
mod derive_pgtype_lifetimes;
mod enum_type_tests;
mod fcinfo_tests;
mod guc_tests;
mod heap_tuple;
#[cfg(feature = "cshim")]
mod hooks_tests;
mod inet_tests;
mod internal_tests;
mod json_tests;
mod lifetime_tests;
mod log_tests;
mod memcxt_tests;
mod name_tests;
mod numeric_tests;
mod pg_extern_tests;
mod pg_guard_tests;
mod pg_try_tests;
mod pgbox_tests;
mod pgx_module_qualification;
mod postgres_type_tests;
mod range_tests;
mod schema_tests;
mod shmem_tests;
mod spi_tests;
mod srf_tests;
mod struct_type_tests;
mod trigger_tests;
mod uuid_tests;
mod variadic_tests;
mod xact_callback_tests;
mod xid64_tests;
mod zero_datum_edge_cases;

pgx::pg_magic_func!();
