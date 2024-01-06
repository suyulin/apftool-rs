cargo build --target=aarch64-apple-darwin -r
cargo build --target=x86_64-apple-darwin -r
lipo -create  target/aarch64-apple-darwin/release/afptool-rs target/x86_64-apple-darwin/release/afptool-rs -output  target/release/afptool-rs