/* LICENSE Portions Copyright 2026 PgCentral Foundation, Inc. <contact@pgcentral.org> */
/* LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file. */
#include <stdio.h>
int main(void)
{
    printf("rank\t%d\n", FALLBACK_RANK(FALLBACK_DARK));
    printf("address\t%d\n", FALLBACK_ADDRESS() != 0);
}
