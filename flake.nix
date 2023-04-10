{
    inputs = {
        flake-utils.url = "github:numtide/flake-utils";
        nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    };
    outputs = { self, flake-utils, nixpkgs }:
        flake-utils.lib.eachDefaultSystem (system:
            let
                pkgs = import nixpkgs {
                    inherit system;
                };
            in
            {
                devShells.default = pkgs.mkShell {
                    buildInputs = with pkgs; [
                        protobuf
                    ];
                };
            }
        );
}