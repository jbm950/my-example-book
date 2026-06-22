# 1553 Heartbeat Example
Example that sends 1553 command and status words on an Ethernet bus. Note, need
to start the executables in the following order so that everything connects
correctly:

```
cargo run --bin bus
cargo run --bin rt -- 5
cargo run --bin rt -- 13
cargo run --bin bus_controller
```

# Limitations/Further Thoughts
* The read logic needs to be refined to:
    * Keep reading the socket until the correct number of bytes have been read
      (TCP could split the messages up).
    * Monitor and process the command word to know how many status/data words
      to skip reading. Right now the reads are on every word to assume it's a
      status word.
* The StatusWord and CmdWord structs did not end up needing to convert to/from
  u16. A better conversion would be [u8; 2] because that's what's actually
  needed.
* The CmdWord could use validations on the fields when being built and throw
  errors if there are invalid inputs.
* Add a `new` method for subaddress to simplify construction and match the
  scheme for CmdWord.

# Next steps
* The most basic next step would be to have an example where data words are
  being sent back and forth (receive and transmit). Maybe just make a
  heartbeat-with-data. Would probably be good to differentiate subaddresses
  with it too.
