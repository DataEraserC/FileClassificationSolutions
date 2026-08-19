{
  rustPlatform,
  version ? "git",
  rev ? "unknown",
  date ? "19700101",
  sqlite,
  mysql80,
  postgresql,
  pkg-config,
  lib,
  ...
}: let
  manifest = (lib.importTOML ../file_classification_webapi/Cargo.toml).package;
  src = lib.fileset.toSource {
    root = ../.;
    fileset = lib.fileset.unions [
      ../Cargo.toml
      ../Cargo.lock
      ../common
      ../file_classification_core
      ../file_classification_webapi
      ../migrations
      ../migrations_sqlite
      ../migrations_mysql
      ../migrations_postgres
    ];
  };
in
  rustPlatform.buildRustPackage rec {
    inherit src;
    nativeBuildInputs = [sqlite];
    buildInputs = [sqlite];
    # nativeBuildInputs = [pkg-config sqlite mysql80 postgresql];
    # buildInputs = [pkg-config sqlite mysql80 postgresql];
    pname = manifest.name;
    version = manifest.version;
    cargoLock.lockFile = ../Cargo.lock;
  }
