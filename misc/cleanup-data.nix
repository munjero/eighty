{ pkgs ? import <nixpkgs> { } }: { name, source }:

with pkgs;

stdenv.mkDerivation {
  name = "cleanup-data-processed-${name}";
  unpackPhase = "true";
  installPhase = ''
    mkdir -p $out
    cp -r ${source}/* $out
    find $out -name '_*' -exec rm {} \;
  '';
}
