CARGO_TARGET_DIR="$(pwd)/target" cargo test --manifest-path=pgrx-examples/"${1}"/Cargo.toml  --features "pg${PG_VER:-14}" --no-default-features
