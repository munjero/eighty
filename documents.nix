{ pkgs ? import <nixpkgs> { } }:
{ name, source }:

let
  special-data =
    import ./misc/special-data.nix { inherit pkgs; };
  asciidoc = import ./asciidoc { inherit pkgs; };
  pandoc = import ./pandoc { inherit pkgs; };
  sitemap = import ./sitemap { inherit pkgs; };
  permalink = import ./permalink { inherit pkgs; };
  layout = import ./layout { inherit pkgs; };
  revision = import ./revision { inherit pkgs; };
  asset = import ./asset { inherit pkgs; };
  cleanup-data =
    import ./misc/cleanup-data.nix { inherit pkgs; };

  special-data-processed = special-data { inherit name; source = source; };
  asciidoc-processed = asciidoc { inherit name; source = special-data-processed; };
  pandoc-processed = pandoc { inherit name; source = asciidoc-processed; };
  sitemap-processed = sitemap { inherit name; source = pandoc-processed; };
  revision-processed = revision { inherit name; source = sitemap-processed; gitSource = source; };
  permalink-processed = permalink { inherit name; source = revision-processed; };
  layout-processed = layout.document { inherit name; source = permalink-processed; };
  asset-processed = asset { inherit name; source = layout-processed; };
  cleanup-data-processed = cleanup-data { inherit name; source = asset-processed; };

in {
  raw-jsondoc = asciidoc-processed;
  raw-permalink = permalink-processed;
  with-data = asset-processed;
  out = cleanup-data-processed;
}
