{
  description = "Postgres extensions in Rust.";

  inputs = {
    nixpkgs.url = "nixpkgs";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    gitignore = {
      url = "github:hercules-ci/gitignore.nix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay, gitignore, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        inherit (pkgs) callPackage;
        inherit (pkgs.rustPlatform) buildRustPackage;
        inherit (gitignore.lib) gitignoreSource;

        overlays = [
          (import rust-overlay)
        ];

        pkgs = import nixpkgs { inherit overlays system; };
        
        rustToolchain = pkgs.rust-bin.stable.latest.default;

        cargo-pgx = callPackage ./cargo-pgx {
          inherit buildRustPackage gitignoreSource rustToolchain;
        };

        cargo-pgx_debug = callPackage ./cargo-pgx {
          release = false;
          inherit buildRustPackage gitignoreSource rustToolchain;
        };
      in {
        packages = {
          default = cargo-pgx;

          inherit cargo-pgx cargo-pgx_debug;
        };

        devShells.default = pkgs.mkShell {
          inputsFrom = with pkgs; [
            postgresql_10
            postgresql_11
            postgresql_12
            postgresql_13
            postgresql_14
          ];
          buildInputs = with pkgs; [
            rustfmt
            nixpkgs-fmt
            cargo-pgx
            rust-bin.stable.latest.minimal
            rust-bin.stable.latest.rustfmt
            rust-bin.stable.latest.clippy
            postgresql
            libiconv
            pkg-config
          ];
          LIBCLANG_PATH = "${pkgs.llvmPackages.libclang.lib}/lib";
          PGX_PG_SYS_SKIP_BINDING_REWRITE = "1";
          BINDGEN_EXTRA_CLANG_ARGS = [
            ''-I"${pkgs.llvmPackages.libclang.lib}/lib/clang/${pkgs.llvmPackages.libclang.version}/include"''
          ] ++ (if pkgs.stdenv.isLinux then [
            "-I ${pkgs.glibc.dev}/include"
          ] else [ ]);
        };

        checks = {
          format = pkgs.runCommand "check-format"
            {
              buildInputs = with pkgs; [ cargo rustfmt ];
            } ''
            ${pkgs.rustfmt}/bin/cargo-fmt fmt --manifest-path ${./.}/Cargo.toml -- --check
            ${pkgs.nixpkgs-fmt}/bin/nixpkgs-fmt --check ${./.}
            touch $out # it worked!
          '';
          pkgs-cargo-pgx = cargo-pgx_debug.out;
        };
      }
    ) // {
      templates = {
        default = {
          path = ./nix/templates/default;
          description = "A basic PGX extension";
        };
      };
    };
}
