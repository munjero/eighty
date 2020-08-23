{ pkgs ? import <nixpkgs> { } }:

let
  documents = import ./documents.nix { inherit pkgs; };

in documents {
  name = "classic";
  source = ./sites/classic;
}
