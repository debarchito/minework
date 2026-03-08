{
  inputs = {
    nixpkgs.url = "https://channels.nixos.org/nixos-unstable/nixexprs.tar.xz";
    flake-parts.url = "github:hercules-ci/flake-parts";
    systems.url = "github:nix-systems/default";
    naersk.url = "github:nix-community/naersk";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };
  outputs =
    inputs@{
      nixpkgs,
      flake-parts,
      systems,
      naersk,
      rust-overlay,
      ...
    }:
    flake-parts.lib.mkFlake { inherit inputs; } {
      imports = [
        flake-parts.flakeModules.easyOverlay
      ];

      systems = import systems;

      perSystem =
        {
          lib,
          system,
          config,
          ...
        }:
        let
          pkgs = import nixpkgs {
            inherit system;
            overlays = [
              (import rust-overlay)
            ];
          };

          toolchain = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;

          naersk' = pkgs.callPackage naersk {
            rustc = toolchain;
            cargo = toolchain;
          };

          nativeBuildInputs = [ pkgs.mold ];

          RUSTFLAGS = "-Clink-args=-fuse-ld=mold";
        in
        {
          packages = rec {
            minework = naersk'.buildPackage rec {
              name = "minework";
              version = "0.1.0";
              src = lib.cleanSource ./.;

              inherit nativeBuildInputs RUSTFLAGS;

              postInstall = ''
                export HOME=$(mktemp -d)
                mkdir -p $out/share/bash-completion/completions
                $out/bin/${name} completion bash > $out/share/bash-completion/completions/${name}
                mkdir -p $out/share/zsh/site-functions
                $out/bin/${name} completion zsh > $out/share/zsh/site-functions/_${name}
                mkdir -p $out/share/fish/vendor_completions.d
                $out/bin/${name} completion fish > $out/share/fish/vendor_completions.d/${name}.fish
                mkdir -p $out/share/elvish/lib
                $out/bin/${name} completion elvish > $out/share/elvish/lib/${name}.elv
                mkdir -p $out/share/powershell/Modules/${name}
                $out/bin/${name} completion powershell > $out/share/powershell/Modules/${name}/${name}.psm1
                mkdir -p $out/share/nushell/vendor/autoload
                $out/bin/${name} completion nushell > $out/share/nushell/vendor/autoload/${name}.nu
              '';
            };

            default = minework;
          };

          overlayAttrs = {
            inherit (config.packages) minework;
          };

          devShells.default = pkgs.mkShell {
            name = "minework-dev";

            nativeBuildInputs = [ toolchain ] ++ nativeBuildInputs;

            inherit RUSTFLAGS;
          };
        };
    };
}
