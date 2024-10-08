{
  rustPlatform,
  sqlite,
  lib,
  ...
}: let
  manifest = (lib.importTOML ../file_classification_cli/Cargo.toml).package;
  src = lib.fileset.toSource {
    root = ../.;
    fileset = lib.fileset.unions [
      ../Cargo.toml
      ../Cargo.lock
      (lib.fileset.fromSource (lib.sources.sourceByRegex ../. ["^file_classification_.*"]))
    ];
  };
in
  rustPlatform.buildRustPackage rec {
    inherit src;
    nativeBuildInputs = [sqlite];
    buildInputs = [sqlite];
    pname = manifest.name;
    version = manifest.version;
    cargoLock.lockFile = ../Cargo.lock;
  }
