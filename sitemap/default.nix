{ pkgs ? import <nixpkgs> { } }: { name, source }:

with pkgs;

let
  sitemap = ./.;

in stdenv.mkDerivation {
  name = "eighty-sitemap-processed-${name}";
  unpackPhase = "true";
  installPhase = ''
    mkdir -p $out
    ${python3}/bin/python ${sitemap}/process.py ${source} $out
  '';
}
