Simple blinky program that flashes the middle LED of the microbit v2 LED matrix
every second and a half, this time relying on timer interrupts rather than a
busy loop delay function. Note this example relies exclusively on the PAC
rather than the HAL to support learning the low level implementation
