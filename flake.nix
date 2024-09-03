{
  description = "portfolio";

  inputs = {
    nixpkgs.url = "nixpkgs/nixos-unstable";
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

        craneLib = (crane.mkLib pkgs).overrideToolchain (p: p.rust-bin.stable.latest.default);

        bin = craneLib.buildPackage { src = ./.; };

        portfolio = pkgs.stdenv.mkDerivation {
          name = "portfolio";
          src = ./.;
          buildInputs = with pkgs; [ tailwindcss ];
          buildPhase = ''
            tailwindcss -i ./templates/input.css -o main.css -m
          '';
          installPhase = ''
            mkdir -p $out/assets/
            cp -r assets/* $out/assets/
            install -D main.css $out/assets/css/main.css
            install -D  ${bin}/bin/portfolio $out/bin/portfolio
          '';
        };
      in
      {
        devShells.default = pkgs.mkShell {
          inputsFrom = [ bin ];

          packages = with pkgs; [
            # Language Servers
            rust-analyzer
            rustfmt
            vscode-langservers-extracted
            tailwindcss-language-server
            nodePackages.typescript-language-server
            taplo

            cargo-watch
            sqlx-cli
            sqlite
            (writeShellScriptBin "tailwindcss" ''
              ${tailwindcss}/bin/tailwindcss --input templates/input.css --output assets/css/main.css "$@"
            '')
          ];
        };

        packages = {
          inherit portfolio;
          default = portfolio;
        };
      }
    );
}
