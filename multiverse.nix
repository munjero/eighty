{ pkgs ? import <nixpkgs> { } }:

let
  documents = import ./documents.nix { inherit pkgs; };

in documents {
  name = "multiverse";
  source = ./sites/multiverse;
}
