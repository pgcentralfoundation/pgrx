#LICENSE Portions Copyright 2019-2021 ZomboDB, LLC.
#LICENSE
#LICENSE Portions Copyright 2021-2023 Technology Concepts & Design, Inc.
#LICENSE
#LICENSE Portions Copyright 2023-2023 PgCentral Foundation, Inc. <contact@pgcentral.org>
#LICENSE
#LICENSE All rights reserved.
#LICENSE
#LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.
 
MODULE_big = pgrx-cshim-${PG_TARGET_VERSION}
OBJS = pgrx-cshim.o $(WIN32RES)
PGFILEDESC = "pgrx-cshim - C shim to help pgrx"
STATIC_LIB_NAME = lib${MODULE_big}.a


${STATIC_LIB_NAME}: pgrx-cshim.o
	$(AR) crv $@ pgrx-cshim.o

all: ${STATIC_LIB_NAME}

EXTRA_CLEAN += ${STATIC_LIB_NAME}

PG_CONFIG = pg_config
PGXS := $(shell $(PG_CONFIG) --pgxs)
include $(PGXS)
