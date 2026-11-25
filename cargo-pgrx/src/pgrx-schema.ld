/*
 * Portions Copyright 2026 PgCentral Foundation, Inc. <contact@pgcentral.org>
 * Use of this source code is governed by the MIT license in the LICENSE file.
 */
SECTIONS
{
  .pgrxsc : { KEEP(*(.pgrxsc)) }
}
/* Even a schema-only cdylib has .dynsym; .rodata may have been collected. */
INSERT AFTER .dynsym;
