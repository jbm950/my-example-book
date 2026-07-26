Simple USART example that showcases interrupts by echoing bytes recieved back
to the sender and sending a timer message every 2 seconds. This example relies
nearly exclusively on the HAL in order to showcase that layer's level of
abstraction.

Note, the read byte section had to drop down to the PAC due to a possible bug
in the HAL where overrun errors were not able to be cleared directly.
