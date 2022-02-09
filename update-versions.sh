#! /usr/bin/env bash

# requires: https://github.com/sunng87/cargo-release

if [ "x$1" == "x" ]; then
	echo "usage:  ./update-verions.sh <VERSION>"
	exit 1
fi

set -x

HEAD=$(git rev-parse HEAD)
VERSION=$1

cargo release --workspace --no-publish --no-push --no-tag --no-dev-version ${VERSION} || exit 1
git reset --soft ${HEAD} || exit 1 
git reset HEAD || exit 1
sed -i '' -e "s/^pgx = .*$/pgx = \"${VERSION}\"/" ./cargo-pgx/src/templates/cargo_toml || exit 1
sed -i '' -e "s/^pgx-macros = .*$/pgx-macros = \"${VERSION}\"/" ./cargo-pgx/src/templates/cargo_toml || exit 1
sed -i '' -e "s/^pgx-tests = .*$/pgx-tests = \"${VERSION}\"/" ./cargo-pgx/src/templates/cargo_toml || exit 1

sed -i '' -e "s/^pgx = .*$/pgx = \"${VERSION}\"/" ./nix/templates/default/Cargo.toml || exit 1
sed -i '' -e "s/^pgx-macros = .*$/pgx-macros = \"${VERSION}\"/" ./nix/templates/default/Cargo.toml || exit 1
sed -i '' -e "s/^pgx-tests = .*$/pgx-tests = \"${VERSION}\"/" ./nix/templates/default/Cargo.toml || exit 1