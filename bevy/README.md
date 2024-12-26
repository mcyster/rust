

# Bevy

## References

https://bevyengine.org/learn/

https://bevy-cheatbook.github.io/setup/getting-started.html
  - https://bevy-cheatbook.github.io/setup/bevy-config.html?highlight=webgl2

https://github.com/bevyengine/bevy/discussions?discussions_q=


# wasm

cargo run --target wasm32-unknown-unknown

wasm-bindgen --out-dir ./out --target web /home/wal/rust/target/wasm32-unknown-unknown/debug/bevy.wasm


# Setup

## Ubuntu

if getting an error like:
```
...
wgpu_core::device::global: surface configuration failed: incompatible window kind    
...
```

Try using sofware emulation, with
```
export WGPU_BACKEND=vulkan
```





