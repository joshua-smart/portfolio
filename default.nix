{
  pkgs ? import <nixpkgs> { },
}:
let
  bin = pkgs.rustPlatform.buildRustPackage {
    pname = "portfolio-bin";
    version = "0.1.0";

    src = ./.;

    cargoHash = "sha256-LLDl61IIE3Eeg4fUuRHn92iphZN7ELzlhlXh6AEJors=";
  };
in
pkgs.stdenvNoCC.mkDerivation {
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
}
