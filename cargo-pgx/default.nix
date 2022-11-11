{ lib
, naersk
, hostPlatform
, postgresql_11
, postgresql_12
, postgresql_13
, pkg-config
, openssl
, rustfmt
, libiconv
, llvmPackages
, gitignoreSource
, release ? true
, callPackage
, fenixToolchain
}:

let
  cargoToml = (builtins.fromTOML (builtins.readFile ./Cargo.toml));
  naerskLib = callPackage naersk {
    cargo = fenixToolchain hostPlatform.system;
    rustc = fenixToolchain hostPlatform.system;
  };
in

naerskLib.buildPackage rec {
  name = cargoToml.package.name;
  version = cargoToml.package.version;

  src = gitignoreSource ../.;
  inherit release;

  cargoBuildOptions = final: final ++ [ "--package" "cargo-pgx" ];
  cargoTestOptions = final: final ++ [ "--package" "cargo-pgx" ];

  nativeBuildInputs = [
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
