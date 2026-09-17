#include <algorithm>
#include <chrono>
#include <cstddef>
#include <iostream>
#include <sys/resource.h>
#include <unistd.h>

constexpr std::size_t NUM_PAGES = 10'000;

using Clock = std::chrono::steady_clock;

void loop_pages(long page_size, std::byte *buffer, std::byte set_val) {

  rusage usage_before{};
  rusage usage_after{};

  getrusage(RUSAGE_SELF, &usage_before);

  auto total_time = Clock::duration::zero();
  auto max_time = Clock::duration::zero();

  for (std::size_t page = 0; page < NUM_PAGES; ++page) {
    auto start = Clock::now();

    buffer[page * page_size] = set_val;

    auto elapsed = Clock::now() - start;

    max_time = std::max(max_time, elapsed);
    total_time += elapsed;
  }

  getrusage(RUSAGE_SELF, &usage_after);

  auto minor_faults = usage_after.ru_minflt - usage_before.ru_minflt;
  auto major_faults = usage_after.ru_majflt - usage_before.ru_majflt;

  std::cout << "Total: "
            << std::chrono::duration<double, std::micro>(total_time).count()
            << " us\n";
  std::cout << "Max: "
            << std::chrono::duration<double, std::micro>(max_time).count()
            << " us\n";

  std::cout << "Minor Faults: " << minor_faults << "\n";
  std::cout << "Major Faults: " << major_faults << "\n";
}

int main() {
  long page_size = sysconf(_SC_PAGESIZE);
  if (page_size == -1) {
    return 1;
  }

  std::size_t buffer_size = NUM_PAGES * page_size;
  auto *buffer = new std::byte[buffer_size];

  // First touch: establish a warm working set
  for (std::size_t page = 0; page < NUM_PAGES; ++page) {
    buffer[page * page_size] = std::byte(0);
  }

  std::cout << "\nFirst pass pages are warm\n";
  loop_pages(page_size, buffer, std::byte{0xFF});

  // Memory pressure
  // Found experimentally to create pressure on my laptop
  const std::size_t pressure_multiplier = 750;
  const std::size_t pressure_pages = NUM_PAGES * pressure_multiplier;
  const std::size_t pressure_buffer_size = pressure_pages * page_size;
  auto *pressure_buffer = new std::byte[pressure_buffer_size];

  for (std::size_t page = 0; page < pressure_pages; ++page) {
      pressure_buffer[page * page_size] = std::byte{0xFF};
  }

  // Memory pressure may have reclaimed the original pages
  std::cout << "\nSecond pass pages are likely reclaimed\n";
  loop_pages(page_size, buffer, std::byte{0xAA});

  delete[] buffer;
  delete[] pressure_buffer;
}
