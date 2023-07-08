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
