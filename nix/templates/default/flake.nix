{
  description = "A PostgreSQL extension built by pgx.";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    naersk.url = "github:nmattia/naersk";
    pgx.url = "github:hoverbear/pgx/develop";
    pgx.inputs.nixpkgs.follows = "nixpkgs";
  };

  outputs = { self, nixpkgs, pgx, naersk }:
    let
      cargoToml = (builtins.fromTOML (builtins.readFile ./Cargo.toml));
      supportedSystems = [ "x86_64-linux" "aarch64-linux" "x86_64-darwin" ];
      forAllSystems = f: nixpkgs.lib.genAttrs supportedSystems (system: f system);
    in
    {
      inherit (pgx) devShell;

      defaultPackage = forAllSystems (system: (import nixpkgs {
        inherit system;
        overlays = [ pgx.overlay self.overlay ];
      })."${cargoToml.package.name}");

      packages = forAllSystems (system:
        let
          pkgs = import nixpkgs {
            inherit system;
            overlays = [ pgx.overlay self.overlay ];
          };
        in
        {
          "${cargoToml.package.name}" = pkgs."${cargoToml.package.name}";
          "${cargoToml.package.name}_10" = pkgs."${cargoToml.package.name}";
          "${cargoToml.package.name}_11" = pkgs."${cargoToml.package.name}";
          "${cargoToml.package.name}_12" = pkgs."${cargoToml.package.name}";
          "${cargoToml.package.name}_13" = pkgs."${cargoToml.package.name}";

          "${cargoToml.package.name}_all" = pkgs.runCommandNoCC "allVersions" { } ''
            mkdir -p $out
            cp -r ${pkgs."${cargoToml.package.name}_10"} $out/${cargoToml.package.name}_10
            cp -r ${pkgs."${cargoToml.package.name}_11"} $out/${cargoToml.package.name}_11
            cp -r ${pkgs."${cargoToml.package.name}_12"} $out/${cargoToml.package.name}_12
            cp -r ${pkgs."${cargoToml.package.name}_13"} $out/${cargoToml.package.name}_13
          '';
        });

      overlay = final: prev: {
        "${cargoToml.package.name}" = final.callPackage ./. { inherit naersk; };
        "${cargoToml.package.name}_10" = final.callPackage ./. { pgxPostgresVersion = 10; inherit naersk; };
        "${cargoToml.package.name}_11" = final.callPackage ./. { pgxPostgresVersion = 11; inherit naersk; };
        "${cargoToml.package.name}_12" = final.callPackage ./. { pgxPostgresVersion = 12; inherit naersk; };
        "${cargoToml.package.name}_13" = final.callPackage ./. { pgxPostgresVersion = 13; inherit naersk; };
      };

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
            overlays = [ pgx.overlay self.overlay ];
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
          "${cargoToml.package.name}" = pkgs."${cargoToml.package.name}";
          "${cargoToml.package.name}_10" = pkgs."${cargoToml.package.name}_10";
          "${cargoToml.package.name}_11" = pkgs."${cargoToml.package.name}_11";
          "${cargoToml.package.name}_12" = pkgs."${cargoToml.package.name}_12";
          "${cargoToml.package.name}_13" = pkgs."${cargoToml.package.name}_13";
        });
    };
}
