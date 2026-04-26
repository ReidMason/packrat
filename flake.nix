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
        config = {
          allowUnfree = true;
          android_sdk.accept_license = true;
        };
      };

      androidComposition = pkgs.androidenv.composeAndroidPackages {
        includeNDK = true;
        ndkVersion = "26.1.10909125";
        platformVersions = ["34"];
        buildToolsVersions = ["34.0.0"];
        abiVersions = ["x86_64" "arm64-v8a"];
        includeExtras = ["extras;google;m2repository"];
      };

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
        # GStreamer
        gst_all_1.gstreamer
        gst_all_1.gst-plugins-base
        gst_all_1.gst-plugins-good
      ];

      buildInputs = with pkgs;
        [
          # Rust toolchain with WASM support for Web
          (rust-bin.stable.latest.default.override {
            extensions = ["rust-src" "rust-analyzer"];
            targets = [
              "wasm32-unknown-unknown"
              "aarch64-linux-android"
              "x86_64-linux-android"
            ];
          })

          # Dioxus CLI and tools
          dioxus-cli
          pkg-config
          trunk
          cargo-tarpaulin
          sqlx-cli

          # Android
          android-tools
          androidComposition.androidsdk
        ]
        ++ runtimeDeps;
    in {
      devShells.default = pkgs.mkShell {
        inherit buildInputs;

        # Ensures Cargo can find libraries for Desktop compilation
        shellHook = ''
          export LD_LIBRARY_PATH="$LD_LIBRARY_PATH:${pkgs.lib.makeLibraryPath runtimeDeps}"
          export GST_PLUGIN_SYSTEM_PATH_1_0="${pkgs.gst_all_1.gst-plugins-base}/lib/gstreamer-1.0:${pkgs.gst_all_1.gst-plugins-good}/lib/gstreamer-1.0"

          export ANDROID_HOME="${androidComposition.androidsdk}/libexec/android-sdk"
          export ANDROID_NDK_HOME="$ANDROID_HOME/ndk-bundle"
          export JAVA_HOME="${pkgs.jdk17.home}"
          export GRADLE_USER_HOME="$PWD/.gradle"
          export ANDROID_SDK_ROOT="$ANDROID_HOME"
          export GRADLE_OPTS="-Dandroid.aapt2FromMaven=false"

          dx --version
        '';
      };
    });
}
