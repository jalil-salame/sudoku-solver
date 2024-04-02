{
  bzip2,
  stdenvNoCC,
  fetchurl,
}:
stdenvNoCC.mkDerivation {
  name = "sudoku17";
  src = fetchurl {
    url = "https://abhinavsarkar.net/files/sudoku17.txt.bz2";
    hash = "sha256-yIktP1fKBlZWXtzjxXIHDjgSA04BrmjUA7TY1cuAv6Q=";
  };
  dontUnpack = true;
  dontPatch = true;
  dontConfigure = true;
  dontBuild = true;
  doCheck = false;
  installPhase = "${bzip2}/bin/bunzip2 --stdout $src > $out";
  dontFixup = true;
}
