Simple blinky program that flashes the built in LED of the Nucleo F446RE every
second, this time relying on timer interrupts rather than a busy loop delay
function. Note this example relies exclusively on the PAC rather than the HAL
to support learning the low level implementation
