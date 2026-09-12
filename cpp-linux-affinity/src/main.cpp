#include <chrono>
#include <iostream>
#include <sched.h>
#include <unistd.h>
#include <vector>

using Clock = std::chrono::steady_clock;

constexpr auto STEP_PERIOD = std::chrono::milliseconds(1);
constexpr int NUM_STEPS = 10'000;
constexpr int PINNED_CPU = 2;

int pin_to_cpu(int cpu) {
  cpu_set_t cpu_set;
  CPU_ZERO(&cpu_set);
  CPU_SET(cpu, &cpu_set);

  const pid_t this_thread = 0;
  return sched_setaffinity(this_thread, sizeof(cpu_set), &cpu_set);
}

void run_example(long num_cpus) {
  std::vector<unsigned int> cpu_counts(num_cpus, 0);
  auto start = Clock::now();

  int previous_cpu = sched_getcpu();
  if (previous_cpu == -1) {
    std::cout << "Error occurred while getting CPU number.\n";
    return;
  }
  unsigned int migrations = 0;
  for (int i = 0; i < NUM_STEPS; ++i) {
    auto scheduled_release = start + (i * STEP_PERIOD);

    auto actual_release = Clock::now();
    while (actual_release < scheduled_release) {
      actual_release = Clock::now();
    }

    int current_cpu = sched_getcpu();
    if (current_cpu == -1) {
      std::cout << "Error occurred while getting CPU number.\n";
      return;
    } else if (current_cpu != previous_cpu) {
      ++migrations;
    }

    ++cpu_counts[current_cpu];
    previous_cpu = current_cpu;
  }

  std::cout << "CPU Distributions:\n";
  for (std::size_t cpu = 0; cpu < cpu_counts.size(); ++cpu) {
    std::cout << "CPU " << cpu << ": " << cpu_counts[cpu] << "\n";
  }
  std::cout << "Migrations: " << migrations << "\n\n";
}

int main() {
  long num_cpus = sysconf(_SC_NPROCESSORS_CONF);

  std::cout << "Unrestricted -------------------\n";
  run_example(num_cpus);

  std::cout << "Pinned -------------------------\n";
  if (pin_to_cpu(PINNED_CPU) != 0) {
    std::cout << "Error encountered while pinning to CPU\n";
    return 1;
  };
  run_example(num_cpus);
}
