{
  description = "portfolio";

  inputs = {
    nixpkgs.url = "nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    {
      self,
      nixpkgs,
      flake-utils,
      rust-overlay,
      ...
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs { inherit system overlays; };

        portfolio = pkgs.callPackage ./default.nix { };
      in
      {
        devShells.default = pkgs.mkShell {
          packages = with pkgs; [
            cargo
            rustc
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
    )
    // {
      nixosModules.default = ./module.nix;

      overlay = final: prev: { portfolio = self.packages.x86_64-linux.default; };
      overlays.default = self.overlay;
    };
}
