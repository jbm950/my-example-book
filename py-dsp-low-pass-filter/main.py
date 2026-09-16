# Good reference: https://tomroelandts.com/articles/how-to-create-a-simple-low-pass-filter

import matplotlib.pyplot as plt
import numpy as np

sample_rate = 1000  # Hz
num_samples = 1000
times = np.arange(num_samples) / sample_rate

low_freq = 50
high_freq = 250

low_freq_sig = np.sin(2 * np.pi * low_freq * times)
high_freq_sig = np.sin(2 * np.pi * high_freq * times)

combined_signal = low_freq_sig + high_freq_sig

freq_cutoff = 100
normalized_cutoff = freq_cutoff / sample_rate

filter_length = 50
filter_idxs = np.arange(filter_length)
sinc_filter = np.sinc(2 * normalized_cutoff * (filter_idxs - (filter_length - 1) / 2))
blackman_window = (
    0.42
    - 0.5 * np.cos(2 * np.pi * filter_idxs / (filter_length - 1))
    + 0.08 * np.cos(4 * np.pi * filter_idxs / (filter_length - 1))
)  # Could use np.blackman(filter_length)

low_pass_coefs = sinc_filter * blackman_window
low_pass_coefs = low_pass_coefs / low_pass_coefs.sum()

filtered_sig = np.convolve(combined_signal, low_pass_coefs, mode="same")

fft_original = np.abs(np.fft.rfft(combined_signal))
fft_filtered = np.abs(np.fft.rfft(filtered_sig[:len(combined_signal)]))
freqs = np.fft.rfftfreq(len(combined_signal), 1 / sample_rate)

_, axes = plt.subplots(4, 1, figsize=(10, 10))

axes[0].plot(combined_signal)
axes[0].set_title("Original Signal")

axes[1].stem(low_pass_coefs)
axes[1].set_title("Coefficients")

axes[2].plot(filtered_sig)
axes[2].set_title("Filtered Signal")

axes[3].plot(freqs, fft_original, label="Original")
axes[3].plot(freqs, fft_filtered, label="Filtered")
axes[3].set_title("Frequency Spectrum")
axes[3].set_xlabel("Frequency (Hz)")
axes[3].legend()

plt.tight_layout()
plt.show()
