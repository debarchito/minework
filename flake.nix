{
  inputs = {
    nixpkgs.url = "https://channels.nixos.org/nixos-unstable/nixexprs.tar.xz";
    nixpkgs-lib.follows = "nixpkgs";
    flake-parts = {
      url = "github:hercules-ci/flake-parts";
      inputs.nixpkgs-lib.follows = "nixpkgs-lib";
    };
    crane.url = "github:ipetkov/crane";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    advisory-db = {
      url = "github:rustsec/advisory-db";
      flake = false;
    };
  };
  outputs =
    inputs@{
      nixpkgs,
      flake-parts,
      crane,
      rust-overlay,
      advisory-db,
      ...
    }:
    flake-parts.lib.mkFlake { inherit inputs; } {
      imports = [
        flake-parts.flakeModules.easyOverlay
      ];

      systems = [
        "x86_64-linux"
        "aarch64-linux"
      ];

      perSystem =
        { system, config, ... }:
        let
          pkgs = import nixpkgs {
            inherit system;
            overlays = [
              (import rust-overlay)
            ];
          };

          toolchain = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
          craneLib = (crane.mkLib pkgs).overrideToolchain toolchain;
          src = craneLib.cleanCargoSource ./.;

          commonArgs = {
            inherit src;
            strictDeps = true;
            nativeBuildInputs = [ pkgs.mold ];
            RUSTFLAGS = "-Clink-args=-fuse-ld=mold";
          };

          cargoArtifacts = craneLib.buildDepsOnly commonArgs;

          minework = craneLib.buildPackage (
            commonArgs
            // rec {
              pname = "minework";
              version = "0.1.0";

              inherit cargoArtifacts;

              postInstall = ''
                export HOME=$(mktemp -d)
                mkdir -p $out/share/bash-completion/completions
                $out/bin/${pname} completion bash > $out/share/bash-completion/completions/${pname}
                mkdir -p $out/share/zsh/site-functions
                $out/bin/${pname} completion zsh > $out/share/zsh/site-functions/_${pname}
                mkdir -p $out/share/fish/vendor_completions.d
                $out/bin/${pname} completion fish > $out/share/fish/vendor_completions.d/${pname}.fish
                mkdir -p $out/share/elvish/lib
                $out/bin/${pname} completion elvish > $out/share/elvish/lib/${pname}.elv
                mkdir -p $out/share/powershell/Modules/${pname}
                $out/bin/${pname} completion powershell > $out/share/powershell/Modules/${pname}/${pname}.psm1
                mkdir -p $out/share/nushell/vendor/autoload
                $out/bin/${pname} completion nushell > $out/share/nushell/vendor/autoload/${pname}.nu
              '';
            }
          );
        in
        {
          packages = {
            inherit minework;
            default = minework;
          };

          checks = {
            inherit minework;

            minework-clippy = craneLib.cargoClippy (
              commonArgs
              // {
                inherit cargoArtifacts;
                cargoClippyExtraArgs = "--all-targets -- --deny warnings";
              }
            );

            minework-fmt = craneLib.cargoFmt {
              inherit src;
            };

            minework-audit = craneLib.cargoAudit {
              inherit src advisory-db;
            };
          };

          overlayAttrs = {
            inherit (config.packages) minework;
          };

          devShells.default = craneLib.devShell {
            name = "minework-dev";
            checks = config.checks;
          };
        };
    };
}
