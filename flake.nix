{
  description = "A basic Rust devshell for NixOS users developing Leptos";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-24.05";
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
    flake-utils.lib.eachDefaultSystem (
      system: let
        overlays = [(import rust-overlay)];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
      in
        with pkgs; {
          devShells.default = mkShell {
            buildInputs = with pkgs;
              [
                openssl
                pkg-config
                cacert
                cargo-make
                trunk
                sqlite
                (rust-bin.selectLatestNightlyWith (toolchain:
                  toolchain.default.override {
                    extensions = ["rust-src" "rust-analyzer"];
                    targets = ["wasm32-unknown-unknown" "x86_64-unknown-linux-gnu"];
                  }))
              ]
              ++ pkgs.lib.optionals pkg.stdenv.isDarwin [
                darwin.apple_sdk.frameworks.SystemConfiguration
              ];

            shellHook = ''
              [ ! -f ./guestbook.db ] && touch guestbook.db
              export DATABASE_URL="sqlite:guestbook.db"

              export PKG_CONFIG_PATH="${pkgs.sqlite.dev}/lib/pkgconfig"
              export RUSTFLAGS="-C target-feature=-crt-static"
            '';
          };
        }
    );
}
