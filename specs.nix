{ pkgs ? import <nixpkgs> { } }:

let
  corepaper = import ./corepaper.nix { inherit pkgs; };
  corepaper-specs = "${corepaper.with-data}/_specs.json";

  template = import ./layout { inherit pkgs; };

in template.spec { name = "specs"; sources = corepaper-specs; }
