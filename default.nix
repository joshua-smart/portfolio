{
  pkgs ? import <nixpkgs> { },
}:
pkgs.callPackage ./portfolio.nix { }
