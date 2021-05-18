{ lib
, naersk
, stdenv
, clangStdenv
, cargo-pgx
, hostPlatform
, targetPlatform
, postgresql
, postgresql_10
, postgresql_11
, postgresql_12
, postgresql_13
, pkg-config
, openssl
, libiconv
, rustfmt
, cargo
, rustc
, llvmPackages
, pgxPostgresVersion ? 11
}:

let
  pgxPostgresPkg =
    if (pgxPostgresVersion == 10) then postgresql_10
    else if (pgxPostgresVersion == 11) then postgresql_11
    else if (pgxPostgresVersion == 12) then postgresql_12
    else if (pgxPostgresVersion == 13) then postgresql_13
    else null;
  pgxPostgresVersionString = builtins.toString pgxPostgresVersion;
  cargoToml = (builtins.fromTOML (builtins.readFile ./Cargo.toml));
in

naersk.lib."${targetPlatform.system}".buildPackage rec {
  name = "${cargoToml.package.name}-pg${pgxPostgresVersionString}";
  version = cargoToml.package.version;

  src = ./.;

  inputsFrom = [ postgresql_10 postgresql_11 postgresql_12 postgresql_13 cargo-pgx ];

  LIBCLANG_PATH = "${llvmPackages.libclang}/lib";
  buildInputs = [
    rustfmt
    cargo-pgx
    pkg-config
    cargo
    rustc
    libiconv
  ];
  checkInputs = [ cargo-pgx cargo rustc ];
  doCheck = true;

  preConfigure = ''
    mkdir -p $out/.pgx/{10,11,12,13}
    export PGX_HOME=$out/.pgx

    cp -r -L ${postgresql_10}/. $out/.pgx/10/
    chmod -R ugo+w $out/.pgx/10
    cp -r -L ${postgresql_10.lib}/lib/. $out/.pgx/10/lib/
    cp -r -L ${postgresql_11}/. $out/.pgx/11/
    chmod -R ugo+w $out/.pgx/11
    cp -r -L ${postgresql_11.lib}/lib/. $out/.pgx/11/lib/
    cp -r -L ${postgresql_12}/. $out/.pgx/12/
    chmod -R ugo+w $out/.pgx/12
    cp -r -L ${postgresql_12.lib}/lib/. $out/.pgx/12/lib/
    cp -r -L ${postgresql_13}/. $out/.pgx/13/
    chmod -R ugo+w $out/.pgx/13
    cp -r -L ${postgresql_13.lib}/lib/. $out/.pgx/13/lib/

    ${cargo-pgx}/bin/cargo-pgx pgx init \
      --pg10 $out/.pgx/10/bin/pg_config \
      --pg11 $out/.pgx/11/bin/pg_config \
      --pg12 $out/.pgx/12/bin/pg_config \
      --pg13 $out/.pgx/13/bin/pg_config
    
    # This is primarily for Mac or other Nix systems that don't use the nixbld user.
    export USER=$(whoami)
    export PGDATA=$out/.pgx/data-${pgxPostgresVersionString}/
    echo "unix_socket_directories = '$out/.pgx'" > $PGDATA/postgresql.conf 
    ${pgxPostgresPkg}/bin/pg_ctl start
    ${pgxPostgresPkg}/bin/createuser -h localhost --superuser --createdb $USER || true
    ${pgxPostgresPkg}/bin/pg_ctl stop

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
  preCheck = ''
    export PGX_HOME=$out/.pgx
    export NIX_PGLIBDIR=$out/.pgx/${pgxPostgresVersionString}/lib
  '';
  preBuild = ''
    export PGX_HOME=$out/.pgx
    ${cargo-pgx}/bin/cargo-pgx pgx schema
    ls -lah ./sql
    cp -v ./sql/* $out/
    rm -v $out/load-order.txt
    cp -v ./${cargoToml.package.name}.control $out/${cargoToml.package.name}.control
  '';
  preFixup = ''
    rm -r $out/.pgx
    mv $out/lib/* $out/
    rm -r $out/lib $out/bin
  '';
  PGX_PG_SYS_SKIP_BINDING_REWRITE = "1";
  CARGO_BUILD_INCREMENTAL = "false";
  RUST_BACKTRACE = "full";
  # This is required to have access to the `sql/*.sql` files.
  singleStep = true;

  cargoBuildOptions = default: default ++ [ "--no-default-features" "--features \"pg${pgxPostgresVersionString}\"" ];
  cargoTestOptions = default: default ++ [ "--no-default-features" "--features \"pg_test pg${pgxPostgresVersionString}\"" ];
  copyLibs = true;

  meta = with lib; {
    description = cargoToml.package.description;
    homepage = cargoToml.package.homepage;
    license = with licenses; [ mit ];
    maintainers = with maintainers; [ hoverbear ];
  };
}
