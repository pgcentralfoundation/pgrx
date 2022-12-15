#! /usr/bin/env bash

# Examples of running this script in CI (currently Github Actions):
#   ./.github/docker/run-docker.sh 14 debian:bullseye
#   ./.github/docker/run-docker.sh 12 fedora:36

PG_MAJOR_VER=$1
DOCKERFILE_ID=$2

echo "Building docker container for PGX using Postgres version $PG_MAJOR_VER in container $DOCKERFILE_ID"
echo "Cargo lock flag set to: $CARGO_LOCKED_OPTION"

docker build \
  --build-arg PG_MAJOR_VER=$PG_MAJOR_VER \
  --build-arg CARGO_LOCKED_OPTION="$CARGO_LOCKED_OPTION" \
  -t pgx \
  -f ".github/docker/Dockerfile.$DOCKERFILE_ID" \
  .

echo "Running PGX test suite using Postgres version $PG_MAJOR_VER in container $DOCKERFILE_ID"

docker run pgx \
  cargo test \
  --no-default-features \
  --features pg$PG_MAJOR_VER \
  "$CARGO_LOCKED_OPTION"
