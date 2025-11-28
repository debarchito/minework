### 1. Build/Run (Linux)

This project makes use of the [mold](https://github.com/rui314/mold) linker. See
[flake.nix](./flake.nix) and [.cargo/config.toml](./.cargo/config.toml).

> **NOTE:** The project has only been tested on Linux. I cannot guarentee
> (at-least in the short term) if it works on Windows or macOS. For starters,
> mold doesn't support either Windows or macOS; so you could potentially start
> testing by changing the linker being used for this project. On macOS
> particularly, you could use the new linker that Apple ships with Xcode 15 and
> above; it's compareable to mold when it comes to speed and parallelization.

```fish
nix build git+https://codeberg.org/debarchito/minework#minework
./result/bin/minework --help
# or
nix run git+https://codeberg.org/debarchito/minework#minework -- --help
```

Alternatively, if you don't want to utilize [Nix](https://nixos.org):

```fish
git clone ssh://git@codeberg.org/debarchito/minework.git
# or use HTTPS instead of SSH: https://codeberg.org/debarchito/minework.git
cd minework && cargo build --release
./target/release/minework --help
```

### 2. Development

This project ships with an [.envrc](./.envrc) file that can be used to scaffold
a development shell using [direnv](https://direnv.net) and Nix.

```
direnv allow
```

### 3. Licensing

This project is licensed under the [Zlib license](./LICENSE).
