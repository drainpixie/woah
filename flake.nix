{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    utils.url = "github:numtide/flake-utils";

    rust = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    hooks = {
      url = "github:cachix/pre-commit-hooks.nix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = {
    nixpkgs,
    hooks,
    utils,
    rust,
    self,
    ...
  }:
    utils.lib.eachDefaultSystem (system: let
      overlays = [(import rust)];
      pkgs = import nixpkgs {inherit system overlays;};

      toolchain = pkgs.rust-bin.selectLatestNightlyWith (toolchain:
        toolchain.default.override {
          extensions = ["rust-src"];
        });
    in {
      devShells.default = let
        check = self.checks.${system}.pre-commit-check;
      in
        nixpkgs.legacyPackages.${system}.mkShell {
          inherit (check) shellHook;
          buildInputs = check.enabledPackages;
        };

      checks.pre-commit-check = let
        lib = hooks.lib.${system};
      in
        lib.run {
          src = ./.;

          hooks = {
            clippy = {
              enable = true;

              packageOverrides = {
                clippy = toolchain;
                cargo = toolchain;
              };
            };

            rustfmt = {
              enable = true;

              packageOverrides = {
                rustfmt = toolchain;
                cargo = toolchain;
              };
            };
          };
        };
    });
}
