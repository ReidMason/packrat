{
  description = "An inventory manager";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = {
    self,
    nixpkgs,
    rust-overlay,
    flake-utils,
    ...
  }:
    flake-utils.lib.eachDefaultSystem (system: let
      overlays = [(import rust-overlay)];
      pkgs = import nixpkgs {
        inherit system overlays;
      };

      # Specific dependencies for Dioxus Desktop/Fullstack
      runtimeDeps = with pkgs; [
        webkitgtk_4_1
        gtk3
        cairo
        gdk-pixbuf
        glib
        dbus
        openssl_3
        librsvg
        xdotool
      ];

      buildInputs = with pkgs;
        [
          # Rust toolchain with WASM support for Web
          (rust-bin.stable.latest.default.override {
            extensions = ["rust-src" "rust-analyzer"];
            targets = ["wasm32-unknown-unknown"];
          })

          # Dioxus CLI and tools
          dioxus-cli
          pkg-config
          trunk
          cargo-tarpaulin
          sqlx-cli
        ]
        ++ runtimeDeps;
    in {
      devShells.default = pkgs.mkShell {
        inherit buildInputs;

        # Ensures Cargo can find libraries for Desktop compilation
        shellHook = ''
          export LD_LIBRARY_PATH="$LD_LIBRARY_PATH:${pkgs.lib.makeLibraryPath runtimeDeps}"
          dx --version
        '';
      };
    });
}
