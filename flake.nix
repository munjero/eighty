{
  inputs.nixpkgs.url = "github:nixos/nixpkgs/nixos-21.11";
  outputs = { self, nixpkgs }: {
    devShell."x86_64-linux" = let
      pkgs = import nixpkgs { system = "x86_64-linux"; };
      asciidocProcessor = import ./processors/asciidoc { inherit pkgs; };
      pandocProcessor = import ./processors/pandoc { inherit pkgs; };
      eighty = pkgs.rustPlatform.buildRustPackage rec {
        pname = "eighty";
        version = "0.1.0";
        src = ./.;
        cargoSha256 = "sha256-XE4/vQs9DwnQhokjbtteYZ+VSVjDO6Nz7ocUNasgk10=";
      };
    in with pkgs; mkShell {
      buildInputs = [
        asciidocProcessor
        pandocProcessor
        eighty
      ];
    };
  };
}
