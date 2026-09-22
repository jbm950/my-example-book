import matplotlib.pyplot as plt
import numpy as np

freq = 10_000  # 10 kHz baseband tone
sample_frequencies = [  # Hz
    30_000,
    25_000,
    20_000,  # Nyquist boundary
    18_000,
    15_000
]

duration = 500e-6  # 500 microseconds total observation window

continuous_time = np.linspace(0, duration, 5000)
continuous_signal = np.cos(2 * np.pi * freq * continuous_time)

plt.plot(continuous_time, continuous_signal, linestyle="--", alpha=0.5)

for sample_frequency in sample_frequencies:
    time = np.arange(0, duration, 1 / sample_frequency)  # Need different time vectors per sample rate
    signal = np.cos(2 * np.pi * freq * time)
    plt.plot(time, signal, "o-", label=f"fs = {sample_frequency/ 1000:g} kHz")

plt.xlabel("Time (s)")
plt.legend()
plt.tight_layout()
plt.show()

