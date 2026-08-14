Example program that captures a self generated PWM signal to showcase the timer
capture functionality of the STM32. This builds on the previous PAC example by
moving the capture of the value from a polling loop to interrupts. Note this
example relies exclusively on the PAC rather than the HAL to support learning
the low level implementation
