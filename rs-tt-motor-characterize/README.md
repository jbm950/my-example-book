Example builds on the measurement example to sweep the different pwm duty
cycles and record the output speed. Note, for this example, a wall power supply
set to 9V was used. When 12V was attempted, the output speed reading dropped
off after ~70% duty despite the motor continuing to spin faster.

Interconnect:
```
                                                          power supply  
                                    Motor Driver          ┌──────────┐  
        Nucleo F446                  TB6612FNG            │    12 V  │  
 ┌──────────────────────┐     ┌─────────────────────┐     │          │  
 │      PWM : PA5 (D13) ┼─────┼ PWMA             Vm ┼─────┼ Vout     │  
 │                      │     │                     │     │          │  
 │   A IN 1 : PA6 (D12) ┼─────┼ A IN 1       Ground ┼─────┼ Ground   │  
 │                      │     │                     │     └──────────┘  
 │   A IN 2 : PA7 (D11) ┼─────┼ A IN 2              │                   
 │                      │     │                     │                   
 │  Standby : PB6 (D10) ┼─────┤ Standby             │        Motor      
 │                      │     │                     │     ┌────────────┐
 │                  3V3 ┼─────┼ Vcc         A OUT 1 ┼─────┼ Power +    │
 │                      │     │                     │     │            │
 │               Ground ┼─────┼ Ground      A OUT 2 ┼─────┼ Power -    │
 │                      │     └─────────────────────┘     │            │
 │                   5V ┼─────────────────────────────────┼ 5V Encoder │
 │                      │                                 │            │
 │               Ground ┼─────────────────────────────────┼ Ground     │
 │                      │                                 │            │
 │ Sig A Cap : PB4 (D5) ┼─────────────────────────────────┼ Sig A      │
 └──────────────────────┘                                 └────────────┘
                                                  Made using ASCII Flow
```
