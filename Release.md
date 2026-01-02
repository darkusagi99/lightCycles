## Windows
### add target for release
rustup target add x86_64-pc-windows-msvc

### release command
cargo build --release --target x86_64-pc-windows-msvc

## Web
### add target for release
rustup target add wasm32-unknown-unknown

### Build WebAssembly
cargo build --release --target wasm32-unknown-unknown

    