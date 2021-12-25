{ pkgs ? import <nixpkgs> { } }:

with pkgs;

let
  pandoc-process = ./.;
  pandoc-sidenote = pkgs.haskellPackages.pandoc-sidenote;
  mkScript = import ../../nix/mk-script.nix { inherit pkgs; };

in mkScript {
  name = "eighty-pandoc";
  script = "${python3}/bin/python ${pandoc-process}/process.py";
  env = [ pandoc-sidenote pandoc ];
}
