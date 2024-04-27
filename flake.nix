{
  inputs = {
    crane = {
      url = "github:ipetkov/crane";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    nixpkgs.url = "nixpkgs/nixos-unstable";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    flake-parts.url = "github:hercules-ci/flake-parts";
    gitignore = {
      url = "github:hercules-ci/gitignore";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = inputs:
    inputs.flake-parts.lib.mkFlake {inherit inputs;} {
      systems = ["x86_64-linux" "aarch64-linux" "aarch64-darwin"];
      perSystem = {
        lib,
        pkgs,
        inputs',
        system,
        ...
      }: let
        rustToolchain = inputs'.fenix.packages.stable.toolchain;
        craneLib = inputs.crane.lib.${system}.overrideToolchain rustToolchain;
      in {
        packages = {
          cargo-pgrx = craneLib.buildPackage {
            src = inputs.gitignore.lib.gitignoreSource ./.;
            inherit (craneLib.crateNameFromCargoToml {cargoToml = ./cargo-pgrx/Cargo.toml;}) pname version;
            cargoExtraArgs = "--package cargo-pgrx";
            nativeBuildInputs = [pkgs.pkg-config];
            buildInputs = [pkgs.openssl] ++ lib.optionals pkgs.stdenv.isDarwin [pkgs.Security];
            # fixes to enable running pgrx tests
            preCheck = ''
              export PGRX_HOME=$(mktemp -d)
            '';
            # skip tests that require pgrx to be initialized using `cargo pgrx init`
            cargoTestExtraArgs = "-- --skip=command::schema::tests::test_parse_managed_postmasters";
          };
        };
      };
    };
}
