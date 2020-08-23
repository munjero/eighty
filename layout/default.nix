{ pkgs ? import <nixpkgs> { } }:

with pkgs;

let
  layout = ./.;

in {
  document = { name, source }: stdenv.mkDerivation {
    name = "eighty-layout-processed-${name}";
    unpackPhase = "true";
    buildInputs = [ python3Packages.jinja2 ];
    installPhase = ''
      mkdir -p $out
      ${python3}/bin/python ${layout}/process-document.py ${source} $out
    '';
  };

  spec = { name, sources }: stdenv.mkDerivation {
    name = "eighty-layout-processed-${name}";
    unpackPhase = "true";
    buildInputs = [ python3Packages.jinja2 ];
    installPhase = ''
      mkdir -p $out
      ${python3}/bin/python ${layout}/process-spec.py ${sources} $out
    '';
  };
}
