{
  description = "Rerun external mesh loader for collada files";

  inputs = {
    flake-parts = {
      url = "github:hercules-ci/flake-parts";
      inputs.nixpkgs-lib.follows = "nixpkgs";
    };
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  };

  outputs =
    inputs:
    inputs.flake-parts.lib.mkFlake { inherit inputs; } {
      systems = inputs.nixpkgs.lib.systems.flakeExposed;
      perSystem =
        { pkgs, self', ... }:
        {
          devShells.default = pkgs.mkShell {
            packages = [
              pkgs.cargo-release
              pkgs.clippy
              pkgs.rustfmt
            ];
            inputsFrom = [ self'.packages.default ];
          };
          packages = {
            default = self'.packages.rerun-loader-collada;
            rerun-loader-collada = pkgs.callPackage ./. { };
          };
        };
    };
}
