echo "STDOUT Default -------------------------"
cargo run

echo ""
echo "Log Debug ------------------------------"
RUST_LOG=debug cargo run

echo ""
echo "Partial log trace ----------------------"
RUST_LOG=error,rs_tracing_env_control_level::module1=trace cargo run

echo ""
echo "Submodule log trace --------------------"
RUST_LOG=error,rs_tracing_env_control_level::module2::submodule2=trace,rs_tracing_env_control_level::module1=warn cargo run
