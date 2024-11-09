# TODO: switch to flake
let
  pkgs = import (fetchTarball("https://github.com/NixOS/nixpkgs/archive/85f7e662eda4fa3a995556527c87b2524b691933.tar.gz")) {};
in pkgs.mkShell {
  buildInputs = [ pkgs.cargo pkgs.rustc pkgs.diesel-cli pkgs.postgresql.lib ];
}
