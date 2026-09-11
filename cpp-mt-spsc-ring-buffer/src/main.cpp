#include <atomic>
#include <cstddef>
#include <iostream>
#include <thread>

constexpr int NUM_SAMPLES = 10;
constexpr std::size_t CAPACITY = 8;

std::size_t next_index(std::size_t current_index) {
  return (current_index + 1) % CAPACITY;
}

void consumer(std::atomic<std::size_t> &read_idx,
              std::atomic<std::size_t> &write_idx, int *buffer) {

  std::size_t samples_received{0};

  while (samples_received < NUM_SAMPLES) {

    auto current_read = read_idx.load(std::memory_order_relaxed);
    auto current_write = write_idx.load(std::memory_order_acquire);

    bool buffer_empty = (current_read == current_write);
    if (buffer_empty) {
      continue;
    }

    int value = buffer[current_read];

    read_idx.store(next_index(current_read), std::memory_order_release);

    std::cout << "Consumer received: " << value << "\n";

    samples_received += 1;
  }
}

int main() {
  int buffer[CAPACITY];
  // Separate producer and consumer indices to avoid false sharing.
  alignas(64) std::atomic<std::size_t> write_idx{0};
  alignas(64) std::atomic<std::size_t> read_idx{0};

  std::thread consumer_thread(consumer, std::ref(read_idx), std::ref(write_idx),
                              buffer);

  std::size_t samples_written{0};
  while (samples_written < NUM_SAMPLES) {

    auto current_write = write_idx.load(std::memory_order_relaxed);
    auto next_write = next_index(current_write);

    auto current_read = read_idx.load(std::memory_order_acquire);

    bool buffer_full = (next_write == current_read);
    if (buffer_full) {
      continue;
    }

    std::cout << "Producing: " << samples_written << "\n";

    buffer[current_write] = samples_written;
    write_idx.store(next_write, std::memory_order_release);

    samples_written += 1;
  }

  consumer_thread.join();
}
