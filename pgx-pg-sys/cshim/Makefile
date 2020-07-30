# Copyright 2020 ZomboDB, LLC <zombodb@gmail.com>. All rights reserved. Use of this source code is
# governed by the MIT license that can be found in the LICENSE file.


MODULE_big = pgx-cshim-${PG_TARGET_VERSION}
OBJS = pgx-cshim.o $(WIN32RES)
PGFILEDESC = "pgx-cshim - C shim to help pgx"
STATIC_LIB_NAME = lib${MODULE_big}.a


${STATIC_LIB_NAME}: pgx-cshim.o
	ar crv $@ pgx-cshim.o

all: ${STATIC_LIB_NAME}

EXTRA_CLEAN += ${STATIC_LIB_NAME}

PG_CONFIG = pg_config
PGXS := $(shell $(PG_CONFIG) --pgxs)
include $(PGXS)
