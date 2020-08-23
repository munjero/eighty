{ pkgs ? import <nixpkgs> { } }: { name, gitSource, source }:

with pkgs;

let
  revision = ./.;

in stdenv.mkDerivation {
  name = "eighty-revision-processed-${name}";
  unpackPhase = "true";
  buildInputs = [ perl bash git python3 ];
  installPhase = ''
    mkdir -p $out
    python3 ${revision}/process.py ${gitSource} ${source} $out
  '';
}
