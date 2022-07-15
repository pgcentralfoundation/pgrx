{ gitignoreSource
, stdenv
, lib
, pkg-config
, openssl
, libiconv
, llvmPackages
, rustPlatform
, darwin
, release ? true
}:

let
  cargoToml = (builtins.fromTOML (builtins.readFile ./Cargo.toml));
in rustPlatform.buildRustPackage {
  inherit release;
  inherit (cargoToml.package) name version;
  cargoSha256 = "sha256-bxzBYrQGA3PmwGh5B2qUaOM9oif9hwk2OR5ZcfDkltY=";
  src = gitignoreSource ../.;
  cargoLock.lockFile = ../Cargo.lock;

  cargoBuildFlags = ["--package" "cargo-pgx"];
  cargoTestFlags = ["--package" "cargo-pgx"];

  nativeBuildInputs = [
    pkg-config
  ];
  buildInputs = [
    openssl
    libiconv
  ] ++ (lib.optional stdenv.isDarwin [ darwin.apple_sdk.frameworks.Security ]);

  LIBCLANG_PATH = "${llvmPackages.libclang}/lib";

  meta = with lib; {
    description = cargoToml.package.description;
    homepage = cargoToml.package.homepage;
    license = with licenses; [ mit ];
    maintainers = with maintainers; [ hoverbear ];
  };
}
