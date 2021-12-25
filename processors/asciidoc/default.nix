{ pkgs ? import <nixpkgs> { } }:

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
  mkScript = import ../../nix/mk-script.nix { inherit pkgs; };

in mkScript {
  name = "eighty-asciidoc";
  env = [ gems ruby ];
  script = "${gems}/bin/bundle exec ${asciidoc}/bin/process";
}
