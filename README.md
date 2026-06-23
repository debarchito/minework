### 1. Build, Run and Install (Linux)

`minework` makes use of the [mold](https://github.com/rui314/mold) linker. See
[flake.nix](./flake.nix) and [.cargo/config.toml](./.cargo/config.toml).

> **NOTE:** The project has only been tested on Linux. I cannot guarantee
> (at-least in the short term) if it works on Windows or macOS. For starters,
> mold doesn't support either Windows or macOS; so you could potentially start
> testing by changing the linker being used for this project. On macOS
> particularly, you could use the new linker that Apple ships with Xcode 15 and
> above; it's comparable to mold when it comes to speed and parallelization.

```fish
nix build git+https://git.sr.ht/~debarchito/minework#minework
./result/bin/minework --help
# or
nix run git+https://git.sr.ht/~debarchito/minework#minework -- --help
```

Alternatively, if you don't want to utilize [Nix](https://nixos.org):

```fish
git clone https://git.sr.ht/~debarchito/minework
cd minework && cargo build --release
./target/release/minework --help
```

To install `minework` on NixOS/through Nix, you can make use of flakes:

```nix
# flake.nix
minework = {
  url = "git+https://git.sr.ht/~debarchito/minework";
  inputs.nixpkgs.follows = "nixpkgs"; # Optional
};

# Using the overlay
pkgs = import nixpkgs {
  overlays = [
    # ...
    minework.overlays.default
    # ...
  ];
};

environment.systemPackages = [
  # ...
  pkgs.minework
  # ...
];

# or, consume the package directly
environment.systemPackages = [
  # ...
  minework.packages.${system}.default
  # e.g. minework.packages.x86_64-linux.default
  # ...
];
```

Alternatively, you can use `cargo` to install `minework`:

```fish
cargo install --git https://git.sr.ht/~debarchito/minework
```

### 2. Development

This project ships with an [.envrc](./.envrc) file that can be used to scaffold
a development shell using [direnv](https://direnv.net) and Nix.

```fish
direnv allow
```

### 3. Licensing

This project is licensed under the [Zlib license](./LICENSE).
