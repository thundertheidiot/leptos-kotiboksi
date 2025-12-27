{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-25.11";
    rust-overlay.url = "github:oxalica/rust-overlay";
    crane.url = "github:ipetkov/crane";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = {
    nixpkgs,
    rust-overlay,
    flake-utils,
    crane,
    ...
  }:
    flake-utils.lib.eachDefaultSystem (
      system: let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [
            (import rust-overlay)
          ];
        };

        toolchain = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
        craneLib = (crane.mkLib pkgs).overrideToolchain toolchain;

        cargoLeptos = craneLib.buildPackage rec {
          pname = "cargo-leptos";
          version = "0.3.2";

          cargoArtifacts = craneLib.buildDepsOnly {
            inherit src pname version buildInputs;
          };

          doCheck = false;
          cargoExtraArgs = "--locked --features no_downloads";

          buildInputs = with pkgs; [
            perl
          ];

          src = pkgs.fetchFromGitHub {
            owner = "leptos-rs";
            repo = pname;
            rev = "v${version}";
            hash = "sha256-aH7b9Dbz4vbh9X4RuhFdmu2U1sgGgbLUZxmwhVxW248=";
          };
        };

        wasmBindgen = pkgs.buildWasmBindgenCli rec {
          src = pkgs.fetchCrate {
            pname = "wasm-bindgen-cli";
            version = "0.2.106";
            hash = "sha256-M6WuGl7EruNopHZbqBpucu4RWz44/MSdv6f0zkYw+44=";
          };

          cargoDeps = pkgs.rustPlatform.fetchCargoVendor {
            inherit src;
            inherit (src) pname version;
            hash = "sha256-ElDatyOwdKwHg3bNH/1pcxKI7LXkhsotlDPQjiLHBwA=";
          };
        };

        cargoToml = builtins.fromTOML (builtins.readFile ./Cargo.toml);
        inherit (cargoToml.package) version name;
      in rec {
        packages.default = craneLib.buildPackage rec {
          inherit version;
          pname = name;

          src = ./.;
          # src = pkgs.lib.sourceFilesBySuffices ./. [
          #   "Cargo.lock"
          #   "Cargo.toml"
          #   ".rs"
          #   ".sql"
          #   ".scss"
          # ];

          cargoArtifacts = craneLib.buildDepsOnly {
            inherit src pname version buildInputs;
          };

          buildInputs = with pkgs; [
            cargoLeptos
            binaryen
            wasmBindgen
          ];

          doNotPostBuildInstallCargoBinaries = true;
          buildPhaseCargoCommand = "cargo leptos build --release -vvv";
          cargoTestCommand = "cargo leptos test --release -vvv";
          cargoExtraArgs = "";

          nativeBuildInputs = with pkgs; [
            makeWrapper
            sqlx-cli
          ];

          # needed to test sqlx queries
          preBuild = ''
            export DATABASE_URL=sqlite:./db.sqlite3
            sqlx database create
            sqlx migrate run
          '';

          installPhaseCommand = ''
            mkdir -p $out/bin
            cp target/x86_64-unknown-linux-gnu/release/${name} $out/bin/
            cp -r target/site $out/bin
            wrapProgram $out/bin/${name} --set LEPTOS_SITE_ROOT $out/bin/site
          '';
        };

        docker = pkgs.dockerTools.buildLayeredImage {
          name = "registry.gitlab.com/thundertheidiot/kotiboksi";
          tag = "latest";
          config = {
            Env = [
              "LEPTOS_SITE_ADDR=0.0.0.0:3000"
            ];
            ExposedPorts = {
              "3000" = {};
            };
            Cmd = "${packages.default}/bin/kotiboksi";
          };
        };

        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs;
            [
              toolchain
              cargoLeptos
              wasmBindgen
              leptosfmt
              binaryen
              sqlx-cli
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
