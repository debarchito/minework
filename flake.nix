{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    flake-parts.url = "github:hercules-ci/flake-parts";
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
      naersk,
      rust-overlay,
      ...
    }:
    flake-parts.lib.mkFlake { inherit inputs; } {
      systems = [
        "x86_64-linux"
        "aarch64-linux"
        "aarch64-darwin"
      ];
      perSystem =
        { system, ... }:
        let
          pkgs = import nixpkgs {
            inherit system;
            overlays = [ (import rust-overlay) ];
          };
          rust-toolchain = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
          naersk' = pkgs.callPackage naersk {
            rustc = rust-toolchain;
            cargo = rust-toolchain;
          };
          commonBuildInputs = [
            pkgs.mold
          ];
          env.RUSTFLAGS = "-Clink-args=-fuse-ld=mold";
        in
        {
          packages = rec {
            minework = naersk'.buildPackage rec {
              name = "minework";
              version = "0.1.0";
              src = ./.;
              nativeBuildInputs = commonBuildInputs;
              inherit (env) RUSTFLAGS;
              postInstall = ''
                export HOME=$(mktemp -d)

                # Bash
                mkdir -p $out/share/bash-completion/completions
                $out/bin/${name} completion bash > $out/share/bash-completion/completions/${name}

                # Zsh
                mkdir -p $out/share/zsh/site-functions
                $out/bin/${name} completion zsh > $out/share/zsh/site-functions/_${name}

                # Fish
                mkdir -p $out/share/fish/vendor_completions.d
                $out/bin/${name} completion fish > $out/share/fish/vendor_completions.d/${name}.fish

                # Elvish
                mkdir -p $out/share/elvish/lib
                $out/bin/${name} completion elvish > $out/share/elvish/lib/${name}.elv

                # PowerShell
                mkdir -p $out/share/powershell/Modules/${name}
                $out/bin/${name} completion powershell > $out/share/powershell/Modules/${name}/${name}.psm1

                # Nushell
                mkdir -p $out/share/nushell/vendor/autoload
                $out/bin/${name} completion nushell > $out/share/nushell/vendor/autoload/${name}.nu
              '';
            };
            default = minework;
          };

          devShells.default = pkgs.mkShell {
            name = "minework";
            nativeBuildInputs = [
              rust-toolchain
            ]
            ++ commonBuildInputs;
            inherit (env) RUSTFLAGS;
          };
        };
    };
}
