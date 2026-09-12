#include <chrono>
#include <iostream>

using Clock = std::chrono::steady_clock;
using Ms = std::chrono::duration<double, std::milli>;

constexpr auto FRAME_PERIOD = std::chrono::milliseconds(10);
constexpr int NUM_STEPS = 500;
constexpr int FORCE_OVERRUN_INTERVAL = 43;
constexpr auto OVERRUN_DURATION = std::chrono::milliseconds(15);
static_assert(OVERRUN_DURATION > FRAME_PERIOD,
              "Forced overrun should be longer than the frame period");

void do_work(int iteration) {
  // Force overrun every FORCE_OVERRUN_INTERVAL iterations
  if (iteration > 0 && iteration % FORCE_OVERRUN_INTERVAL == 0) {
    auto start = Clock::now();
    while (Clock::now() < (start + OVERRUN_DURATION)) {
    }
  }
}

int main() {
  unsigned int deadline_misses = 0;
  double max_release_latency = 0.0;
  double max_execution_time = 0.0;
  double max_lateness = 0.0;

  auto sim_start = Clock::now();
  for (int i = 0; i < NUM_STEPS; ++i) {
    auto scheduled_release = sim_start + (i * FRAME_PERIOD);
    auto deadline = scheduled_release + FRAME_PERIOD;

    auto actual_release = Clock::now();
    while (actual_release < scheduled_release) {
      actual_release = Clock::now();
    }

    double release_latency = Ms(actual_release - scheduled_release).count();

    do_work(i);

    auto completion_time = Clock::now();

    if (release_latency > max_release_latency) {
      max_release_latency = release_latency;
    }

    double execution_time = Ms(completion_time - actual_release).count();
    if (execution_time > max_execution_time) {
      max_execution_time = execution_time;
    }

    if (completion_time > deadline) {
      ++deadline_misses;

      double lateness = Ms(completion_time - deadline).count();
      if (lateness > max_lateness) {
        max_lateness = lateness;
      }

      std::cout << "Iteration: " << i << ", Execution Time: " << execution_time
                << " Lateness: " << lateness << "\n";
    }
  }

  std::cout << "Max Release Jitter: " << max_release_latency << "ms\n"
            << "Max Execution Time: " << max_execution_time << "ms\n"
            << "Deadline Misses: " << deadline_misses << "\n"
            << "Max Lateness: " << max_lateness << "ms\n";
}
