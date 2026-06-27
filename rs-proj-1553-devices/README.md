# 1553 Send Data Example

Example that sends 1553 command and status words on an Ethernet bus and those
transactions specify subaddresses and include data words. To run the example
start the following executables in different terminals in the following order:

```
cargo run --bin bus
cargo run --bin rt -- 5
cargo run --bin rt -- 13
cargo run --bin bus_controller
```

Note, this example took and built upon the library code that was started in the
1553 heartbeat example. Most of the "example specific" code lives in /app and
/bin with the rest being reusable architecture.

# Limitations/Further Thoughts
* The `protocol` module has made a lot of progress as well as the new `net`
  module.
* The `app` module could use work to make setting up the bus controller and RTs
  more modular.
    * Bus control needs to be able to react to scheduled and on demand
      transactions.
    * RTs should look into generalizing how to respond to different
      subaddresses.
* Would be nice to start considering making "realistic devices" where each has
  their own subaddress definitions (including what the data words mean) and
  remote terminals.

# Next steps
* The next project could simply be setting up the bus controller to do
  scheduled and manual transactions.
* Could also work on implementing a bus monitor that just prints out the
  messages it sees on the bus in a CLI table format.
* Lastly making actual device remote terminals would be a useful building block
  towards doing a TUI style interface over the bus controller.
