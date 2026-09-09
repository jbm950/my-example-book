#include <chrono>
#include <condition_variable>
#include <cstddef>
#include <iostream>
#include <mutex>
#include <thread>
#include <vector>

int main() {
  std::mutex mutex;
  std::condition_variable cv;

  // Regular usage
  std::thread worker1([&] {
    std::cout << "Worker waiting\n";

    std::unique_lock lock(mutex);
    // Relies on the sleep ordering of the demo to ensure the wait is hit
    // before notify is called.
    cv.wait(lock);

    std::cout << "Worker woke up!\n";
  });

  std::this_thread::sleep_for(std::chrono::seconds(1));

  std::cout << "Sending notification\n";
  cv.notify_one();

  worker1.join();

  // Usage with a predicate
  bool ready{false};

  std::thread worker2([&] {
    std::cout << "Worker waiting\n";

    std::unique_lock lock(mutex);
    cv.wait(lock, [&] { return ready; });

    std::cout << "Worker woke up!\n";
  });

  std::this_thread::sleep_for(std::chrono::seconds(1));

  {
    std::lock_guard lock(mutex);
    ready = true;
  }

  std::cout << "Sending notification\n";
  cv.notify_one();

  worker2.join();

  // Notifying a collection of threads
  const std::size_t num_threads{10};
  std::vector<std::thread> threads;
  threads.reserve(num_threads);

  for (std::size_t i = 0; i < num_threads; i++) {
    threads.emplace_back([&, i] {
      std::cout << "Thread " << i << " waiting\n";

      std::unique_lock lock(mutex);
      cv.wait(lock);

      std::cout << "Thread " << i << " woke up!\n";
    });
  }

  std::this_thread::sleep_for(std::chrono::seconds(1));

  std::cout << "Waking one thread\n";
  cv.notify_one();

  std::this_thread::sleep_for(std::chrono::seconds(1));

  std::cout << "Waking remaining threads\n";
  cv.notify_all();

  for (auto &thread : threads) {
    thread.join();
  }
}
