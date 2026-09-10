#include <chrono>
#include <condition_variable>
#include <iostream>
#include <mutex>
#include <queue>
#include <thread>

constexpr int NUM_SAMPLES = 10;

void consumer(std::condition_variable &cv, std::mutex &mtx,
              std::queue<int> &shared_queue) {
  for (int i = 0; i < NUM_SAMPLES; ++i) {
    std::unique_lock lock(mtx);
    cv.wait(lock, [&] { return !shared_queue.empty(); });

    int item = shared_queue.front();
    shared_queue.pop();

    lock.unlock();

    std::cout << "Consumer received: " << item << "\n";
  }
}

int main() {
  std::mutex mtx;
  std::condition_variable cv;
  std::queue<int> shared_queue{};

  std::thread consumer_thread(consumer, std::ref(cv), std::ref(mtx),
                              std::ref(shared_queue));

  for (int i = 0; i < NUM_SAMPLES; ++i) {
    std::cout << "Producing: " << i << "\n";
    {
      std::lock_guard lock(mtx);
      shared_queue.push(i);
    }
    cv.notify_one();
    std::this_thread::sleep_for(std::chrono::milliseconds(500));
  }

  consumer_thread.join();
}
