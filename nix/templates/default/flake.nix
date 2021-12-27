{
  description = "A PostgreSQL extension built by pgx.";

  inputs = {
    nixpkgs.url = "nixpkgs";
    rust-overlay.url = "github:oxalica/rust-overlay";
    rust-overlay.inputs.nixpkgs.follows = "nixpkgs";
    naersk.url = "github:nix-community/naersk";
    naersk.inputs.nixpkgs.follows = "nixpkgs";
    gitignore.url = "github:hercules-ci/gitignore.nix";
    gitignore.inputs.nixpkgs.follows = "nixpkgs";
    pgx.url = "github:zombodb/pgx/develop";
    pgx.inputs.nixpkgs.follows = "nixpkgs";
    pgx.inputs.naersk.follows = "naersk";
  };

  outputs = { self, nixpkgs, rust-overlay, naersk, gitignore, pgx }:
    let
      cargoToml = (builtins.fromTOML (builtins.readFile ./Cargo.toml));
      supportedPostgresVersions = [ 10 11 12 13 14 ];
    in
    {
      inherit (pgx) devShell;

      defaultPackage = pgx.lib.forAllSystems (system: (import nixpkgs {
        inherit system;
        overlays = [ pgx.overlay self.overlay ];
      })."${cargoToml.package.name}");

      packages = pgx.lib.forAllSystems (system:
        let
          pkgs = nixpkgsWithOverlays system nixpkgs;
        in
        {
          "${cargoToml.package.name}" = pkgs."${cargoToml.package.name}";
          "${cargoToml.package.name}_debug" = pkgs."${cargoToml.package.name}_debug";
          "${cargoToml.package.name}_all" = pkgs.runCommandNoCC "allVersions" { } ''
            mkdir -p $out
            cp -r ${pkgs."${cargoToml.package.name}_10"} $out/${cargoToml.package.name}_10
            cp -r ${pkgs."${cargoToml.package.name}_11"} $out/${cargoToml.package.name}_11
            cp -r ${pkgs."${cargoToml.package.name}_12"} $out/${cargoToml.package.name}_12
            cp -r ${pkgs."${cargoToml.package.name}_13"} $out/${cargoToml.package.name}_13
            cp -r ${pkgs."${cargoToml.package.name}_14"} $out/${cargoToml.package.name}_14
          '';
          "${cargoToml.package.name}_all_debug" = pkgs.runCommandNoCC "allVersions" { } ''
            mkdir -p $out
            cp -r ${pkgs."${cargoToml.package.name}_10_debug"} $out/${cargoToml.package.name}_10
            cp -r ${pkgs."${cargoToml.package.name}_11_debug"} $out/${cargoToml.package.name}_11
            cp -r ${pkgs."${cargoToml.package.name}_12_debug"} $out/${cargoToml.package.name}_12
            cp -r ${pkgs."${cargoToml.package.name}_13_debug"} $out/${cargoToml.package.name}_13
            cp -r ${pkgs."${cargoToml.package.name}_14_debug"} $out/${cargoToml.package.name}_14
          '';
        } // (nixpkgs.lib.foldl'
          (x: y: x // y)
          { }
          (map
            (version:
              let versionString = builtins.toString version; in
              {
                "${cargoToml.package.name}_${versionString}" = pkgs."${cargoToml.package.name}_${versionString}";
                "${cargoToml.package.name}_${versionString}_debug" = pkgs."${cargoToml.package.name}_${versionString}_debug";
              })
            supportedPostgresVersions)
        ));

      overlay = final: prev: {
        "${cargoToml.package.name}" = final.callPackage ./. {
          inherit naersk;
          gitignoreSource = gitignore.lib.gitignoreSource;
        };
        "${cargoToml.package.name}_debug" = final.callPackage ./. {
          inherit naersk;
          release = false;
          gitignoreSource = gitignore.lib.gitignoreSource;
        };
      } // (nixpkgs.lib.foldl'
        (x: y: x // y)
        { }
        (map
          (version:
            let versionString = builtins.toString version; in
            {
              "${cargoToml.package.name}_${versionString}" = final.callPackage ./. {
                inherit naersk;
                pgxPostgresVersion = version;
                gitignoreSource = gitignore.lib.gitignoreSource;
              };
              "${cargoToml.package.name}_${versionString}_debug" = final.callPackage ./. {
                inherit naersk;
                release = false;
                pgxPostgresVersion = version;
                gitignoreSource = gitignore.lib.gitignoreSource;
              };
            })
          supportedPostgresVersions)
      );

      nixosModule = { config, pkgs, lib, ... }:
        let
          cfg = config.services.postgresql."${cargoToml.package.name}";
        in
        with lib;
        {
          options = {
            services.postgresql."${cargoToml.package.name}".enable = mkEnableOption "Enable ${cargoToml.package.name}.";
          };
          config = mkIf cfg.enable {
            nixpkgs.overlays = [ self.overlay pgx.overlay ];
            services.postgresql.extraPlugins = with pkgs; [
              "${cargoToml.package.name}"
            ];
          };
        };

      checks = forAllSystems (system:
        let
          pkgs = import nixpkgs {
            inherit system;
            overlays = [
              self.overlay
              pgx.overlay
              rust-overlay.overlay
              (self: super:
                {
                  rustc = self.rust-bin.stable.latest.rustc;
                  cargo = self.rust-bin.stable.latest.cargo;
                }
              )
            ];
          };
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
          # audit = pkgs.runCommand "audit" { } ''
          #   HOME=$out
          #   ${pkgs.cargo-audit}/bin/cargo-audit audit --no-fetch
          #   # it worked!
          # '';
          "${cargoToml.package.name}_debug" = pkgs."${cargoToml.package.name}_debug";
          "${cargoToml.package.name}_10_debug" = pkgs."${cargoToml.package.name}_10_debug";
          "${cargoToml.package.name}_11_debug" = pkgs."${cargoToml.package.name}_11_debug";
          "${cargoToml.package.name}_12_debug" = pkgs."${cargoToml.package.name}_12_debug";
          "${cargoToml.package.name}_13_debug" = pkgs."${cargoToml.package.name}_13_debug";
          "${cargoToml.package.name}_14_debug" = pkgs."${cargoToml.package.name}_14_debug";
        });
    };
}
