#include <chrono>
#include <iostream>
#include <pthread.h>
#include <sched.h>
#include <system_error>
#include <thread>
#include <unistd.h>

using Clock = std::chrono::steady_clock;
using Ms = std::chrono::duration<double, std::milli>;

constexpr auto STEP_PERIOD = std::chrono::milliseconds(10);
constexpr int NUM_STEPS = 500;
constexpr int PINNED_CPU = 2;
constexpr int RT_PRIORITY = 80;

void pin_to_cpu(int cpu) {
  cpu_set_t cpu_set;
  CPU_ZERO(&cpu_set);
  CPU_SET(cpu, &cpu_set);

  // 0 => operate on the calling thread (NPTL maps pthreads to kernel PIDs)
  if (sched_setaffinity(0, sizeof(cpu_set), &cpu_set) != 0) {
    throw std::system_error(errno, std::generic_category(),
                            "sched_setaffinity");
  }
}

void set_scheduler(int policy, int priority) {
  sched_param parameters{};
  parameters.sched_priority = priority;

  int result = pthread_setschedparam(pthread_self(), policy, &parameters);

  if (result != 0) {
    throw std::system_error(result, std::generic_category(),
                            "pthread_setschedparam");
  }
}

void instrumented_worker() {
  pin_to_cpu(PINNED_CPU);
  set_scheduler(SCHED_FIFO, RT_PRIORITY);

  double max_release_latency = 0.0;
  double total_release_latency = 0.0;

  auto start = Clock::now();
  for (int i = 0; i < NUM_STEPS; ++i) {
    auto scheduled_release = start + (i * STEP_PERIOD);

    auto actual_release = Clock::now();
    while (actual_release < scheduled_release) {
      actual_release = Clock::now();
    }

    double release_latency = Ms(actual_release - scheduled_release).count();
    total_release_latency += release_latency;

    if (release_latency > max_release_latency) {
      max_release_latency = release_latency;
    }

  }

  double avg_latency = total_release_latency / NUM_STEPS;

  std::cout << "Avg Release Latency: " << avg_latency << "ms\n" 
            << "Max Release Latency: " << max_release_latency << "ms\n"
            << "Total Release Latency: " << total_release_latency << "ms\n";
}

void busy_worker() {
  pin_to_cpu(PINNED_CPU);

  auto start = Clock::now();
  for (int i = 0; i < NUM_STEPS; ++i) {
    auto scheduled_release = start + (i * STEP_PERIOD);
    while (Clock::now() < scheduled_release) {
    }

    asm volatile("" ::: "memory");
  }
}

int main() {
  std::cout << "\n\nNo Contention -------------------\n";
  std::thread thread_0(instrumented_worker);

  thread_0.join();

  std::cout << "\n\nWith Contention -------------------\n";
  std::thread thread_1(instrumented_worker);
  std::thread thread_2(busy_worker);

  thread_1.join();
  thread_2.join();

  std::cout << "\n";
}
