//LICENSE Portions Copyright 2026 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.

#define SHELL_ADD(left, right) ((left) + (right))
#define SHELL_UNGROUPED(value) (value) + 1
#define SHELL_CAPTURE(value) ((value) + scope)
#define SHELL_RETURN(condition) if (condition) return -1; else return 256
