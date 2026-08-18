{
  rustPlatform,
  stdenvNoCC,
  tailwindcss,
  ...
}:
let
  bin = rustPlatform.buildRustPackage {
    pname = "portfolio-bin";
    version = "0.1.0";

    src = ./.;

    cargoHash = "sha256-l8FDHNr49r78vx/Sgp/Q2Ds4P5iWLoeDKJ7TlWOrvSQ=";
  };
in
stdenvNoCC.mkDerivation {
  name = "portfolio";
  src = ./.;
  buildInputs = [ tailwindcss ];
  buildPhase = ''
    tailwindcss -i ./templates/input.css -o main.css -m
  '';
  installPhase = ''
    mkdir -p $out/assets/
    cp -r assets/* $out/assets/
    install -D data.ron $out/data.ron
    install -D main.css $out/assets/css/main.css
    install -D  ${bin}/bin/portfolio $out/bin/portfolio
  '';
}
