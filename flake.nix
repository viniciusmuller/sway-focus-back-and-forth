{
  description = "Adds a back-and-forth focus mechanism to sway using its IPC";

  inputs.flake-utils.url = "github:numtide/flake-utils";

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let pkgs = nixpkgs.legacyPackages.${system}; in
      rec {
        devShell = pkgs.mkShell {
          buildInputs = with pkgs; [
            cargo
            rustc
            rust-analyzer
            rustfmt
          ];
        };
        defaultPackage = pkgs.rustPlatform.buildRustPackage {
          name = "sway-focus-back-and-forth";
          version = "0.1.0";
          src = ./.;
          cargoSha256 = "sha256-V6cH4HNdDWBJTLiT3hox2gJwulF4Lz09KU5H+HhKgTM=";
        };
      }
    );
}
