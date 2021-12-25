{ pkgs ? import <nixpkgs> { } }:

with pkgs;

{ name, script, env ? [ ] }: writeTextFile {
  name = "${name}";
  executable = true;
  destination = "/bin/${name}";
  text = ''
    #!${bash}/bin/bash

    for i in ${lib.concatStringsSep " " env}; do
      export PATH="$i/bin:$PATH"
    done

    exec ${script} $@
  '';
}
