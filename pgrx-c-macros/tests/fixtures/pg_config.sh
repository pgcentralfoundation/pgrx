#!/bin/sh

case "$1" in
    --version) printf '%s\n' 'PostgreSQL 18.0' ;;
    --includedir-server) printf '%s\n' "$PGRX_C_MACROS_TEST_INCLUDE_DIR" ;;
    --cppflags) printf '%s\n' "${PGRX_C_MACROS_TEST_CPPFLAGS:--DPG_CONFIG_FIXTURE_VALUE=91}" ;;
    *) exit 1 ;;
esac
