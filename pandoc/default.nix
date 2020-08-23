{ pkgs ? import <nixpkgs> { } }: { name, source }:

with pkgs;

let
  pandoc-process = ./.;

  pandoc-sidenote = pkgs.haskell.packages.ghc883.pandoc-sidenote.overrideDerivation (oldAttrs: {
    src = pkgs.fetchFromGitHub {
      owner = "jez";
      repo = "pandoc-sidenote";
      rev = "766eb9db5c849b1007f5c310b8012e7da68e51a6";
      sha256 = "11xk353ypb8gvd15kk756hccc6bdhf42bj3mzjcdd8nsynhzp2cy";
    };
  });

in stdenv.mkDerivation {
  name = "eighty-pandoc-processed-${name}";
  unpackPhase = "true";
  buildInputs = [ pandoc pandoc-sidenote ];
  installPhase = ''
    mkdir -p $out
    ${python3}/bin/python ${pandoc-process}/process.py ${source} $out
  '';
}
