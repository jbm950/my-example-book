#include <chrono>
#include <cstddef>
#include <iostream>
#include <vector>

using Clock = std::chrono::steady_clock;

constexpr std::size_t NUM_TARGETS = 1'000'000;
constexpr std::size_t NUM_ITERATIONS = 1'000;

struct Target {
  float x;
  float y;
  float z;
  float vx;
  float vy;
  float vz;
  float rcs;
};

struct Targets {
  std::vector<float> x;
  std::vector<float> y;
  std::vector<float> z;
  std::vector<float> vx;
  std::vector<float> vy;
  std::vector<float> vz;
  std::vector<float> rcs;
};

int main() {
  std::chrono::time_point<Clock, std::chrono::duration<double, std::milli>>
      start;
  std::vector<Target> aos(NUM_TARGETS);

  Targets soa{
      std::vector<float>(NUM_TARGETS), std::vector<float>(NUM_TARGETS),
      std::vector<float>(NUM_TARGETS), std::vector<float>(NUM_TARGETS),
      std::vector<float>(NUM_TARGETS), std::vector<float>(NUM_TARGETS),
      std::vector<float>(NUM_TARGETS),
  };

  // Initialize values before performance measurements
  for (std::size_t target_idx = 0; target_idx < NUM_TARGETS; ++target_idx) {
    aos[target_idx] = Target{
        .x = static_cast<float>(target_idx),
        .y = static_cast<float>(target_idx + 1),
        .z = static_cast<float>(target_idx + 2),
        .vx = 1.0F,
        .vy = 2.0F,
        .vz = 2.0F,
        .rcs = 10.0F,
    };

    soa.x[target_idx] = static_cast<float>(target_idx);
    soa.y[target_idx] = static_cast<float>(target_idx + 1);
    soa.z[target_idx] = static_cast<float>(target_idx + 2);
    soa.vx[target_idx] = 1.0F;
    soa.vy[target_idx] = 2.0F;
    soa.vz[target_idx] = 2.0F;
    soa.rcs[target_idx] = 10.0F;
  }

  // Position Update
  // AoS
  start = Clock::now();
  for (std::size_t iteration = 0; iteration < NUM_ITERATIONS; ++iteration) {
    for (std::size_t target_idx = 0; target_idx < NUM_TARGETS; ++target_idx) {
      Target &current_target = aos[target_idx];

      current_target.x += current_target.vx;
      current_target.y += current_target.vy;
      current_target.z += current_target.vz;
    }
  }
  std::chrono::duration<double, std::milli> aos_position_update =
      Clock::now() - start;

  // SoA
  start = Clock::now();
  for (std::size_t iteration = 0; iteration < NUM_ITERATIONS; ++iteration) {
    for (std::size_t target_idx = 0; target_idx < NUM_TARGETS; ++target_idx) {
      soa.x[target_idx] += soa.vx[target_idx];
      soa.y[target_idx] += soa.vy[target_idx];
      soa.z[target_idx] += soa.vz[target_idx];
    }
  }
  std::chrono::duration<double, std::milli> soa_position_update =
      Clock::now() - start;

  std::cout << "Position Update:\n    AoS: " << aos_position_update
            << "\n    SoA: " << soa_position_update << "\n";

  // Position Access Only
  // AoS
  double aos_position_only_checksum = 0.0;
  start = Clock::now();
  for (std::size_t iteration = 0; iteration < NUM_ITERATIONS; ++iteration) {
    for (std::size_t target_idx = 0; target_idx < NUM_TARGETS; ++target_idx) {
      const Target &current_target = aos[target_idx];

      aos_position_only_checksum +=
          current_target.x + current_target.y + current_target.z;
    }
  }
  std::chrono::duration<double, std::milli> aos_position_only =
      Clock::now() - start;

  // SoA
  double soa_position_only_checksum = 0.0;
  start = Clock::now();
  for (std::size_t iteration = 0; iteration < NUM_ITERATIONS; ++iteration) {
    for (std::size_t target_idx = 0; target_idx < NUM_TARGETS; ++target_idx) {
      soa_position_only_checksum +=
          soa.x[target_idx] + soa.y[target_idx] + soa.z[target_idx];
    }
  }
  std::chrono::duration<double, std::milli> soa_position_only =
      Clock::now() - start;

  std::cout << "Position Access Only:\n    AoS: " << aos_position_only
            << ", Checksum: " << aos_position_only_checksum
            << "\n    SoA: " << soa_position_only
            << ", Checksum: " << soa_position_only_checksum << "\n";

  // Whole Struct
  // AoS
  double aos_whole_struct_checksum = 0.0;
  start = Clock::now();
  for (std::size_t iteration = 0; iteration < NUM_ITERATIONS; ++iteration) {
    for (std::size_t target_idx = 0; target_idx < NUM_TARGETS; ++target_idx) {
      const Target &current_target = aos[target_idx];

      aos_whole_struct_checksum += current_target.x + current_target.y +
                                   current_target.z + current_target.vx +
                                   current_target.vy + current_target.vz +
                                   current_target.rcs;
    }
  }
  std::chrono::duration<double, std::milli> aos_whole_struct =
      Clock::now() - start;

  // SoA
  double soa_whole_struct_checksum = 0.0;
  start = Clock::now();
  for (std::size_t iteration = 0; iteration < NUM_ITERATIONS; ++iteration) {
    for (std::size_t target_idx = 0; target_idx < NUM_TARGETS; ++target_idx) {
      soa_whole_struct_checksum += soa.x[target_idx] + soa.y[target_idx] +
                                   soa.z[target_idx] + soa.vx[target_idx] +
                                   soa.vy[target_idx] + soa.vz[target_idx] +
                                   soa.rcs[target_idx];
    }
  }
  std::chrono::duration<double, std::milli> soa_whole_struct =
      Clock::now() - start;

  std::cout << "Whole Struct:\n    AoS: " << aos_whole_struct
            << ", Checksum: " << aos_whole_struct_checksum
            << "\n    SoA: " << soa_whole_struct
            << ", Checksum: " << soa_whole_struct_checksum << "\n";
}
