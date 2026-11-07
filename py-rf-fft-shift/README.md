Example that showcases that FFT ordering starts at 0, goes towards max
frequency, wraps to min frequency then goes back towards zero. When fftshift is
applied, the ordering goes from min to max frequency as one might initially
expect.
