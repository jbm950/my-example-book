Simple example that shows how to use PWM on the Raspberry Pi. Note the Pi had
to be set up independently to have the PWM peripheral enabled and was set to
GPIO18, pin 12. The following command was used for the logic analyzer to prove
PWM is working.

```bash
sigrok-cli --driver fx2lafw --channels D0 --config samplerate=8m --time 1s -o ~/pwm.sr
```
