{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixpkgs-unstable";
    flake-parts.url = "github:hercules-ci/flake-parts";

    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = { self, flake-parts, ... } @inputs:
    flake-parts.lib.mkFlake { inherit inputs; } {
      systems = [ "x86_64-linux" "x86_64-darwin" "aarch64-darwin" "aarch64-linux" ];

      perSystem = { self', config, pkgs, lib, system, ... }: let
        rustToolchain = pkgs.rust-bin.stable.latest.default.override {
          extensions = [ "rust-src" "rust-analyzer" "clippy" ];
          targets = [ "wasm32-unknown-unknown" ];
        };
        rustBuildInputs = (with pkgs; [ openssl libiconv pkg-config ])
          ++ lib.optionals pkgs.stdenv.isLinux (with pkgs; [
            glib gtk3 libsoup_3 webkitgtk_4_1 xdotool
          ])
          ++ lib.optionals pkgs.stdenv.isDarwin (with pkgs.darwin.apple_sdk.frameworks; [
            SystemConfiguration IOKit Carbon WebKit Security Cocoa
          ]);
      in
      {
        _module.args.pkgs = import inputs.nixpkgs {
          inherit system;
          overlays = [ inputs.rust-overlay.overlays.default ];
        };

        formatter = pkgs.nixfmt-rfc-style;
        packages.default = let
          cargoToml = builtins.fromTOML (builtins.readFile ./Cargo.toml);
          rev = toString (self.shortRev or self.dirtyShortRev or self.lastModified or "unknown");
        in
        pkgs.rustPlatform.buildRustPackage {
          pname = cargoToml.package.name;
          version = "${cargoToml.package.version}-${rev}";
          src = ./.;
          strictDeps = true;
          buildInputs = rustBuildInputs;
          nativeBuildInputs = with pkgs; [
            dioxus-cli
            rustToolchain
            rustPlatform.bindgenHook
            wasm-bindgen-cli_0_2_100
          ] ++ rustBuildInputs;
          buildPhase = ''
            dx build --release --platform web
          '';
          installPhase = ''
            mkdir -p $out
            cp -r target/dx/$pname/release/web $out/bin
          '';
          cargoLock.lockFile = ./Cargo.lock;
          meta.mainProgram = "server";
        };

        devShells.default = pkgs.mkShell {
          name = "dioxus-dev";
          buildInputs = rustBuildInputs;
          nativeBuildInputs = with pkgs; [
            # Add shell dependencies here
            rustToolchain
            wasm-bindgen-cli_0_2_100
            dioxus-cli
          ];
          shellHook = ''
            # For rust-analyzer 'hover' tooltips to work.
            export RUST_SRC_PATH="${rustToolchain}/lib/rustlib/src/rust/library";
          '';
        };
      };
    };
}
