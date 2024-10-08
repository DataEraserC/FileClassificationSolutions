{pkgs ? import <nixpkgs> {}}:
pkgs.mkShell {
  # Get dependencies from the main package
  inputsFrom = [(pkgs.callPackage ./default.nix {})];
  # Additional tooling
  buildInputs = with pkgs; [
    rust-analyzer # LSP Server
    rustfmt # Formatter
    clippy # Linter
    sqlite
    openssl
    diesel-cli
  ];
  shellHook = ''
    export OPENSSL_DIR="${pkgs.openssl.dev}"
    export OPENSSL_LIB_DIR="${pkgs.openssl.out}/lib"
  '';
}
