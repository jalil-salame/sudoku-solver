{
  description = "A Sudoku Solver written in Rust as an exercise.";

  inputs.nixpkgs.url = "nixpkgs/nixos-unstable";

  inputs.pre-commit-hooks.url = "github:cachix/pre-commit-hooks.nix";
  inputs.pre-commit-hooks.inputs.nixpkgs.follows = "nixpkgs";

  outputs = {
    self,
    nixpkgs,
    pre-commit-hooks,
  }: let
    supportedSystems = ["x86_64-linux" "aarch64-darwin" "x86_64-darwin" "aarch64-linux"];
    forEachSystem = f:
      nixpkgs.lib.genAttrs supportedSystems (system:
        f {
          inherit system;
          pkgs = import nixpkgs {inherit system;};
        });
  in {
    packages = forEachSystem ({pkgs, ...}: let
      sudoku-solver = pkgs.callPackage ./default.nix {};
      sudoku17 = pkgs.callPackage ./sudoku17.nix {};
    in {
      inherit sudoku-solver sudoku17;
      default = sudoku-solver;
    });
    checks = forEachSystem ({
      pkgs,
      system,
    }: {
      pre-commit-check = pre-commit-hooks.lib.${system}.run {
        src = builtins.path {
          path = ./.;
          name = "sudoku-solver";
        };
        hooks.typos.enable = true;
        hooks.alejandra.enable = true;
        hooks.clippy.enable = true;
        hooks.rustfmt.enable = true;
        hooks.cargo-check.enable = true;
      };
    });
    formatter = forEachSystem ({pkgs, ...}: pkgs.alejandra);
    devShells = forEachSystem ({
      pkgs,
      system,
    }: {
      default = pkgs.mkShellNoCC {
        inherit (self.checks.${system}.pre-commit-check) shellHook;
        nativeBuildInputs = with pkgs; [alejandra];
        SUDOKU17 = "${self.packages.${system}.sudoku17}";
      };
    });
  };
}
