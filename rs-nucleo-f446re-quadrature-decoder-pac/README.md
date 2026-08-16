Example that exercises the timer encoder mode to determine position and
direction information from a quadrature signal (quadrature generated on the
same MCU and looped back. Note this example relies exclusively on the PAC
rather than the HAL to support learning the low level implementation

**Interconnect**

```
    Nucleo F446            
┌────────────────────┐     
│  Generator (TIM2)  │     
│ Signal A: PA5 (D13)┼────┐
│                    │    │
│  Signal B: PA1 (A1)┼──┐ │
│                    │  │ │
│                    │  │ │
│   Encoder (TIM1)   │  │ │
│  Signal B: PA9 (D8)┼──┘ │
│                    │    │
│  Signal A: PA8 (D7)├────┘
└────────────────────┘     
```
