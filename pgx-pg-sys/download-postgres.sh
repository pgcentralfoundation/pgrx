#! /bin/bash

PGVER=$1
TARGETDIR=$2
PGTARBALL="${TARGETDIR}/${PGVER}.tar.bz2"
if [ "x${PGVER}" == "x" ]; then
  echo "Must specify postgres version"
  exit 1;
fi

if [ "x${TARGETDIR}" == "x" ]; then
  echo "Must specify target directory"
  exit 1;
fi

set -x

# download the specified version of Postgres
if [ ! -f "${PGTARBALL}" ]; then
  wget -O "${PGTARBALL}" "https://ftp.postgresql.org/pub/source/v${PGVER}/postgresql-${PGVER}.tar.bz2" || exit 1
fi

# and untar it into our target directory
if [ ! -f "${TARGETDIR}/postgresql-${PGVER}/Makefile" ]; then
  tar -C "${TARGETDIR}" -xjf "${PGTARBALL}" || exit 1
fi
