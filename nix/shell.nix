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
          extensions = [
            "rust-src"
            "rustfmt"
            "rust-analyzer"
            "clippy"
          ];
        })

        nodePackages.cspell

        pkgs.diesel-cli

        pkgs.bashInteractive
      ]
      ++ (oa.nativeBuildInputs or []);

    env.RUST_BACKTRACE = "1";
  })
