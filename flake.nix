{
  description = "Rust with WebAssembly";

  inputs = {
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    nixpkgs.url = "nixpkgs/nixos-24.05";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, fenix, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem
      (system:
        let
          pkgs = nixpkgs.legacyPackages.${system};
          f = with fenix.packages.${system}; combine [
            stable.toolchain
            targets.wasm32-unknown-unknown.stable.rust-std
          ];
        in
          {
            devShells.default =
              pkgs.mkShell {
                name = "rust-wasm-first-attempt";

                packages = with pkgs; [
                  f
                  llvmPackages.bintools
                  nodejs
                  wasm-pack
                ];
                CARGO_TARGET_WASM32_UNKNOWN_UNKNOWN_LINKER = "lld";
              };
          }
      );
}
