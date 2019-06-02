# micox

[![Build Status][build-badge]][build-status]

## Prerequisites

```bash
brew install qemu
cargo install bootimage
cargo install cargo-xbuild
rustup component add rust-src
rustup component add llvm-tools-preview
```

## Testing

```bash
cargo xtest
```

## Running

```bash
cargo xrun
```

## Building

```bash
cargo xbuild
cargo bootimage
```

[build-badge]: https://github.com/airt/micox/workflows/build/badge.svg
[build-status]: https://github.com/airt/micox/actions
