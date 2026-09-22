import adi

sdr = adi.Pluto("ip:192.168.2.1")

print(f"RX LO: {sdr.rx_lo} Hz")
print(f"RX Sample Rate: {sdr.sample_rate} Hz")
print(f"RX RF Bandwidth: {sdr.rx_rf_bandwidth} Hz")
print(f"RX Gain Mode: {sdr.gain_control_mode_chan0}")
