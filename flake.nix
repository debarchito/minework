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
      self,
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
            overlays = [
              (import rust-overlay)
              self.overlays.default
            ];
          };
        in
        {
          packages = {
            minework = pkgs.minework;
            default = pkgs.minework;
          };
          devShells.default = pkgs.mkShell {
            name = "minework";
            nativeBuildInputs = [
              (pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml)
              pkgs.mold
            ];
            RUSTFLAGS = "-Clink-args=-fuse-ld=mold";
          };
        };
      flake.overlays.default =
        final: prev:
        let
          rust-overlay-pkg = import rust-overlay;
          rustOverlay = rust-overlay-pkg final prev;
        in
        rustOverlay
        // {
          minework =
            let
              rust-toolchain = rustOverlay.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
              naersk' = final.callPackage naersk {
                rustc = rust-toolchain;
                cargo = rust-toolchain;
              };
            in
            naersk'.buildPackage rec {
              name = "minework";
              version = "0.1.0";
              src = ./.;
              nativeBuildInputs = [ final.mold ];
              RUSTFLAGS = "-Clink-args=-fuse-ld=mold";
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
        };
    };
}
