{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    hooks.url = "github:cachix/pre-commit-hooks.nix";
    utils.url = "github:numtide/flake-utils";
  };

  outputs = {
    self,
    nixpkgs,
    hooks,
    utils,
    ...
  }:
    utils.lib.eachDefaultSystem (system: let
      pkgs = import nixpkgs {inherit system;};
      name = "woah_${system}";
      src = ./.;

      nativeBuildInputs = builtins.attrValues {
        inherit (pkgs) gnumake pkg-config;
      };

      buildInputs = builtins.attrValues {
        inherit (pkgs) clang-tools;
        inherit (pkgs.llvmPackages_latest) libstdcxxClang libcxx;
      };
    in {
      packages.default = pkgs.stdenv.mkDerivation {
        inherit src name;
        inherit nativeBuildInputs buildInputs;

        buildPhase = ''
          TARGET=${name} make
        '';

        installPhase = ''
          mkdir -p $out/bin
          cp ${name} $out/bin
        '';
      };

      checks.pre-commit-check = let
        lib = hooks.lib.${system};
      in
        lib.run {
          inherit src;

          hooks = {
            clang-format.enable = true;
            clang-tidy.enable = true;
          };
        };

      devShells.default = let
        check = self.checks.${system}.pre-commit-check;
      in
        nixpkgs.legacyPackages.${system}.mkShell {
          inherit (check) shellHook;
          inherit nativeBuildInputs;

          buildInputs = buildInputs ++ check.enabledPackages;
        };
    });
}
