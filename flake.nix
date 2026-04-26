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

      androidBuildToolsVersion = "34.0.0";
      androidEnv = pkgs.androidenv.override {licenseAccepted = true;};
      # NOTE: Requires programs.nix-ld.enable = true;
      androidComposition = androidEnv.composeAndroidPackages {
        includeNDK = true;
        ndkVersion = "26.1.10909125";

        cmdLineToolsVersion = "8.0";
        platformVersions = ["34"];
        platformToolsVersion = "37.0.0";
        buildToolsVersions = [androidBuildToolsVersion];
        abiVersions = ["x86_64" "arm64-v8a"];
        includeExtras = ["extras;google;m2repository"];
        useGoogleAPIs = true;
        extraLicenses = [
          "android-googletv-license"
          "android-sdk-arm-dbt-license"
          "android-sdk-license"
          "android-sdk-preview-license"
          "google-gdk-license"
          "intel-android-extra-license"
          "intel-android-sysimage-license"
          "mips-android-sysimage-license"
        ];

        includeEmulator = true;
        emulatorVersion = "36.6.3";

        includeSystemImages = true;
        systemImageTypes = ["google_apis" "google_apis_playstore"];
      };
      androidSdk = androidComposition.androidsdk;

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

          dioxus-cli
          tailwindcss_4
          pkg-config
          trunk
          cargo-tarpaulin
          sqlx-cli

          # Android
          nix-ld
          androidComposition.androidsdk
        ]
        ++ runtimeDeps;
    in {
      devShells.default = pkgs.mkShell {
        inherit buildInputs;

        RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
        ANDROID_HOME = "${androidSdk}/libexec/android-sdk";
        ANDROID_SDK_ROOT = "${androidSdk}/libexec/android-sdk";
        JAVA_HOME = pkgs.jdk21.home;
        GRADLE_OPTS = "-Dorg.gradle.project.android.aapt2FromMavenOverride=${androidSdk}/libexec/android-sdk/build-tools/${androidBuildToolsVersion}/aapt2";
        QT_QPA_PLATFORM = "wayland;xcb";
        LD_LIBRARY_PATH = "${pkgs.lib.makeLibraryPath [pkgs.vulkan-loader pkgs.libGL]}";

        shellHook = ''
          dx --version
        '';
      };
    });
}
