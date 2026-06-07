Simple example intended to show that fixing the seed for madsim will make the
tokio task order consistent. Looks like it's already consistent in the
beginning, then madsim introduced randomization and finally fixing the seed
makes it deterministic again.
