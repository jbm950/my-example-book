import matplotlib.pyplot as plt
import numpy as np

def main():
    x = np.array([1, 2, 3, 4, 5])
    h = np.array([1, 2, 3])

    num_samples = len(x) + (len(h) - 1)

    y = np.zeros(num_samples)

    for n in range(num_samples):
        print(f"\nn: {n}")
        for k in range(len(x)):
            h_index = n - k
            if 0 <= h_index < len(h):
                y[n] += x[k] * h[h_index]
                print(
                    f"x[{k}] * h[{h_index}] = "
                    f"{x[k]} * {h[h_index]} = {x[k] * h[h_index]}"
                )

    assert np.allclose(y, np.convolve(x, h))

    fig, axes = plt.subplots(3, 1, figsize=(10, 10), sharex=True)

    axes[0].stem(x)
    axes[0].set_title("Input")

    axes[1].stem(h)
    axes[1].set_title("Filter")

    axes[2].stem(y)
    axes[2].set_title("Output")

    plt.show()

if __name__ == "__main__":
    main()
