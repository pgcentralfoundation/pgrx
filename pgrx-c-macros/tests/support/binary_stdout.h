//LICENSE Portions Copyright 2026 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.

/* Oracle observations must preserve every output byte, including intentional
 * CRLFs. Configure the initialized Windows CRT before either C or Rust main
 * writes anything, rather than rewriting an inseparable mixed output stream.
 * This header is appended to test programs only, after their original source
 * so its declarations cannot change the fixture's source-location probes.
 */
#ifdef _WIN32
#include <fcntl.h>
#include <io.h>
#include <stdio.h>
#include <stdlib.h>

/* Clang registers this function in the target's normal startup machinery;
 * the CRT has initialized stdout, and no oracle output has been buffered yet.
 * A failed mode change must fail the oracle rather than distort observations.
 */
__attribute__((constructor)) static void pgrx_c_oracle_binary_stdout(void)
{
    if (_setmode(_fileno(stdout), _O_BINARY) == -1)
        _Exit(1);
}
#endif
