import matplotlib.pyplot as plt
import numpy as np

# 1 MHz sample rate * 500 microsecond observation window = 500 FFT bins
f_s = 1_000_000  # 1 MHz sample rate
duration = 500e-6  # 500 microseconds total observation window
fft_size = int(f_s * duration)

time = np.arange(fft_size) / f_s

freq = 20_000  # 20 kHz tone

in_phase = np.cos(2 * np.pi * freq * time)
quadrature = np.sin(2 * np.pi * freq * time)

bin_spacing = f_s / fft_size

# FFT spans from -Fs/2 to Fs/2 (or -500 kHz to 500 kHz in this example)
# Frequencies are ordered from 0 Hz through positive frequencies,
# followed by negative frequencies.
freqs_fft = np.fft.fftfreq(fft_size, d=1 / f_s)
magnitude_fft = np.abs(np.fft.fft(in_phase + 1j * quadrature))

bin_index = round(freq / bin_spacing)
peak_bin = np.argmax(magnitude_fft)

print(f"FFT Size: {fft_size} bins")
print(f"Frequency Spacing: {bin_spacing} Hz")
print(f"20 kHz bin: {bin_index}")
print(f"Freq at that bin: {freqs_fft[bin_index]}")
# Magnitude is fft_size * signal amplitude. Since the signal is unscaled
# sin/cos functions, it's amplitude is just 1 for this example.
print(f"Magnitude at that bin: {magnitude_fft[bin_index]}")
print(f"Peak bin: {peak_bin}")
print(f"Magnitude at peak bin: {magnitude_fft[peak_bin]}")

_, axes = plt.subplots(2, 1, figsize=(10, 10))

axes[0].plot(time, in_phase)
axes[0].plot(time, quadrature)
axes[0].set_title("Base Band")
axes[0].set_xlabel("Time (s)")
axes[0].set_ylabel("Amplitude")

axes[1].plot(freqs_fft, magnitude_fft)
axes[1].set_title("FFT Magnitude")
axes[1].set_xlabel("Frequency (Hz)")
axes[1].set_ylabel("Magnitude")

plt.tight_layout()
plt.show()
