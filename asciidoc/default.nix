{ pkgs ? import <nixpkgs> { } }: { name, source }:

with pkgs;

let
  gems = bundlerEnv {
    name = "eighty-asciidoc-gems";
    inherit ruby;
    gemdir = ./.;
  };

  asciidoc = stdenv.mkDerivation {
    name = "eighty-asciidoc";
    buildInputs = [gems ruby];
    installPhase = ''
      mkdir -p $out
      cp -r . $out
    '';
    src = ./.;
  };

in stdenv.mkDerivation {
  name = "eighty-asciidoc-processed-${name}";
  buildInputs = [gems ruby];
  unpackPhase = "true";
  installPhase = ''
    mkdir -p $out
    ${asciidoc}/bin/process ${source} $out
  '';
}
