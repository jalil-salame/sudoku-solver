{
  lib,
  rustPlatform,
  fetchCrate,
  valgrind-light,
  symlinkJoin,
  makeWrapper,
}: let
  cargoToml = lib.importTOML ./Cargo.toml;
  tomlVersion = cargoToml.dev-dependencies.iai-callgrind;
  cargoLock = lib.importTOML ./Cargo.lock;
  pname = "iai-callgrind-runner";
  version = (lib.lists.findFirst ({name, ...}: name == pname) {version = tomlVersion;} cargoLock.package).version;
  unwrapped = rustPlatform.buildRustPackage {
    inherit pname version;
    src = fetchCrate {
      inherit pname version;
      hash = "sha256-C8n7qMoG4VaLLH7anLjQ43Xcz+fZwiR+C4iRw4jQ5LY=";
    };
    cargoHash = "sha256-L9FKSe3sma4tS0KGIOEMc1R/AClmZMvnx+41IOJs8CM=";
    doCheck = false; # tests fail for some reason?
  };
in
  symlinkJoin {
    name = "iai-callgrind-runner";
    paths = [unwrapped];
    buildInputs = [makeWrapper];
    postBuild = ''
      wrapProgram $out/bin/iai-callgrind-runner \
        --prefix PATH : "${lib.makeBinPath [valgrind-light]}"
    '';
  }
