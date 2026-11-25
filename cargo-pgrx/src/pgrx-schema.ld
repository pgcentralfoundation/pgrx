/*
 * Portions Copyright 2026 PgCentral Foundation, Inc. <contact@pgcentral.org>
 * Use of this source code is governed by the MIT license in the LICENSE file.
 */
/*
 * cargo-pgrx passes this script to the link of an extension's shared library so that
 * the linker's section garbage collection keeps the embedded SQL schema records.
 * GNU ld and LLVM lld read it. mold and gold cannot: when the link fails on this file,
 * set CARGO_PGRX_DISABLE_GC_SECTIONS_WORKAROUND=1 for cargo-pgrx, which then leaves the
 * script out. See cargo-pgrx's README.
 */
SECTIONS
{
  .pgrxsc : { KEEP(*(.pgrxsc)) }
}
/* Even a schema-only cdylib has .dynsym; .rodata may have been collected. */
INSERT AFTER .dynsym;
