{
  description = "jupyter stuff";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs =
    {
      self,
      nixpkgs,
      flake-utils,
      ...
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import nixpkgs { inherit system; };
        lib = pkgs.lib;
        buildInputs = with pkgs; [ zeromq ];
        version = "0.1";
        pname = "shux";
      in
      {
        devShells.default = pkgs.mkShell {
          buildInputs = buildInputs;
          inputsFrom = with pkgs; [ zeromq ];
          LD_LIBRARY_PATH = lib.makeLibraryPath buildInputs;
        };
      }
    );
}
