echo "Run tests normally"
for i in {1..3}; do
    cargo test -q --lib | grep "test result"
    echo "----------------------------------"
done

echo ""

echo "Run the tests with madsim, random seeds"
for i in {1..3}; do
    RUSTFLAGS="--cfg madsim" cargo test -q --lib | grep "test result"
    echo "----------------------------------"
done

echo ""

echo "Run with madsim with a failing seed: 1780859886242972210"
for i in {1..3}; do
    MADSIM_TEST_SEED=1780859886242972210 RUSTFLAGS="--cfg madsim" cargo test -q --lib | grep "test result"
    echo "----------------------------------"
done
