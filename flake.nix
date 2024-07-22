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

  outputs =
    {
      nixpkgs,
      flake-utils,
      rust-overlay,
      crane,
      ...
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs { inherit system overlays; };

        craneLib = crane.mkLib pkgs;

        lib = nixpkgs.lib;

        bin = craneLib.buildPackage {
          src = pkgs.lib.cleanSourceWith {
            src = ./.;
            filter =
              path: type:
              (lib.path.hasPrefix ./templates (/. + path))
              || (lib.path.hasPrefix ./.sqlx (/. + path))
              || (lib.path.hasPrefix ./migrations (/. + path))
              || (craneLib.filterCargoSources path type);
          };
        };

        assets = pkgs.stdenv.mkDerivation {
          name = "portfolio-assets";
          src = ./.;
          buildInputs = with pkgs; [ tailwindcss ];
          buildPhase = ''
            tailwindcss -i ./templates/input.css -o main.css -m
          '';
          installPhase = ''
            mkdir -p $out/assets/
            cp -r assets/* $out/assets/
            mkdir -p $out/assets/css/
            mv main.css $out/assets/css/
          '';
        };

        dockerImage = pkgs.dockerTools.buildImage {
          name = "portfolio";
          tag = "dev";
          copyToRoot = pkgs.buildEnv {
            name = "portfolio";
            paths = [
              bin
              assets
            ];
            pathsToLink = [
              "/bin"
              "/assets"
            ];
          };
          config = {
            Cmd = [ "${bin}/bin/portfolio" ];
          };
        };
      in
      {
        devShells.default = pkgs.mkShell {
          packages = with pkgs; [
            # Language Servers
            rust-analyzer
            rustfmt
            vscode-langservers-extracted
            tailwindcss-language-server
            nodePackages.typescript-language-server
            taplo

            rust-bin.stable.latest.default
            cargo-watch
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
      }
    );
}
