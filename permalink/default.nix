{ pkgs ? import <nixpkgs> { } }: { name, source }:

with pkgs;

let
  permalink = ./.;

in stdenv.mkDerivation {
  name = "eighty-permalink-processed-${name}";
  unpackPhase = "true";
  installPhase = ''
    mkdir -p $out
    ${python3}/bin/python ${permalink}/process.py ${source} $out
  '';
}
