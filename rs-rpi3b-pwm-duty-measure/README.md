Simple example where the Pi produces a PWM signal on one pin and then captures
that signal on another pin to determine the duty cycle.

Interconnect
```
 ┌────────────────────────┐  
 │    Raspberry Pi 3B     │  
 │                        │  
 │    PWM GPIO18 (Pin 12) ┼─┐
 │                        │ │
 │Capture GPIO10 (Pin 19) ┼─┘
 └────────────────────────┘  
```
