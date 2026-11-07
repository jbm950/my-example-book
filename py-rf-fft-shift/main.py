import matplotlib.pyplot as plt
import numpy as np

f_s = 1_000_000  # 1 MHz sample rate
duration = 500e-6  # 500 microseconds total observation window
fft_size = int(f_s * duration)

# Frequencies are ordered from 0 Hz through positive frequencies,
# followed by negative frequencies.
bin_indices = np.arange(fft_size)
freqs_fft = np.fft.fftfreq(fft_size, d=1 / f_s)
# FFT spans from -Fs/2 to Fs/2 (or -500 kHz to 500 kHz in this example)
freqs_fft_shifted = np.fft.fftshift(freqs_fft)

print(f"Unshifted, first bin frequency: {freqs_fft[0]}, last bin frequency: {freqs_fft[-1]}")
print(f"Shifted, first bin frequency: {freqs_fft_shifted[0]}, last bin frequency: {freqs_fft_shifted[-1]}")

_, axes = plt.subplots(2, 1, figsize=(10, 10))

axes[0].plot(bin_indices, freqs_fft)
axes[0].set_title("Frequencies, No Shift")
axes[0].set_xlabel("FFT Bin Index")
axes[0].set_ylabel("Frequency (Hz)")

axes[1].plot(bin_indices, freqs_fft_shifted)
axes[1].set_title("Frequencies, Shifted")
axes[1].set_xlabel("FFT Bin Index")
axes[1].set_ylabel("Frequency (Hz)")

plt.tight_layout()
plt.show()
