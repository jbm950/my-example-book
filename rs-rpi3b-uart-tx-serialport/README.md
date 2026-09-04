Simple example showing sending data over Raspberry Pi UART with GPIO pins. Pins
GPIO14 (tx) and GPIO15(rx) are tied together forming a loopback. The following
command is run in another terminal to verify the result:

`cat /dev/serial0`
