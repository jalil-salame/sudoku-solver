{rustPlatform}: let
  pname = "sudoku-solver";
  version = "0.0.1";
in
  rustPlatform.buildRustPackage {
    inherit pname version;
    src = builtins.path {
      path = ./.;
      name = pname;
    };
    cargoLock.lockFile = ./Cargo.lock;
    useNextest = true;
  }
