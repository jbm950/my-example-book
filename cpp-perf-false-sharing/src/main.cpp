#include <cerrno>
#include <chrono>
#include <cstddef>
#include <iostream>
#include <sched.h>
#include <system_error>
#include <thread>

using Clock = std::chrono::steady_clock;

constexpr std::size_t NUM_STEPS = 100'000'000;
constexpr int CPU_A = 1;
constexpr int CPU_B = 2;

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

// no synchronization needed: each thread owns a distinct member
struct CounterFalseSharing {
  std::size_t count0;
  std::size_t count1;
};

struct CounterNoFalseSharing {
  alignas(64) std::size_t count0;
  alignas(64) std::size_t count1;
};

template <typename Counter>
void run_experiment(const char* label) {
  Counter counter{};

  std::cout << label << ": Counter 0: " << &counter.count0
            << " Counter 1: " << &counter.count1 << '\n';

  std::chrono::duration<double, std::milli> elapsed0;
  std::chrono::duration<double, std::milli> elapsed1;

  std::thread thread_0([&] {
    pin_to_cpu(CPU_A);

    auto start = Clock::now();
    for (std::size_t i = 0; i < NUM_STEPS; ++i) {
      ++counter.count0;
    }
    elapsed0 = Clock::now() - start;
  });

  std::thread thread_1([&] {
    pin_to_cpu(CPU_B);

    auto start = Clock::now();
    for (std::size_t i = 0; i < NUM_STEPS; ++i) {
      ++counter.count1;
    }
    elapsed1 = Clock::now() - start;
  });

  thread_0.join();
  thread_1.join();

  std::cout << "Counter 0 incrementing took: " << elapsed0
            << ", Value: " << counter.count0 << "\n";
  std::cout << "Counter 1 incrementing took: " << elapsed1
            << ", Value: " << counter.count1 << "\n";
}

int main() {
  run_experiment<CounterFalseSharing>("False Sharing");
  run_experiment<CounterNoFalseSharing>("No False Sharing");
}
