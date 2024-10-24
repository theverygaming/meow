with import (fetchTarball https://github.com/NixOS/nixpkgs/archive/2768c7d042a37de65bb1b5b3268fc987e534c49d.tar.gz) { };
let
  sillyORMPackage = pkgs.python312Packages.buildPythonPackage rec {
    pname = "sillyorm";
    version = "dev"; # TODO: release
    pyproject = true;

    nativeBuildInputs = [
      python312Packages.setuptools
    ];

    src = fetchFromGitHub {
      owner = "theverygaming";
      repo = "sillyORM";
      rev = "dev";
      hash = "sha256-huxVwYQ8ZoqLCGIiqvq5Z8vUXaxRFU4V+R+UNsioZf0=";
    };
  };
in
stdenv.mkDerivation {
  name = "meow";
  buildInputs = [
    python312

    sillyORMPackage

    # lint, fmt, type, docs
    python312Packages.pylint
    python312Packages.mypy
    python312Packages.black
    sphinx
    gnumake

    # test
    python312Packages.coverage
    python312Packages.pytest

    # build
    python312Packages.build

    # deps
    python312Packages.fastapi

    # postgres
    python312Packages.psycopg2
    python312Packages.types-psycopg2
    postgresql_16

    # for convenience
    sqlitebrowser

    # for dev server
    python312Packages.uvicorn
  ];
}
