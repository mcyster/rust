# rust

## Development

To run, for example the hello_world program in sanity
```
cd $R_HOME/sanity/hellow_world
cargo run
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


# References

- https://github.com/bevyengine/bevy/blob/e67cfdf82b5726db4d449e9af31b865a5324aa19/docs/linux_dependencies.md#nix
