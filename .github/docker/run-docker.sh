#! /usr/bin/env bash

PG_MAJOR_VER=$1
DOCKERFILE_ID=$2

echo "Building docker container for PGX using Postgres version $PG_MAJOR_VER in container $DOCKERFILE_ID"

docker build \
  --build-arg PG_MAJOR_VER=$PG_MAJOR_VER \
  -t pgx -f ".github/docker/Dockerfile.$DOCKERFILE_ID" .

echo "Running PGX test suite using Postgres version $PG_MAJOR_VER in container $DOCKERFILE_ID"

docker run pgx cargo test --no-default-features --features pg$PG_MAJOR_VER --locked
