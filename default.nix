{ pkgs ? import <nixpkgs> { } }:

with pkgs;

let
  corepaper = import ./corepaper.nix { inherit pkgs; };
  classic = import ./classic.nix { inherit pkgs; };
  specs = import ./specs.nix { inherit pkgs; };
  wei = import ./wei.nix { inherit pkgs; };
  kulupu = import ./kulupu.nix { inherit pkgs; };

  all = stdenv.mkDerivation {
    name = "eighty-sites-all";
    unpackPhase = "true";
    installPhase = ''
      mkdir -p $out
      ln -s ${corepaper.out} $out/corepaper
      ln -s ${classic.out} $out/classic
      ln -s ${wei.out} $out/wei
      ln -s ${kulupu.out} $out/kulupu
      ln -s ${specs} $out/specs
    '';
  };

in all
