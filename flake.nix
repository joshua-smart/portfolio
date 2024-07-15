{
  description = "portfolio";

  inputs = {
    nixpkgs.url = "nixpkgs/nixos-24.05";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    crane = {
      url = "github:ipetkov/crane";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { nixpkgs, flake-utils, rust-overlay, crane, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs { inherit system overlays; };

        craneLib = crane.mkLib pkgs;

        bin = craneLib.buildPackage {
          src = pkgs.lib.cleanSourceWith {
            src = ./.;
            filter = path: type:
              (builtins.match "^templates/.*" != null)
              || (craneLib.filterCargoSources path type);
            name = "source";
          };
        };

        styles = pkgs.stdenv.mkDerivation {
          pname = "portfolio-styles";
          src = ./.;
          buildInputs = with pkgs; [ tailwindcss ];
          buildPhase = ''
            tailwindcss -i ./templates/input.css -o main.css -m
          '';
          installPhase = ''
            mkdir -p $out/assets/css/
            mv main.css $out/assets/css/
          '';
        };

        app = pkgs.lib.fileset.toSource {
          root = ./.;
          fileset = ./assets;
        };

        dockerImage = pkgs.dockerTools.buildImage {
          name = "portfolio";
          tag = "dev";
          copyToRoot = pkgs.buildEnv {
            name = "portfolio";
            paths = [ bin app styles ];
            pathsToLink = [ "/bin" "/assets" ];
          };
          config = { Cmd = [ "${bin}/bin/portfolio" ]; };
        };
      in {
        devShells.default = pkgs.mkShell {
          packages = with pkgs; [
            rust-bin.stable.latest.default
            rust-analyzer
            rustfmt
            cargo-watch
            vscode-langservers-extracted
            tailwindcss-language-server
            nodePackages.typescript-language-server
            taplo
            sqlx-cli
            sqlite
            (pkgs.writeShellScriptBin "tailwindcss" ''
              ${pkgs.tailwindcss}/bin/tailwindcss --input templates/input.css --output assets/css/main.css "$@"
            '')
          ];
        };

        packages = {
          inherit bin dockerImage;
          default = bin;
        };
      });
}
