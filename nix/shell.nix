{
  callPackage,
  rust-bin,
  nodePackages,
  pkgs,
}: let
  mainPkg = callPackage ./file_classification_cli.nix {};
in
  mainPkg.overrideAttrs (oa: {
    nativeBuildInputs =
      [
        (rust-bin.stable.latest.default.override {
          # rustc cargo rust-std rust-mingw rust-docs rustfmt-preview clippy-preview
          extensions = [
            "rust-src"
            "rustfmt"
            "rust-analyzer"
            "clippy"
            # for rust-rover (?)
            # "rust-std"
          ];
        })

        # for rust-rover (?)
        # pkgs.rustup

        nodePackages.cspell

        pkgs.diesel-cli

        pkgs.bashInteractive

        pkgs.pkg-config
      ]
      ++ (oa.nativeBuildInputs or []);

    env.RUST_BACKTRACE = "1";
  })
