Simple example program that shows the minimal amount of code needed to create
and flash to the Nucleo F446RE board.

memory.x - Defines the MCU's flash and RAM layout for the linker.
.cargo/config - Selects the target and tells Cargo how to flash the board.

Note: Unlike the nRF52833 used in the micro:bit examples, cargo embed was
unable to automatically identify the STM32F446RE. An Embed.toml specifying chip
= "STM32F446RE" was required for cargo embed, while cargo run worked via the
--chip argument in .cargo/config.toml.
