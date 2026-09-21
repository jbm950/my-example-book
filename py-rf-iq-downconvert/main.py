import matplotlib.pyplot as plt
import numpy as np
from scipy.signal import butter, filtfilt

# Upconvert taken from previous project
f_c = 200_000  # 200 kHz "carrier"
f_bb = 20_000  # 20 kHz baseband tone
f_s = 1_000_000  # 1 MHz sample rate

duration = 500e-6  # 500 microseconds total observation window
time = np.arange(0, duration, 1 / f_s)

# Create a pulse rather than continuous wave. Done because projects are
# working towards pulsed radar learning.
pulse_start = 200e-6
pulse_width = 100e-6
envelope = np.where(
    (time >= pulse_start) & (time < pulse_start + pulse_width), 1.0, 0.0
)

in_phase = np.cos(2 * np.pi * f_bb * time) * envelope
quadrature = np.sin(2 * np.pi * f_bb * time) * envelope

rx_signal = in_phase * np.cos(2 * np.pi * f_c * time) - quadrature * np.sin(
    2 * np.pi * f_c * time
)

# Downconvert
candidate_in_phase = rx_signal * np.cos(2 * np.pi * f_c * time)
candidate_quadrature = rx_signal * -1 * np.sin(2 * np.pi * f_c * time)

freqs_bb = np.fft.fftshift(np.fft.fftfreq(len(candidate_in_phase), d=1 / f_s))
magnitude_bb = np.abs(
    np.fft.fftshift(np.fft.fft(candidate_in_phase + 1j * candidate_quadrature))
)

cutoff = 50_000  # Hz, above f_bb and well below the image at ~400+ kHz
order = 4
b, a = butter(order, cutoff, fs=f_s, btype="low")

in_phase_filtered = 2 * filtfilt(b, a, candidate_in_phase)
quadrature_filtered = 2 * filtfilt(b, a, candidate_quadrature)

# Plotting
_, axes = plt.subplots(2, 1, figsize=(10, 10))

axes[0].plot(freqs_bb, magnitude_bb)
axes[0].set_title("Candidate I/Q Spectrum")
axes[0].set_xlabel("Frequency (Hz)")

axes[1].plot(time, in_phase, label="I")
axes[1].plot(time, quadrature, label="Q")
axes[1].plot(time, in_phase_filtered, label="I-Filtered")
axes[1].plot(time, quadrature_filtered, label="Q-Filtered")
axes[1].set_title("Filtered I/Q")
axes[1].set_xlabel("Time (s)")
axes[1].legend()

plt.tight_layout()
plt.show()
