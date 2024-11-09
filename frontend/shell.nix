with import (fetchTarball https://github.com/NixOS/nixpkgs/archive/4aa36568d413aca0ea84a1684d2d46f55dbabad7.tar.gz) { };
stdenv.mkDerivation {
  name = "frontend";
  buildInputs = [
    nodejs_22
  ];
}
