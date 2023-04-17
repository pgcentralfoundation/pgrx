{ lib
, naersk
, stdenv
, clangStdenv
, cargo-pgrx
, hostPlatform
, targetPlatform
, pkg-config
, openssl
, libiconv
, rust-bin
, llvmPackages
, gitignoreSource
, runCommand
, targetPostgres
, release ? true
, source ? ./.
, additionalFeatures
, doCheck ? true
}:

let
  maybeReleaseFlag = if release == true then "--release" else "";
  maybeDebugFlag = if release == true then "" else "--debug";
  pgrxPostgresMajor = builtins.head (lib.splitString "." targetPostgres.version);
  cargoToml = (builtins.fromTOML (builtins.readFile "${source}/Cargo.toml"));
  preBuildAndTest = ''
    export PGRX_HOME=$(mktemp -d)
    mkdir -p $PGRX_HOME/${pgrxPostgresMajor}

    cp -r -L ${targetPostgres}/. $PGRX_HOME/${pgrxPostgresMajor}/
    chmod -R ugo+w $PGRX_HOME/${pgrxPostgresMajor}
    cp -r -L ${targetPostgres.lib}/lib/. $PGRX_HOME/${pgrxPostgresMajor}/lib/

    ${cargo-pgrx}/bin/cargo-pgrx pgrx init \
      --pg${pgrxPostgresMajor} $PGRX_HOME/${pgrxPostgresMajor}/bin/pg_config \

    # This is primarily for Mac or other Nix systems that don't use the nixbld user.
    export USER=$(whoami)
    export PGDATA=$PGRX_HOME/data-${pgrxPostgresMajor}/
    export NIX_PGLIBDIR=$PGRX_HOME/${pgrxPostgresMajor}/lib

    echo "unix_socket_directories = '$(mktemp -d)'" > $PGDATA/postgresql.conf 
    ${targetPostgres}/bin/pg_ctl start
    ${targetPostgres}/bin/createuser -h localhost --superuser --createdb $USER || true
    ${targetPostgres}/bin/pg_ctl stop

    # Set C flags for Rust's bindgen program. Unlike ordinary C
    # compilation, bindgen does not invoke $CC directly. Instead it
    # uses LLVM's libclang. To make sure all necessary flags are
    # included we need to look in a few places.
    # TODO: generalize this process for other use-cases.
    export BINDGEN_EXTRA_CLANG_ARGS="$(< ${stdenv.cc}/nix-support/libc-crt1-cflags) \
      $(< ${stdenv.cc}/nix-support/libc-cflags) \
      $(< ${stdenv.cc}/nix-support/cc-cflags) \
      $(< ${stdenv.cc}/nix-support/libcxx-cxxflags) \
      ${lib.optionalString stdenv.cc.isClang "-idirafter ${stdenv.cc.cc}/lib/clang/${lib.getVersion stdenv.cc.cc}/include"} \
      ${lib.optionalString stdenv.cc.isGNU "-isystem ${stdenv.cc.cc}/include/c++/${lib.getVersion stdenv.cc.cc} -isystem ${stdenv.cc.cc}/include/c++/${lib.getVersion stdenv.cc.cc}/${stdenv.hostPlatform.config} -idirafter ${stdenv.cc.cc}/lib/gcc/${stdenv.hostPlatform.config}/${lib.getVersion stdenv.cc.cc}/include"}
    "
  '';
in

naersk.lib."${targetPlatform.system}".buildPackage rec {
  inherit release doCheck;
  name = "${cargoToml.package.name}-pg${pgrxPostgresMajor}";
  version = cargoToml.package.version;

  src = gitignoreSource source;

  inputsFrom = [ targetPostgres cargo-pgrx ];

  LIBCLANG_PATH = "${llvmPackages.libclang.lib}/lib";
  buildInputs = [
    rust-bin.stable.latest.default
    cargo-pgrx
    pkg-config
    libiconv
    targetPostgres
  ];
  checkInputs = [
    cargo-pgrx
    rust-bin.stable.latest.default
  ];

  postPatch = "patchShebangs .";
  preBuild = preBuildAndTest;
  preCheck = preBuildAndTest;
  postBuild = ''
    if [ -f "${cargoToml.package.name}.control" ]; then
      export NIX_PGLIBDIR=${targetPostgres.out}/share/postgresql/extension/
      ${cargo-pgrx}/bin/cargo-pgrx pgrx package --pg-config ${targetPostgres}/bin/pg_config ${maybeDebugFlag} --features "${builtins.toString additionalFeatures}" --out-dir $out
      export NIX_PGLIBDIR=$PGRX_HOME/${pgrxPostgresMajor}/lib
    fi
  '';
  # Certain extremely slow machines (Github actions...) don't clean up their socket properly.
  preFixup = ''
    if [ -f "${cargoToml.package.name}.control" ]; then
      ${cargo-pgrx}/bin/cargo-pgrx pgrx stop all

      mv -v $out/${targetPostgres.out}/* $out
      rm -rfv $out/nix
    fi
  '';

  PGRX_PG_SYS_SKIP_BINDING_REWRITE = "1";
  CARGO_BUILD_INCREMENTAL = "false";
  RUST_BACKTRACE = "full";

  cargoBuildOptions = default: default ++ [ "--no-default-features" "--features \"pg${pgrxPostgresMajor} ${builtins.toString additionalFeatures}\"" ];
  cargoTestOptions = default: default ++ [ "--no-default-features" "--features \"pg_test pg${pgrxPostgresMajor} ${builtins.toString additionalFeatures}\"" ];
  doDoc = false;
  copyLibs = false;
  copyBins = false;

  meta = with lib; {
    description = cargoToml.package.description;
    homepage = cargoToml.package.homepage;
    license = with licenses; [ mit ];
    maintainers = with maintainers; [ hoverbear ];
  };
}
