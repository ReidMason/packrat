{
  description = "An inventory manager";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
  };

  outputs = {
    self,
    nixpkgs,
  }: let
    systems = ["x86_64-linux" "aarch64-linux"];

    eachSystem = f:
      nixpkgs.lib.genAttrs systems (system:
        f system nixpkgs.legacyPackages.${system});
  in {
    packages = eachSystem (system: pkgs: {
      default = pkgs.rustPlatform.buildRustPackage {
        name = "packrat";
        src = ./.;
        buildInputs = [pkgs.glib];
        nativeBuildInputs = [pkgs.pkg-config];
        cargoLock.lockFile = ./Cargo.lock;
      };
    });

    devShells = eachSystem (system: pkgs: {
      default = pkgs.mkShell {
        packages = with pkgs; [
          cargo
          rustc
          rustfmt
          clippy
          rust-analyzer
          cargo-tarpaulin
        ];

        RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
      };
    });
  };
}
