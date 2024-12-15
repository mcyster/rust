# rust

## Development

To run
```
cd $R_HOME
cargo run --bin hello_world
cargo run --bin rectangles
```

If your having problems try:
```
cargo clean
cargo build
cargo run --bin hello_world
```

## Setup

Install nix-shell on your environment

```
git clone git@github.com:mcyster/rust.git
cd rust
nix-shell
# develop in this shell
```

