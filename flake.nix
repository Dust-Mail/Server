{
  description = "Dust-Mail HTTP REST backend api";

  inputs = {
    cargo2nix.url = "github:cargo2nix/cargo2nix/release-0.11.0";
    flake-utils.follows = "cargo2nix/flake-utils";
    flake-compat = {
      url = "github:edolstra/flake-compat";
      flake = false;
    };
    nixpkgs.url = "github:nixos/nixpkgs/nixos-23.11";

  };

  outputs = inputs: with inputs;
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ cargo2nix.overlays.default ];
        };

        rustPkgs = pkgs.rustBuilder.makePackageSet {
          rustVersion = "1.75.0";
          packageFun = import ./Cargo.nix;
        };

      in
      rec {
        devShells = {
          default = rustPkgs.workspaceShell {
            packages = with pkgs;
              [
                pkg-config
                nodejs_20
                yarn
              ];
          };
        };

        packages = {
          dust-mail-server = (rustPkgs.workspace.dust-mail-server { });

          default = packages.dust-mail-server;
        };
      }
    );

}
