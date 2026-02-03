{
  description = "Description for the project";

  nixConfig = {
    extra-trusted-public-keys = "eigenvalue.cachix.org-1:ykerQDDa55PGxU25CETy9wF6uVDpadGGXYrFNJA3TUs=";
    extra-substituters = "https://eigenvalue.cachix.org";
  };

  inputs = {
    flake-parts.url = "github:hercules-ci/flake-parts";
    cargo2nix.url = "github:cargo2nix/cargo2nix/release-0.12";
    nixpkgs.follows = "cargo2nix/nixpkgs";
    unstable.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
  };

  outputs = inputs@{ flake-parts, ... }:
    flake-parts.lib.mkFlake { inherit inputs; } {
      imports = [ ];
      systems = [ "x86_64-linux" "aarch64-linux" "aarch64-darwin" "x86_64-darwin" ];
      perSystem = { config, self', inputs', pkgs, system, ... }:
        let
          p = import inputs.nixpkgs
            {
              inherit system;
              overlays = [ inputs.cargo2nix.overlays.default ];
            };
          unstable = import inputs.unstable { inherit system; };
          rustPkgs = p.rustBuilder.makePackageSet {
            rustVersion = "1.83.0";
            packageFun = import ./Cargo.nix;
          };

          workspaceShell = rustPkgs.workspaceShell {
            packages = [
              inputs.cargo2nix.packages.${system}.cargo2nix
              unstable.turso
              unstable.turso-cli
            ];
          };

          phreaky = rustPkgs.workspace.phreaky { };
        in
        {
          packages.default = phreaky;
          devShells.default = workspaceShell;
          apps = {
            default = { type = "app"; program = "${ pkgs.lib.getExe' phreaky "phreaky" }"; };
            bootstrap = {
              type = "app";
              program = (pkgs.writeShellScriptBin "bootstrap" ''
                ${p.cargo}/bin/cargo generate-lockfile
                ${inputs.cargo2nix.packages.${system}.cargo2nix}/bin/cargo2nix
                ${pkgs.git}/bin/git add Cargo.nix Cargo.lock
              '');
            };
          };
        };
      flake = { };
    };
}
