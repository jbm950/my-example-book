# 1553 Devices Example

Example that extends the previous 1553 examples to have the remote terminals
simulate actual devices. To run the example start the following executables in
different terminals in the following order:

```
cargo run --bin bus
cargo run --bin rt -- 5
cargo run --bin rt -- 13
cargo run --bin bus_controller
```

Note, this example took and built upon the library code that was started in the
1553 send-data example. Most of the "example specific" code lives in /app and
/bin with the rest being reusable architecture.

# Limitations/Further Thoughts
* There are a lot of spots constructing vectors. Consideration could be added
  to reduce the number of heap allocations necessary especially when doing
  empty values (maybe change data on messages to be an option?)

# Next steps
* The next project could simply be setting up the bus controller to do
  scheduled and manual transactions.
