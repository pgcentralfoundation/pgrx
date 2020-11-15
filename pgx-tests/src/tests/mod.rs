// Copyright 2020 ZomboDB, LLC <zombodb@gmail.com>. All rights reserved. Use of this source code is
// governed by the MIT license that can be found in the LICENSE file.

mod anyarray_tests;
mod array_tests;
mod bytea_tests;
mod datetime_tests;
mod default_arg_value_tests;
mod derive_pgtype_lifetimes;
mod enum_type_tests;
mod fcinfo_tests;
mod guc_tests;
mod hooks_tests;
mod inet_tests;
mod json_tests;
mod log_tests;
mod memcxt_tests;
mod numeric_tests;
mod pg_extern_args_tests;
mod pg_try_tests;
mod postgres_type_tests;
mod schema_tests;
mod spi_tests;
mod srf_tests;
mod struct_type_tests;
mod variadic_tests;
mod xact_callback_tests;
mod xid64_tests;

pgx::pg_module_magic!();
