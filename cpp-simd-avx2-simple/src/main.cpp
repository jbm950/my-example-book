#include <array>
#include <chrono>
#include <cstddef>
#include <immintrin.h>
#include <iostream>

using Clock = std::chrono::steady_clock;

constexpr std::size_t NUM_ELEMENTS = 1'000'000;

int main() {
  static std::array<float, NUM_ELEMENTS> x{};
  static std::array<float, NUM_ELEMENTS> y{};
  static std::array<float, NUM_ELEMENTS> z{};

  for (std::size_t i = 0; i < NUM_ELEMENTS; ++i) {
    x[i] = static_cast<float>(i);
    y[i] = static_cast<float>(i + 1);
  }

  std::chrono::time_point<Clock> start;

  start = Clock::now();
  for (std::size_t i = 0; i < NUM_ELEMENTS; ++i) {
    z[i] = x[i] * 2 + y[i]; // May be auto vectorized
  }
  std::chrono::duration<double, std::milli> scalar_elapsed = Clock::now() - start;

  std::cout << "Scalar calculation (may be auto-vectorized): "
            << scalar_elapsed << "\n";

  const __m256 x_scalar = _mm256_set1_ps(2.0f);
  start = Clock::now();
  for (std::size_t i = 0; i < NUM_ELEMENTS; i += 8) {
    __m256 x_vec = _mm256_loadu_ps(&x[i]);
    __m256 y_vec = _mm256_loadu_ps(&y[i]);

    // z = x*2 + y
    __m256 z_vec = _mm256_add_ps(
      _mm256_mul_ps(x_vec, x_scalar),
      y_vec
    );

    _mm256_storeu_ps(&z[i], z_vec);
  }
  std::chrono::duration<double, std::milli> simd_elapsed = Clock::now() - start;

  std::cout << "SIMD calculation: " << simd_elapsed << "\n";
}
