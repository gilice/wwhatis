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
        defaultPackage = pkgs.rustPlatform.buildRustPackage rec {
          pname = "wwhatis";
          version = "0.1";
          src = self;

          nativeBuildInputs = with pkgs; [

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
          ];


          buildInputs = with pkgs; [
            openssl
            pkg-config
          ];

          postPhases = ''
            echo asdasdasds
          '';
          cargoSha256 = "sha256-BiHDmXNR/kMnkqQ3NJ+jwC1zFGtbW/vtlgZr54eDeAU=";
        };

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
