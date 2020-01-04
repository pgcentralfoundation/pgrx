MODULE_big = bridge-c-shim-${PG_TARGET_VERSION}
OBJS = bridge-c-shim.o $(WIN32RES)
PGFILEDESC = "bridge-c-shim - C shim to help pg-rs-bridge"
STATIC_LIB_NAME = lib${MODULE_big}.a

${STATIC_LIB_NAME}: bridge-c-shim.o
	ar crv $@ bridge-c-shim.o

all: ${STATIC_LIB_NAME}

EXTRA_CLEAN += *.a

PG_CONFIG = pg_config
PGXS := $(shell $(PG_CONFIG) --pgxs)
include $(PGXS)
