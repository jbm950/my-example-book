#include <atomic>
#include <iostream>
#include <thread>

int main() {
  std::atomic<bool> ready{false};
  std::atomic<int> data{0};

  // Release/acquire synchronization
  std::thread acquire_worker([&] {
    while (!ready.load(std::memory_order_acquire)) {
    }
    // relaxed is fine here; the acquire above already establishes the ordering
    std::cout << "Data was: " << data.load(std::memory_order_relaxed) << "\n";
  });

  data.store(42, std::memory_order_relaxed);
  ready.store(true, std::memory_order_release);

  acquire_worker.join();

  // Relaxed: no inter-thread synchronization
  ready.store(false, std::memory_order_relaxed); // Reset
  std::thread relaxed_worker([&] {
    while (!ready.load(std::memory_order_relaxed)) {
    }
    std::cout << "Data was: " << data.load(std::memory_order_relaxed)
              << ". May be correct, but is not guaranteed\n";
  });

  data.store(35, std::memory_order_relaxed);
  ready.store(true, std::memory_order_relaxed);

  relaxed_worker.join();
}
