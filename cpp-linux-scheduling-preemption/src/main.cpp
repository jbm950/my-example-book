#include <chrono>
#include <condition_variable>
#include <iostream>
#include <mutex>
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

void worker(const char *name, int policy, int priority, std::mutex &mutex,
            std::condition_variable &condition_variable) {
  pin_to_cpu(PINNED_CPU);
  set_scheduler(policy, priority);

  {
    std::unique_lock lock(mutex);
    condition_variable.wait(lock);
  }

  auto prev = Clock::now();
  for (int i = 0; i < NUM_STEPS; ++i) {
    auto target = prev + STEP_PERIOD;
    auto current = Clock::now();
    while (current < target) {
        current = Clock::now();
    }
    std::cout << name << std::flush;
    prev = current;
  }
}

int main() {
  std::mutex low_mutex;
  std::condition_variable low_cv;
  std::thread low_thread(worker, "L", SCHED_FIFO, 30, std::ref(low_mutex),
                         std::ref(low_cv));

  std::mutex medium_mutex;
  std::condition_variable medium_cv;
  std::thread medium_thread(worker, "M", SCHED_FIFO, 50, std::ref(medium_mutex),
                            std::ref(medium_cv));

  std::mutex high_mutex;
  std::condition_variable high_cv;
  std::thread high_thread(worker, "H", SCHED_FIFO, 80, std::ref(high_mutex),
                          std::ref(high_cv));

  // Initial sleep to ensure low thread hits the wait before notify. Could be
  // done with a predicate but using the sleep to keep the example simple.
  std::this_thread::sleep_for(std::chrono::seconds(1));

  // Note, this example leaves out predicates for simplicity but that makes it
  // susceptible to spurrious wake ups.
  low_cv.notify_one();
  std::this_thread::sleep_for(std::chrono::seconds(1));

  medium_cv.notify_one();
  std::this_thread::sleep_for(std::chrono::seconds(1));

  high_cv.notify_one();
  std::this_thread::sleep_for(std::chrono::seconds(1));

  high_thread.join();
  medium_thread.join();
  low_thread.join();

  std::cout << "\n";
}
