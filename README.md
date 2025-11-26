### 1. Build (Linux)

This project makes use of the [mold](https://github.com/rui314/mold) linker. See
[flake.nix](/flake.nix) and [.cargo/config.toml](/.cargo/config.toml).

```sh
nix build github:debarchito/minework#minework
# or
git clone git@github.com:debarchito/minework
cd minework
cargo build --release
```

### 2. Development

```
direnv allow
```

### 3. Licensing

This project is licensed under the [Zlib license](/LICENSE).
