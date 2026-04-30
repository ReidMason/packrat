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

      inherit (pkgs) lib;

      androidBuildToolsVersion = "34.0.0";

      # Linux-only: NDK, emulator, images, nix-ld. Kept in a function so the outer `let`/`in` parse cleanly.
      makeLinuxAndroid = pkgs: let
        androidEnv = pkgs.androidenv.override {licenseAccepted = true;};
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
      in {
        inherit androidComposition androidSdk;
        packages = [
          pkgs.nix-ld
          androidSdk
        ];
        shellEnv = {
          ANDROID_HOME = "${androidSdk}/libexec/android-sdk";
          ANDROID_SDK_ROOT = "${androidSdk}/libexec/android-sdk";
          JAVA_HOME = pkgs.jdk21.home;
          GRADLE_OPTS = "-Dorg.gradle.project.android.aapt2FromMavenOverride=${androidSdk}/libexec/android-sdk/build-tools/${androidBuildToolsVersion}/aapt2";
          QT_QPA_PLATFORM = "wayland;xcb";
          LD_LIBRARY_PATH = "${lib.makeLibraryPath [pkgs.vulkan-loader pkgs.libGL]}";
        };
      };

      linuxAndroid =
        if pkgs.stdenv.isLinux
        then makeLinuxAndroid pkgs
        else {
          androidComposition = null;
          androidSdk = null;
          packages = [];
          shellEnv = {};
        };

      linuxDesktopDeps = with pkgs; [
        webkitgtk_4_1
        gtk3
        cairo
        gdk-pixbuf
        glib
        dbus
        librsvg
        xdotool
        gst_all_1.gstreamer
        gst_all_1.gst-plugins-base
        gst_all_1.gst-plugins-good
      ];

      # Linux: GTK/WebKit/GStreamer for Dioxus desktop. Darwin uses system frameworks via the linker;
      # explicit apple_sdk.frameworks stubs were removed from nixpkgs (see nixpkgs Darwin legacy frameworks).
      runtimeDeps = lib.optionals pkgs.stdenv.isLinux linuxDesktopDeps;

      rustToolchain = pkgs.rust-bin.stable.latest.default.override {
        extensions = ["rust-src" "rust-analyzer"];
        targets =
          ["wasm32-unknown-unknown" "aarch64-apple-darwin"]
          ++ (
            lib.optionals pkgs.stdenv.isLinux [
              "aarch64-linux-android"
              "x86_64-linux-android"
            ]
          );
      };
      commonInputs = with pkgs; [
        rustToolchain
        dioxus-cli
        tailwindcss_4
        pkg-config
        openssl_3
        trunk
        cargo-tarpaulin
        sqlx-cli
      ];

      buildInputs =
        commonInputs ++ linuxAndroid.packages ++ runtimeDeps;
    in {
      devShells.default = pkgs.mkShell ({
          inherit buildInputs;

          RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";

          shellHook = ''
            dx --version
          '';
        }
        // linuxAndroid.shellEnv);
    });
}
