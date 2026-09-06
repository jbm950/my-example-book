#include <iostream>
#include <mutex>
#include <thread>
#include <vector>

constexpr int NUM_THREADS = 30;
constexpr int NUM_INCREMENTS = 100'000;
constexpr int EXPECTED_COUNT = NUM_THREADS * NUM_INCREMENTS;

static std::mutex my_mutex;
static int counter_no_mutex{0}; // Intentional data race for demonstration
static int counter_mutex{0};

void task_no_mutex() {
  for (int i = 0; i < NUM_INCREMENTS; ++i) {
    ++counter_no_mutex;
  }
}

void task_mutex() {
  for (int i = 0; i < NUM_INCREMENTS; ++i) {
    std::lock_guard<std::mutex> lock(my_mutex);
    ++counter_mutex;
  }
}

int main() {
  std::vector<std::thread> threads;
  threads.reserve(NUM_THREADS * 2);

  for (int i = 0; i < NUM_THREADS; ++i) {
    threads.emplace_back(task_no_mutex);
    threads.emplace_back(task_mutex);
  }

  for (auto &thread : threads) {
    thread.join();
  }

  std::cout << "Expected: " << EXPECTED_COUNT
            << ", No Mutex: " << counter_no_mutex
            << ", With Mutex: " << counter_mutex << "\n";
}
