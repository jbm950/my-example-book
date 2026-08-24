Simple example to turn on a TT motor using the TB6612FNG motor driver. Note,
for this example, a wall power supply set to 12V was used. Also, this example
implements the on sequence using a TB6612FNG crate found on crates.io rather
than manually driving the GPIO pins.

Interconnect:
```
                                                         power supply 
                                   Motor Driver          ┌──────────┐ 
       Nucleo F446                  TB6612FNG            │    12 V  │ 
 ┌─────────────────────┐     ┌─────────────────────┐     │          │ 
 │     PWM : PA5 (D13) ┼─────┼ PWMA             Vm ┼─────┼ Vout     │ 
 │                     │     │                     │     │          │ 
 │  A IN 1 : PA6 (D12) ┼─────┼ A IN 1       Ground ┼─────┼ Ground   │ 
 │                     │     │                     │     └──────────┘ 
 │  A IN 2 : PA7 (D11) ┼─────┼ A IN 2              │                  
 │                     │     │                     │                  
 │  Standby: PB6 (D10) ┼─────┤ Standby             │        Motor     
 │                     │     │                     │     ┌──────────┐ 
 │                 3V3 ┼─────┼ Vcc         A OUT 1 ┼─────┼ Power +  │ 
 │                     │     │                     │     │          │ 
 │              Ground ┼─────┼ Ground      A OUT 2 ┼─────┼ Power -  │ 
 └─────────────────────┘     └─────────────────────┘     └──────────┘ 
                                                  Made using ASCII Flow
```
