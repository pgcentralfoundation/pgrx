{ pkgsPath ? <nixpkgs>, crossSystem ? null, channel ? { channel = "stable"; } }:

let
  unstable = import <nixos-unstable> {};
  moz_overlay = import (builtins.fetchTarball https://github.com/mozilla/nixpkgs-mozilla/archive/master.tar.gz) ;
  pkgs = import <nixpkgs> { overlays = [ moz_overlay ]; };


in 
  with pkgs;
  stdenv.mkDerivation {
  name = "rust";
  nativeBuildInputs = [
    unstable.buildPackages.clang
    buildPackages.llvm
    buildPackages.llvmPackages.libclang
    buildPackages.llvmPackages.libcxxStdenv
    buildPackages.postgresql_10
    buildPackages.postgresql_11
    unstable.buildPackages.postgresql_12
    buildPackages.rust-bindgen
    buildPackages.cargo-edit
    buildPackages.openssl
    ((buildPackages.rustChannelOf channel ).rust.override { 
      extensions = [
          "rustfmt-preview"
          "clippy-preview"
	  "rls-preview"
        ];
      })
  ];

  PG10_INCLUDE_PATH="${buildPackages.postgresql_10.out}";
  PG11_INCLUDE_PATH="${buildPackages.postgresql_11.out}";
  PG12_INCLUDE_PATH="${unstable.buildPackages.postgresql_12.out}";
  LIBCLANG_PATH="${buildPackages.llvmPackages.libclang}/lib";
}
