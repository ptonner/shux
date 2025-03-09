{
  description = "jupyter stuff";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs =
    {
      self,
      rust-overlay,
      nixpkgs,
      flake-utils,
      ...
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ (import rust-overlay) ];
        };
        lib = pkgs.lib;
        buildInputs = with pkgs; [
          zeromq
          openssl.dev
          glib.dev
          pkg-config

          clippy
          rust-analyzer
          just
        ];
      in
      {
        # REF: https://discourse.nixos.org/t/rust-src-not-found-and-other-misadventures-of-developing-rust-on-nixos/11570/12
        devShell = pkgs.mkShell {
          nativeBuildInputs = with pkgs; [
            (pkgs.rust-bin.stable.latest.default.override {
              extensions = [
                "rust-src"
                "cargo"
                "rustc"
              ];
            })
            gcc
          ];

          RUST_SRC_PATH = "${
            pkgs.rust-bin.stable.latest.default.override { extensions = [ "rust-src" ]; }
          }/lib/rustlib/src/rust/library";

          # inputsFrom = with pkgs; [ zeromq ];
          buildInputs = buildInputs;
          LD_LIBRARY_PATH = lib.makeLibraryPath buildInputs;
        };
      }
    );
}
