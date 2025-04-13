# nix builder refer to https://github.com/sxyazi/yazi
{
  description = "A File Classification Solution";
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
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
          ];
        };
        toolchain = pkgs.rust-bin.stable.latest.default;
        rustPlatform = pkgs.makeRustPlatform {
          cargo = toolchain;
          rustc = toolchain;
        };

        rev = self.shortRev or self.dirtyShortRev or "dirty";
        date = self.lastModifiedDate or self.lastModified or "19700101";
        version =
          (builtins.fromTOML (builtins.readFile ./file_classification_cli/Cargo.toml)).package.version
          + "pre${builtins.substring 0 8 date}_${rev}";
      in {
        packages = {
          file_classification_cli = pkgs.callPackage ./nix/file_classification_cli.nix {
            inherit
              version
              rev
              date
              rustPlatform
              ;
          };
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
