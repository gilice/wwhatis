{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-22.05";
    flake-utils.url = "github:numtide/flake-utils";
    crane.url = "github:ipetkov/crane";
    crane.inputs.nixpkgs.follows = "nixpkgs";
    flake-compat = {
      url = "github:edolstra/flake-compat";
      flake = false;
    };
  };

  outputs = { self, nixpkgs, crane, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
        };

        craneLib = crane.lib.${system};

        commonArgs = {
          src = ./.;
          buildInputs = with pkgs; [
            openssl
          ];

          nativeBuildInputs = with pkgs; [
            pkg-config
          ];
        };

        cargoArtifacts = craneLib.buildDepsOnly (pkgs.lib.recursiveUpdate commonArgs {
          pname = "wwhatis";
        });

        wwhatisClippy = craneLib.cargoClippy (pkgs.lib.recursiveUpdate commonArgs {
          inherit cargoArtifacts;
          cargoClippyExtraArgs = "--all-targets -- --deny warnings";
        });

        wwhatis = craneLib.buildPackage (pkgs.lib.recursiveUpdate commonArgs {
          inherit cargoArtifacts;
        });

      in
      {
        packages.default = wwhatis;
        devShells.default = pkgs.mkShell {
          # this is needed to run cargoabout on commit
          shellHook = ''
            git config core.hooksPath .githooks
            cargo-about generate about.hbs | sed "s/&quot;/'/g;s/&lt;/</g;s/&gt;/>/g;s/&#x27;/'/g" > thirdparty/THIRDPARTY
          '';

          buildInputs = with pkgs; [
            cargo-about
            convco
          ];

          RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
        };
      }
    );
}
