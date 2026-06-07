echo "Run tests normally"
for i in {1..3}; do
    cargo test -q -- --show-output | grep INFO
    echo "----------------------------------"
done

echo "Run the tests with madsim, random seeds"
for i in {1..3}; do
    RUSTFLAGS="--cfg madsim" cargo test -q -- --show-output | grep INFO
    echo "----------------------------------"
done

echo "Run with madsim with a set seed"
for i in {1..3}; do
    MADSIM_TEST_SEED=1234 RUSTFLAGS="--cfg madsim" cargo test -q -- --show-output | grep INFO
    echo "----------------------------------"
done
