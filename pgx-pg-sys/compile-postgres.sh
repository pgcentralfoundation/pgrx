#! /bin/bash

PGVER=$1
TARGETDIR=$2
PORT=$3
PGDIR="${TARGETDIR}/postgresql-${PGVER}"
INSTALLDIR="${PGDIR}/pgx-install"
if [ "x${PGVER}" == "x" ]; then
  echo "Must specify postgres version"
  exit 1;
fi

if [ "x${TARGETDIR}" == "x" ]; then
  echo "Must specify target directory"
  exit 1;
fi

if [ "x${PORT}" == "x" ]; then
  echo "Must specify port number"
  exit 1;
fi

if [ "x${NUM_CPUS}" == "x" ]; then
  NUM_CPUS=1
fi

set -x

cd "${PGDIR}" || exit 1

# configure postgres if not already
if [ ! -f "config.status" ]; then
  ./configure --prefix="${INSTALLDIR}" --with-pgport=${PORT} || exit 1
fi

# compile postgres and install it locally
if [ ! -f "${INSTALLDIR}/bin/postgres" ]; then
  make -j $NUM_CPUS || exit 1
  make install || exit 1

  # exit 2 means we did compile Postgres
  exit 2
fi

# we didn't do anything
exit 0
