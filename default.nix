{ pkgs ? import <nixpkgs> { } }:

with pkgs;

let
  corepaper = import ./corepaper.nix { inherit pkgs; };
  classic = import ./classic.nix { inherit pkgs; };
  specs = import ./specs.nix { inherit pkgs; };
  wei = import ./wei.nix { inherit pkgs; };
  multiverse = import ./multiverse.nix { inherit pkgs; };
  tifmuhezahi = import ./tifmuhezahi.nix { inherit pkgs; };

  all = stdenv.mkDerivation {
    name = "eighty-sites-all";
    unpackPhase = "true";
    installPhase = ''
      mkdir -p $out
      ln -s ${corepaper.out} $out/corepaper
      ln -s ${classic.out} $out/classic
      ln -s ${wei.out} $out/wei
      ln -s ${multiverse.out} $out/multiverse
      ln -s ${tifmuhezahi.out} $out/tifmuhezahi
      ln -s ${specs} $out/specs
    '';
  };

in all
