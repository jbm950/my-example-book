Simple example program that shows the minimal amount of code needed to create
and flash to the Microbit v2 board.


memory.x - Defines the MCU's flash and RAM layout for the linker.
.cargo/config - Selects the target and tells Cargo how to flash the board.

Note: this example does not include a Hardware Abstraction Layer crate nor a
Board Support Package crate. If one were included, the memory.x file can
probably be omitted as those crates would provide it.
