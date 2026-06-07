echo "Run normally, shows tracing messages"
cargo run

echo "Run the tests normally, takes ~5 seconds"
cargo test

echo "Run with madsim enabled, test should complete nearly instantly"
RUSTFLAGS="--cfg madsim" cargo test
