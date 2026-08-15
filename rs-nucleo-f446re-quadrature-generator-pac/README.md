Example that produces quadrature signals where signal A leads signal B by 90
degrees. The example outputs were verified using a Lonely Binary Logic Analyzer
(see interconnect below). Note this example relies exclusively on the PAC
rather than the HAL to support learning the low level implementation

Record command:  
`sigrok-cli --driver fx2lafw --channels D0,D1 --config samplerate=8m --time 1s -o ~/pwm.sr`

**Interconnect**

```
      Nucleo F446              LB Logic Analyzer
  ┌────────────────────┐        ┌────────────┐  
  │                    │        │            │  
  │ Signal A: PA5 (D13)├────────┤Channel 0   │  
  │                    │        │            │  
  │  Signal B: PA1 (A1)├────────┼Channel 1   │  
  │                    │        │            │  
  └────────────────────┘        └────────────┘  
```
