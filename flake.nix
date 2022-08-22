{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-22.05";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let pkgs = import nixpkgs {
        inherit system;
      };
      in
      {
        devShells.default = pkgs.mkShell {
          shellHook = ''
            git config core.hooksPath .githooks
          '';
          buildInputs = with pkgs;
            [
              # basic
              rustc
              cargo

              # for development
              rustfmt

              # custom
              cargo-about
              upx

              # needed for SSL
              openssl
              pkg-config

              git
            ];

          RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
        };
      }
    );
}
