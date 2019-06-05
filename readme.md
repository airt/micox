# micox

[![Build Status][build-badge]][build-status]

## Prerequisites

```bash
brew install qemu
cargo install bootimage
rustup component add rust-src llvm-tools-preview
```

## Testing

```bash
cargo test
```

## Running

```bash
cargo run
```

## Building

```bash
cargo bootimage
```

[build-badge]: https://github.com/airt/micox/workflows/build/badge.svg
[build-status]: https://github.com/airt/micox/actions
