#! /usr/bin/env bash

PG_VER=$1
DOCKERFILE_ID=$2

echo "Building docker container for PGX using Postgres version $PG_VER in container $DOCKERFILE_ID"

docker build -t pgx -f ".github/docker/Dockerfile.$DOCKERFILE_ID" .

echo "Running PGX test suite using Postgres version $PG_VER in container $DOCKERFILE_ID"

docker run pgx
# docker run \
#   --rm \
#   --volume "$(pwd)":/checkout:rw \
#   --workdir /checkout \
#   --privileged \
#   pgx


#   target=$(echo "${1}" | sed 's/-emulated//')
#     echo "Building docker container for TARGET=${1}"
#     docker build -t stdarch -f "ci/docker/${1}/Dockerfile" ci/
#     mkdir -p target c_programs rust_programs
#     echo "Running docker"
#     # shellcheck disable=SC2016
#     docker run \
#       --rm \
#       --user "$(id -u)":"$(id -g)" \
#       --env CARGO_HOME=/cargo \
#       --env CARGO_TARGET_DIR=/checkout/target \
#       --env TARGET="${target}" \
#       --env STDARCH_TEST_EVERYTHING \
#       --env STDARCH_ASSERT_INSTR_IGNORE \
#       --env STDARCH_DISABLE_ASSERT_INSTR \
#       --env NOSTD \
#       --env NORUN \
#       --env RUSTFLAGS \
#       --env STDARCH_TEST_NORUN \
#       --volume "${HOME}/.cargo":/cargo \
#       --volume "$(rustc --print sysroot)":/rust:ro \
#       --volume "$(pwd)":/checkout:ro \
#       --volume "$(pwd)"/target:/checkout/target \
#       --volume "$(pwd)"/c_programs:/checkout/c_programs \
#       --volume "$(pwd)"/rust_programs:/checkout/rust_programs \
#       --init \
#       --workdir /checkout \
#       --privileged \
#       stdarch \
#       sh -c "HOME=/tmp PATH=\$PATH:/rust/bin exec ci/run.sh ${1}"