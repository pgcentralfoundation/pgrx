{ lib, naersk, hostPlatform, fetchFromGitHub, postgresql_10, postgresql_11, postgresql_12, postgresql_13, pkg-config, openssl, rustfmt, libiconv, llvmPackages, }:

let
  cargoToml = (builtins.fromTOML (builtins.readFile ./Cargo.toml));
in

naersk.lib."${hostPlatform.system}".buildPackage rec {
  name = cargoToml.package.name;
  version = cargoToml.package.version;

  src = ../.;

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
