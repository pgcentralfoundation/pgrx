{
  description = "A PostgreSQL extension built by pgrx.";

  inputs = {
    nixpkgs.url = "nixpkgs";
    rust-overlay.url = "github:oxalica/rust-overlay";
    rust-overlay.inputs.nixpkgs.follows = "nixpkgs";
    naersk.url = "github:nix-community/naersk";
    naersk.inputs.nixpkgs.follows = "nixpkgs";
    gitignore.url = "github:hercules-ci/gitignore.nix";
    gitignore.inputs.nixpkgs.follows = "nixpkgs";
    pgrx.url = "github:tcdi/pgrx";
    pgrx.inputs.nixpkgs.follows = "nixpkgs";
    pgrx.inputs.naersk.follows = "naersk";
  };

  outputs = { self, nixpkgs, rust-overlay, naersk, gitignore, pgrx }:
    let
      cargoToml = (builtins.fromTOML (builtins.readFile ./Cargo.toml));
    in
    {
      inherit (pgrx) devShell;

      defaultPackage = pgrx.lib.forAllSystems (system:
        let
          pkgs = pgrx.lib.nixpkgsWithOverlays { inherit system nixpkgs; extraOverlays = [ self.overlay ]; };
        in
        pkgs."${cargoToml.package.name}");

      packages = pgrx.lib.forAllSystems (system:
        let
          pkgs = pgrx.lib.nixpkgsWithOverlays { inherit system nixpkgs; extraOverlays = [ self.overlay ]; };
        in
        (nixpkgs.lib.foldl'
          (x: y: x // y)
          { }
          (map
            (version:
              let versionString = builtins.toString version; in
              {
                "${cargoToml.package.name}_${versionString}" = pkgs."${cargoToml.package.name}_${versionString}";
                "${cargoToml.package.name}_${versionString}_debug" = pkgs."${cargoToml.package.name}_${versionString}_debug";
              })
            pgrx.lib.supportedPostgresVersions)
        ));

      overlay = final: prev: {
        "${cargoToml.package.name}" = pgrx.lib.buildPgrxExtension {
          pkgs = final;
          source = ./.;
          pgrxPostgresVersion = 11;
        };
        "${cargoToml.package.name}_debug" = pgrx.lib.buildPgrxExtension {
          pkgs = final;
          source = ./.;
          pgrxPostgresVersion = 11;
          release = false;
        };
      } // (nixpkgs.lib.foldl'
        (x: y: x // y)
        { }
        (map
          (version:
            let versionString = builtins.toString version; in
            {
              "${cargoToml.package.name}_${versionString}" = pgrx.lib.buildPgrxExtension {
                pkgs = final;
                source = ./.;
                pgrxPostgresVersion = version;
              };
              "${cargoToml.package.name}_${versionString}_debug" = pgrx.lib.buildPgrxExtension {
                pkgs = final;
                source = ./.;
                pgrxPostgresVersion = version;
                release = false;
              };
            })
          pgrx.lib.supportedPostgresVersions)
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
            nixpkgs.overlays = [ self.overlay pgrx.overlay ];
            services.postgresql.extraPlugins = with pkgs; [
              "${cargoToml.package.name}"
            ];
          };
        };

      checks = pgrx.lib.forAllSystems (system:
        let
          pkgs = pgrx.lib.nixpkgsWithOverlays { inherit system nixpkgs; extraOverlays = [ self.overlay ]; };
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
          "${cargoToml.package.name}_11_debug" = pkgs."${cargoToml.package.name}_11_debug";
          "${cargoToml.package.name}_12_debug" = pkgs."${cargoToml.package.name}_12_debug";
          "${cargoToml.package.name}_13_debug" = pkgs."${cargoToml.package.name}_13_debug";
          "${cargoToml.package.name}_14_debug" = pkgs."${cargoToml.package.name}_14_debug";
        });
    };
}
