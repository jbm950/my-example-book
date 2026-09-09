#include <atomic>
#include <iostream>
#include <thread>
#include <vector>

constexpr int NUM_THREADS = 30;
constexpr int NUM_INCREMENTS = 100'000;
constexpr int EXPECTED_COUNT = NUM_THREADS * NUM_INCREMENTS;

static int counter_no_atomic{0}; // Intentional data race for demonstration
static std::atomic<int> counter_atomic{0};

void task_no_atomic() {
  for (int i = 0; i < NUM_INCREMENTS; ++i) {
    ++counter_no_atomic;
  }
}

void task_atomic() {
  for (int i = 0; i < NUM_INCREMENTS; ++i) {
    counter_atomic.fetch_add(1, std::memory_order_relaxed);
  }
}

int main() {
  std::vector<std::thread> threads;
  threads.reserve(NUM_THREADS * 2);

  for (int i = 0; i < NUM_THREADS; ++i) {
    threads.emplace_back(task_no_atomic);
    threads.emplace_back(task_atomic);
  }

  for (auto &thread : threads) {
    thread.join();
  }

  std::cout << "Expected: " << EXPECTED_COUNT
            << ", No Atomic: " << counter_no_atomic
            << ", With Atomic: " << counter_atomic << "\n";
}
