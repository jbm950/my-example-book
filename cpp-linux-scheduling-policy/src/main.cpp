#include <chrono>
#include <iostream>
#include <pthread.h>
#include <sched.h>
#include <system_error>
#include <thread>
#include <unistd.h>

using Clock = std::chrono::steady_clock;

constexpr auto STEP_PERIOD = std::chrono::milliseconds(10);
constexpr int NUM_STEPS = 500;
constexpr int PINNED_CPU = 2;

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

void worker(int id, int policy, int priority) {
  pin_to_cpu(PINNED_CPU);
  set_scheduler(policy, priority);

  auto start = Clock::now();

  for (int i = 0; i < NUM_STEPS; ++i) {
    auto target = start + (i * STEP_PERIOD);
    while (Clock::now() < target) {
    };
    std::cout << id << std::flush;
  }
}

int main() {
  std::cout << "Default Policy -------------------\n";
  std::thread thread_0(worker, 0, SCHED_OTHER, 0);
  std::thread thread_1(worker, 1, SCHED_OTHER, 0);

  thread_0.join();
  thread_1.join();

  std::cout << "\n\nFIFO Policy -------------------\n";
  std::thread thread_2(worker, 0, SCHED_FIFO, 50);
  std::thread thread_3(worker, 1, SCHED_FIFO, 50);

  thread_2.join();
  thread_3.join();

  std::cout << "\n\nRound Robin Policy -------------------\n";
  std::thread thread_4(worker, 0, SCHED_RR, 50);
  std::thread thread_5(worker, 1, SCHED_RR, 50);

  thread_4.join();
  thread_5.join();

  std::cout << "\n";
}
