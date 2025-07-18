{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixpkgs-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = { self, nixpkgs, rust-overlay, ... } @inputs: let
    systems = [ "x86_64-linux" "x86_64-darwin" "aarch64-darwin" "aarch64-linux" ];
    
    perSystemOutputs = system: let
      pkgs = import nixpkgs {
        inherit system;
        overlays = [ rust-overlay.overlays.default ];
      };
      
      rustToolchain = pkgs.rust-bin.stable.latest.minimal.override {
        extensions = [ "rust-src" ];
        targets = [ "wasm32-unknown-unknown" ];
      };
      
      rustBuildInputs = (with pkgs; [ openssl libiconv pkg-config ])
        ++ pkgs.lib.optionals pkgs.stdenv.isLinux (with pkgs; [
          glib gtk3 libsoup_3 webkitgtk_4_1 xdotool
        ])
        ++ pkgs.lib.optionals pkgs.stdenv.isDarwin (with pkgs.darwin.apple_sdk.frameworks; [
          SystemConfiguration IOKit Carbon WebKit Security Cocoa
        ]);
      
      cargoToml = builtins.fromTOML (builtins.readFile ./Cargo.toml);
      rev = toString (self.shortRev or self.dirtyShortRev or self.lastModified or "unknown");
    in {
      packages.default = pkgs.rustPlatform.buildRustPackage {
        pname = cargoToml.package.name;
        version = "${cargoToml.package.version}-${rev}";
        src = ./.;
        cargoLock.lockFile = ./Cargo.lock;
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
          mkdir -p $out/bin
          cp -r target/dx/$pname/release/desktop $out/bin
        '';
        meta.mainProgram = "server";
      };
      devShells.default = pkgs.mkShell {
        name = cargoToml.package.name;
        buildInputs = rustBuildInputs;
        nativeBuildInputs = with pkgs; [
          rustToolchain
          wasm-bindgen-cli_0_2_100
          dioxus-cli
        ];
        shellHook = ''
          export RUST_SRC_PATH="${rustToolchain}/lib/rustlib/src/rust/library";
        '';
      };
      formatter = pkgs.nixfmt-rfc-style;
    };
    
  in {
    packages = nixpkgs.lib.genAttrs systems (system: (perSystemOutputs system).packages);
    devShells = nixpkgs.lib.genAttrs systems (system: (perSystemOutputs system).devShells);
    formatter = nixpkgs.lib.genAttrs systems (system: (perSystemOutputs system).formatter);
  };
}
