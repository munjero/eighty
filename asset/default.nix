{ pkgs ? import <nixpkgs> { } }: { name, source }:

with pkgs;

let
  cssDist = ./css;
  fontDist = ./font;
  jsDist = ./js;

in stdenv.mkDerivation {
  name = "eighty-asset-processed-${name}";
  unpackPhase = "true";
  installPhase = ''
    mkdir -p $out
    cp -r ${source}/* $out
    cp -r ${fontDist} $out/font
    cp -r ${jsDist} $out/js
    cp -r ${cssDist} $out/css
  '';
}
