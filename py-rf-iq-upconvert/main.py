import matplotlib.pyplot as plt
import numpy as np

f_s = 1_000_000  # 1 MHz sample rate
f_bb = 20_000  # 20 kHz baseband tone
f_c = 200_000  # 200 kHz "carrier"

duration = 500e-6  # 500 microseconds total observation window

time = np.arange(0, duration, 1 / f_s)

# Create a pulse rather than continuous wave. Done because projects are working
# towards pulsed radar learning.
pulse_start = 200e-6
pulse_width = 100e-6
envelope = np.where(
    (time >= pulse_start) & (time < pulse_start + pulse_width), 1.0, 0.0
)

in_phase = np.cos(2 * np.pi * f_bb * time) * envelope
quadrature = np.sin(2 * np.pi * f_bb * time) * envelope

freqs_bb = np.fft.fftshift(np.fft.fftfreq(len(in_phase), d=1 / f_s))
magnitude_bb = np.abs(np.fft.fftshift(np.fft.fft(in_phase + 1j * quadrature)))

signal = in_phase * np.cos(2 * np.pi * f_c * time) - quadrature * np.sin(
    2 * np.pi * f_c * time
)

freqs_sig = np.fft.fftshift(np.fft.fftfreq(len(signal), d=1 / f_s))
magnitude_sig = np.abs(np.fft.fftshift(np.fft.fft(signal)))

_, axes = plt.subplots(4, 1, figsize=(10, 10))

axes[0].plot(time, in_phase)
axes[0].plot(time, quadrature)
axes[0].set_title("Base Band)")
axes[0].set_xlabel("Time (s)")

axes[1].plot(freqs_bb, magnitude_bb)
axes[1].set_title("Base Band)")
axes[1].set_xlabel("Frequency (Hz)")

axes[2].plot(time, signal)
axes[2].set_title("Upconverted Signal")
axes[2].set_xlabel("Time (s)")

axes[3].plot(freqs_sig, magnitude_sig)
axes[3].set_title("Upconverted Signal)")
axes[3].set_xlabel("Frequency (Hz)")

plt.tight_layout()
plt.show()
