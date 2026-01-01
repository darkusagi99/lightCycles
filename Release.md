### add target for release
rustup target add x86_64-pc-windows-msvc

### release command
cargo build --release --target x86_64-pc-windows-msvc
    