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
    naersk = {
      url = "github:nix-community/naersk";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay, gitignore, naersk, ... }:
    let
      # Helper function for producing a package-specific nixpkgs
      nixpkgsWithOverlays = { nixpkgs, system, extraOverlays ? [ ] }: (import nixpkgs {
        inherit system;
        overlays = [
          self.overlays.default
          (import rust-overlay)
          (self: super: { inherit (self.rust-bin.stable.latest) rustc cargo rustdoc rust-std; })
        ] ++ extraOverlays;
      });

      # Inheritance helpers
      inherit (gitignore.lib) gitignoreSource;
    in flake-utils.lib.eachDefaultSystem (system:
      {
        packages = let
          pkgs = nixpkgsWithOverlays { inherit nixpkgs system; };
        in rec {
          default = cargo-pgx;
          inherit (pkgs) cargo-pgx;
        };

        devShells.default = let
          pkgs = nixpkgsWithOverlays { inherit nixpkgs system; };
        in pkgs.mkShell {
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
            postgresql
            libiconv
            pkg-config
          ] ++ (with pkgs.rust-bin.stable.latest; [
            clippy
            minimal
            rustfmt
          ]);
          LIBCLANG_PATH = "${pkgs.llvmPackages.libclang.lib}/lib";
          PGX_PG_SYS_SKIP_BINDING_REWRITE = "1";
          BINDGEN_EXTRA_CLANG_ARGS = [
            ''-I"${pkgs.llvmPackages.libclang.lib}/lib/clang/${pkgs.llvmPackages.libclang.version}/include"''
          ] ++ (if pkgs.stdenv.isLinux then [
            "-I ${pkgs.glibc.dev}/include"
          ] else [ ]);
        };

        checks = let
          pkgs = nixpkgsWithOverlays { inherit system nixpkgs; };
        in {
          format = pkgs.runCommand "check-format"
            {
              buildInputs = with pkgs; [ rustfmt cargo ];
            } ''
            ${pkgs.rustfmt}/bin/cargo-fmt fmt --manifest-path ${./.}/Cargo.toml -- --check
            ${pkgs.nixpkgs-fmt}/bin/nixpkgs-fmt --check ${./.}
            touch $out # it worked!
          '';
          pkgs-cargo-pgx = pkgs.cargo-pgx_debug.out;
        };
      }
    ) // {
      lib = let
        supportedSystems = [ "x86_64-linux" "aarch64-linux" "x86_64-darwin" "aarch64-darwin" ];
        forAllSystems = f: nixpkgs.lib.genAttrs supportedSystems (system: f system);
      in {
        inherit supportedSystems forAllSystems nixpkgsWithOverlays;
        buildPgxExtension =
          { pkgs
          , source
          , targetPostgres
          , additionalFeatures ? [ ]
          , release ? true
          }: pkgs.callPackage ./nix/extension.nix {
            inherit source targetPostgres release naersk additionalFeatures gitignoreSource;
          };
      };

      overlays.default = final: prev: {
        cargo-pgx = final.callPackage ./cargo-pgx {
          inherit gitignoreSource;
        };
        cargo-pgx_debug = final.callPackage ./cargo-pgx {
          inherit gitignoreSource;
          release = false;
        };
      };

      templates = {
        default = {
          path = ./nix/templates/default;
          description = "A basic PGX extension";
        };
      };
    };
}
