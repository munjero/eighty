{ pkgs ? import <nixpkgs> { } }: { name, source }:

with pkgs;

let
  pandoc-process = ./.;

  pandoc-sidenote = pkgs.haskellPackages.pandoc-sidenote;

in stdenv.mkDerivation {
  name = "eighty-pandoc-processed-${name}";
  unpackPhase = "true";
  buildInputs = [ pandoc pandoc-sidenote ];
  installPhase = ''
    mkdir -p $out
    ${python3}/bin/python ${pandoc-process}/process.py ${source} $out
  '';
}
