echo "---- setup rustc ----"
# setting default to a release channel syncs to that
rustup default stable

# only needed for cross-compile tests but we want consistent rust configuration
rustup target add aarch64-unknown-linux-gnu

# output full rustc version data
rustc --version --verbose
# that this determines what our cargo version is, so don't ask
