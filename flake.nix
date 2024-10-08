{
  description = "A File Classification Solution";
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay/b7996075da11a2d441cfbf4e77c2939ce51506fd"; # FIX: pin to a specific commit until cargo-c is updated
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = {
    self,
    nixpkgs,
    rust-overlay,
    flake-utils,
    ...
  }:
    flake-utils.lib.eachDefaultSystem (
      system: let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [
            rust-overlay.overlays.default
            (
              final: prev: let
                toolchain = final.rust-bin.stable.latest.default;
              in {
                rustPlatform = prev.makeRustPlatform {
                  cargo = toolchain;
                  rustc = toolchain;
                };
              }
            )
          ];
        };

        rev = self.shortRev or self.dirtyShortRev or "dirty";
        date = self.lastModifiedDate or self.lastModified or "19700101";
        version =
          (builtins.fromTOML (builtins.readFile ./file_classification_cli/Cargo.toml)).package.version
          + "pre${builtins.substring 0 8 date}_${rev}";
      in {
        packages = {
          file_classification_cli = pkgs.callPackage ./nix/file_classification_cli.nix {};
          default = self.packages.${system}.file_classification_cli;
        };

        devShells = {
          default = pkgs.callPackage ./nix/shell.nix {};
        };

        formatter = pkgs.nixfmt-rfc-style;
      }
    )
    // {
      overlays = {
        default = self.overlays.file_classification_cli;
        file_classification_cli = _: prev: {inherit (self.packages.${prev.stdenv.system}) file_classification_cli;};
      };
    };
}
