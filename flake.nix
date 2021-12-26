{
  inputs.nixpkgs.url = "github:nixos/nixpkgs/nixos-21.11";
  outputs = { self, nixpkgs }: {
    devShell."x86_64-linux" = let
      pkgs = import nixpkgs { system = "x86_64-linux"; };
      asciidocProcessor = import ./processors/asciidoc { inherit pkgs; };
      pandocProcessor = import ./processors/pandoc { inherit pkgs; };
    in with pkgs; mkShell {
      buildInputs = [
        asciidocProcessor
        pandocProcessor
      ];
    };
  };
}
