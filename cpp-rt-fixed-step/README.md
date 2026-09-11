Simple example showing 3 different ways simulation time can be done.

1. Run at CPU clock speed, no relation to real time.
2. Attempt to pace real time. The time steps are subject to jitter rather than
   being deterministic.
3. Fixed simulation time steps that pace real time using an accumulator.
