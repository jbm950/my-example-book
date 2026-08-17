Example that exercises the timer encoder mode to determine position and
direction information from a quadrature signal (quadrature generated on the
same MCU and looped back. Note this example attempts to rely on the HAL as much
as possible but the HAL doesn't have an API for a quadrature generator and the
encoder API has to be grabbed from the version 0.2 backport module as it was
dropped in version 1.0.

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
