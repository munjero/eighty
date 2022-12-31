{
  inputs.nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
  outputs = { self, nixpkgs }: let
    system = "x86_64-linux";
    pkgs = import nixpkgs { inherit system; };
    asciidocProcessor = import ./processors/asciidoc { inherit pkgs; };
    pandocProcessor = import ./processors/pandoc { inherit pkgs; };
    # eighty =

  in {
    devShell."${system}" = pkgs.mkShell {
      buildInputs = [
        asciidocProcessor
        pandocProcessor
      ];
    };

    packages."${system}".eighty = let
      rustPkg = pkgs.rustPlatform.buildRustPackage rec {
        pname = "eighty";
        version = "0.1.0";
        src = ./.;
        cargoSha256 = "sha256-XE4/vQs9DwnQhokjbtteYZ+VSVjDO6Nz7ocUNasgk10=";
      };

    in pkgs.writeShellApplication {
      name = "eighty";
      text = ''
        ${rustPkg}/bin/eighty "$@"
      '';
      runtimeInputs = [ asciidocProcessor pandocProcessor ];
    };

    apps."${system}".build = let
      scriptPkg = pkgs.writeScriptBin "build-to-dist" ''
        #!${pkgs.stdenv.shell}

        ${self.packages."${system}".eighty}/bin/eighty build sites dist
      '';
    in {
      type = "app";
      program = "${scriptPkg}/bin/build-to-dist";
    };
  };
}
