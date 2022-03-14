{ lib
, naersk
, stdenv
, clangStdenv
, cargo-pgx
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
  pgxPostgresMajor = builtins.head (lib.splitString "." targetPostgres.version);
  cargoToml = (builtins.fromTOML (builtins.readFile "${source}/Cargo.toml"));
  preBuildAndTest = ''
    mkdir -p $out/.pgx/${pgxPostgresMajor}
    export PGX_HOME=$out/.pgx

    cp -r -L ${targetPostgres}/. $out/.pgx/${pgxPostgresMajor}/
    chmod -R ugo+w $out/.pgx/${pgxPostgresMajor}
    cp -r -L ${targetPostgres.lib}/lib/. $out/.pgx/${pgxPostgresMajor}/lib/

    ${cargo-pgx}/bin/cargo-pgx pgx init \
      --pg${pgxPostgresMajor} $out/.pgx/${pgxPostgresMajor}/bin/pg_config \

    # This is primarily for Mac or other Nix systems that don't use the nixbld user.
    export USER=$(whoami)
    export PGDATA=$out/.pgx/data-${pgxPostgresMajor}/
    export NIX_PGLIBDIR=$out/.pgx/${pgxPostgresMajor}/lib

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
  name = "${cargoToml.package.name}-pg${pgxPostgresMajor}";
  version = cargoToml.package.version;

  src = gitignoreSource source;

  inputsFrom = [ targetPostgres cargo-pgx ];

  LIBCLANG_PATH = "${llvmPackages.libclang.lib}/lib";
  buildInputs = [
    rust-bin.stable.latest.default
    cargo-pgx
    pkg-config
    libiconv
    targetPostgres
  ];
  checkInputs = [
    cargo-pgx
    rust-bin.stable.latest.default
  ];

  postPatch = "patchShebangs .";
  preBuild = preBuildAndTest;
  preCheck = preBuildAndTest;
  postBuild = ''
    if [ -f "${cargoToml.package.name}.control" ]; then
      export PGX_HOME=$out/.pgx
      export NIX_PGLIBDIR=${targetPostgres.out}/share/postgresql/extension/
      ${cargo-pgx}/bin/cargo-pgx pgx package --pg-config ${targetPostgres}/bin/pg_config ${maybeDebugFlag} --features "${builtins.toString additionalFeatures}" --out-dir $out
      export NIX_PGLIBDIR=$out/.pgx/${pgxPostgresMajor}/lib
    fi
  '';
  # Certain extremely slow machines (Github actions...) don't clean up their socket properly.
  preFixup = ''
    if [ -f "${cargoToml.package.name}.control" ]; then
      ${cargo-pgx}/bin/cargo-pgx pgx stop all

      mv -v $out/${targetPostgres.out}/* $out
      rm -rfv $out/nix

      until [ ! -f $out/.pgx/.s.PGSQL.* ] do;
        sleep 3;
        echo "Waiting for PostgreSQL socket to close..."
      done
      rm -rfv $out/.pgx
    fi
  '';

  PGX_PG_SYS_SKIP_BINDING_REWRITE = "1";
  CARGO_BUILD_INCREMENTAL = "false";
  RUST_BACKTRACE = "full";

  cargoBuildOptions = default: default ++ [ "--no-default-features" "--features \"pg${pgxPostgresMajor} ${builtins.toString additionalFeatures}\"" ];
  cargoTestOptions = default: default ++ [ "--no-default-features" "--features \"pg_test pg${pgxPostgresMajor} ${builtins.toString additionalFeatures}\"" ];
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
