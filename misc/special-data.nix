{ pkgs ? import <nixpkgs> { } }: { name, source }:

with pkgs;

stdenv.mkDerivation {
  name = "special-data-processed-${name}";
  unpackPhase = "true";
  installPhase = ''
    mkdir -p $out
    cp -r ${source}/* $out
  '';
}
