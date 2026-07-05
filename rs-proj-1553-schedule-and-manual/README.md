# 1553 Schedule and Manual Example

Example that extends the previous 1553 examples to have commands sent at
scheduled intervals as well as manually. To run the example start the following
executables in different terminals in the following order:

```
cargo run --bin bus
cargo run --bin power -- 5
cargo run --bin gps -- 13
cargo run --bin bus_controller
```

Note, this example took and built upon the library code that was started in the
1553 devices example. Most of the "example specific" code lives in /app and
/bin with the rest being reusable architecture.

# Limitations/Further Thoughts
* There are a lot of spots constructing vectors. Consideration could be added
  to reduce the number of heap allocations necessary especially when doing
  empty values (maybe change data on messages to be an option?)

# Next steps
* The next project is to do a TUI display for the data the bus controller is
  receiving (and then the follow on would be to have the TUI send commands).
