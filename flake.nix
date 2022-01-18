{
  description = "Postgres extensions in Rust.";

  inputs = {
    nixpkgs.url = "nixpkgs";
    rust-overlay.url = "github:oxalica/rust-overlay";
    rust-overlay.inputs.nixpkgs.follows = "nixpkgs";
    naersk.url = "github:nix-community/naersk";
    naersk.inputs.nixpkgs.follows = "nixpkgs";
    gitignore.url = "github:hercules-ci/gitignore.nix";
    gitignore.inputs.nixpkgs.follows = "nixpkgs";
  };

  outputs = { self, nixpkgs, rust-overlay, naersk, gitignore }:
    let
      supportedSystems = [ "x86_64-linux" "aarch64-linux" "x86_64-darwin" "aarch64-darwin" ];
      forAllSystems = f: nixpkgs.lib.genAttrs supportedSystems (system: f system);
      supportedPostgresVersions = [ 10 11 12 13 14 ];
      nixpkgsWithOverlays = { system, nixpkgs, extraOverlays ? [ ] }: (import nixpkgs {
        inherit system;
        overlays = [
          self.overlay
          rust-overlay.overlay
          (self: super: { inherit (self.rust-bin.stable.latest) rustc cargo rustdoc; })
        ] ++ extraOverlays;
      });
      releaseAndDebug = attr: call: args: {
        "${attr}" = call args;
        "${attr}_debug" = call (args // { release = false; });
      };
    in
    {
      lib = {
        inherit supportedSystems supportedPostgresVersions forAllSystems nixpkgsWithOverlays;
        buildPgxExtension =
          { pkgs
          , source
          , pgxPostgresVersion
          , additionalFeatures ? [ ]
          , release ? true
          }: pkgs.callPackage ./nix/extension.nix {
            inherit source pgxPostgresVersion release naersk additionalFeatures;
            inherit (gitignore.lib) gitignoreSource;
          };
      };
      defaultPackage = forAllSystems (system: (nixpkgsWithOverlays { inherit system nixpkgs; }).cargo-pgx);

      packages = forAllSystems (system:
        let
          pkgs = nixpkgsWithOverlays { inherit system nixpkgs; };
        in
        {
          inherit (pkgs) cargo-pgx;
        });

      overlay = final: prev: {
        cargo-pgx = final.callPackage ./cargo-pgx {
          inherit naersk;
          gitignoreSource = gitignore.lib.gitignoreSource;
        };
        cargo-pgx_debug = final.callPackage ./cargo-pgx {
          inherit naersk;
          release = false;
          gitignoreSource = gitignore.lib.gitignoreSource;
        };
      };

      devShell = forAllSystems (system:
        let
          pkgs = nixpkgsWithOverlays { inherit system nixpkgs; };
        in
        pkgs.mkShell {
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
        });

      checks = forAllSystems (system:
        let
          pkgs = nixpkgsWithOverlays { inherit system nixpkgs; };
        in
        {
          format = pkgs.runCommand "check-format"
            {
              buildInputs = with pkgs; [ rustfmt cargo ];
            } ''
            ${pkgs.rustfmt}/bin/cargo-fmt fmt --manifest-path ${./.}/Cargo.toml -- --check
            ${pkgs.nixpkgs-fmt}/bin/nixpkgs-fmt --check ${./.}
            touch $out # it worked!
          '';
          pkgs-cargo-pgx = pkgs.cargo-pgx_debug.out;
        });

      defaultTemplate = self.templates.default;
      templates = {
        default = {
          path = ./nix/templates/default;
          description = "A basic PGX extension";
        };
      };
    };
}
