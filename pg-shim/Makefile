MODULE_big = pg-shim-${PG_TARGET_VERSION}
OBJS = pg-shim.o $(WIN32RES)
PGFILEDESC = "pg-shim - C shim to help pg-rs-bridge"
STATIC_LIB_NAME = lib${MODULE_big}.a

${STATIC_LIB_NAME}: pg-shim.o
	ar crv $@ pg-shim.o

all: ${STATIC_LIB_NAME}

EXTRA_CLEAN += *.a

PG_CONFIG = pg_config
PGXS := $(shell $(PG_CONFIG) --pgxs)
include $(PGXS)
