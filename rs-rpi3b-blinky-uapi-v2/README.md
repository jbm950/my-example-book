Simple example showing how to interact with the Raspberry Pi 3B GPIO to blink
an LED using the Linux GPIO uAPI v2.

Interconnect
```
┌─────────────────┐              
│ Raspberry Pi 3B │              
│                 │              
│ GPIO17 (Pin 11) ┼─── +LED- ───┐
│                 │             │
│ Ground (Pin 39) ┼───Resistor──┘
└─────────────────┘   560 Ohm    
```
