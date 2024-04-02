{
  lib,
  rustPlatform,
}: let
  cargoToml = (lib.importTOML ./Cargo.toml).package;
in
  rustPlatform.buildRustPackage {
    inherit (cargoToml) version;
    pname = cargoToml.name;
    src = builtins.path {
      inherit (cargoToml) name;
      path = ./.;
    };
    cargoLock.lockFile = ./Cargo.lock;
    useNextest = true;
  }
