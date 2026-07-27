Simple USART example that showcases sending commands to the board and using
those to control and query the state of the LED.

Note, the read byte section had to drop down to the PAC due to a possible bug
in the HAL where overrun errors were not able to be cleared directly.
