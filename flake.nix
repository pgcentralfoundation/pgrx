{
  description = "Postgres extensions in Rust.";

  inputs = {
    nixpkgs.url = "nixpkgs";
    fenix.url = "github:nix-community/fenix";
    fenix.inputs.nixpkgs.follows = "nixpkgs";
    naersk.url = "github:nix-community/naersk";
    naersk.inputs.nixpkgs.follows = "nixpkgs";
    gitignore.url = "github:hercules-ci/gitignore.nix";
    gitignore.inputs.nixpkgs.follows = "nixpkgs";
  };

  outputs = { self, nixpkgs, fenix, naersk, gitignore }:
    let
      supportedSystems = [ "x86_64-linux" "aarch64-linux" "x86_64-darwin" "aarch64-darwin" ];
      forAllSystems = f: nixpkgs.lib.genAttrs supportedSystems (system: f system);
      nixpkgsWithOverlays = { system, nixpkgs, extraOverlays ? [ ] }: (import nixpkgs {
        inherit system;
        overlays = [
          self.overlay
          fenix.overlays.default
        ] ++ extraOverlays;
      });
      releaseAndDebug = attr: call: args: {
        "${attr}" = call args;
        "${attr}_debug" = call (args // { release = false; });
      };
      fenixToolchain = system: with fenix.packages.${system};
        combine ([
          stable.clippy
          stable.rustc
          stable.cargo
          stable.rustfmt
          stable.rust-src
        ]);
    in
    {
      lib = {
        inherit supportedSystems forAllSystems nixpkgsWithOverlays;
        buildPgrxExtension =
          { pkgs
          , source
          , targetPostgres
          , additionalFeatures ? [ ]
          , release ? true
          }: pkgs.callPackage ./nix/extension.nix {
            inherit source targetPostgres release naersk additionalFeatures;
            inherit (gitignore.lib) gitignoreSource;
          };
      };
      defaultPackage = forAllSystems (system: (nixpkgsWithOverlays { inherit system nixpkgs; }).cargo-pgrx);

      packages = forAllSystems (system:
        let
          pkgs = nixpkgsWithOverlays { inherit system nixpkgs; };
        in
        {
          inherit (pkgs) cargo-pgrx;
        });

      overlay = final: prev: {
        cargo-pgrx = final.callPackage ./cargo-pgrx {
          inherit fenixToolchain;
          inherit naersk;
          gitignoreSource = gitignore.lib.gitignoreSource;
        };
        cargo-pgrx_debug = final.callPackage ./cargo-pgrx {
          inherit fenixToolchain;
          inherit naersk;
          release = false;
          gitignoreSource = gitignore.lib.gitignoreSource;
        };
      };

      devShell = forAllSystems (system:
        let
          rust-toolchain = fenixToolchain system;
          pkgs = nixpkgsWithOverlays { inherit system nixpkgs; };
        in
        pkgs.mkShell {
          inputsFrom = with pkgs; [
            postgresql_12
            postgresql_13
            postgresql_14
            postgresql_15
            postgresql_16
          ];
          buildInputs = with pkgs; [
            rustfmt
            nixpkgs-fmt
            cargo-pgrx
            rust-toolchain
            postgresql
            libiconv
            pkg-config
          ];
          LIBCLANG_PATH = "${pkgs.llvmPackages.libclang.lib}/lib";
          PGRX_PG_SYS_SKIP_BINDING_REWRITE = "1";
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
          pkgs-cargo-pgrx = pkgs.cargo-pgrx_debug.out;
        });

      defaultTemplate = self.templates.default;
      templates = {
        default = {
          path = ./nix/templates/default;
          description = "A basic PGRX extension";
        };
      };
    };
}
