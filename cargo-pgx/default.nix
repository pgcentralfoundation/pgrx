{ buildRustPackage
, lib
, hostPlatform
, fetchFromGitHub
, postgresql_10
, postgresql_11
, postgresql_12
, postgresql_13
, pkg-config
, openssl
, rustToolchain
, rustfmt
, libiconv
, llvmPackages
, gitignoreSource
, release ? true
}:

let
  cargoToml = (builtins.fromTOML (builtins.readFile ./Cargo.toml));
in buildRustPackage {
  inherit release;
  inherit (cargoToml.package) name version;
  cargoSha256 = "sha256-bxzBYrQGA3PmwGh5B2qUaOM9oif9hwk2OR5ZcfDkltY=";
  src = gitignoreSource ../.;

  nativeBuildInputs = [
    rustToolchain
    pkg-config
  ];
  buildInputs = [
    openssl
    libiconv
  ];

  LIBCLANG_PATH = "${llvmPackages.libclang}/lib";

  meta = with lib; {
    description = cargoToml.package.description;
    homepage = cargoToml.package.homepage;
    license = with licenses; [ mit ];
    maintainers = with maintainers; [ hoverbear ];
  };
}
