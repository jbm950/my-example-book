echo "STDOUT ---------------------------------"
cargo run

echo ""
echo "Log File -------------------------------"
cat app.log

echo ""
echo "STDOUT with Level set in Env -----------"
RUST_LOG=trace cargo run
