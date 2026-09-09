#include <chrono>
#include <iostream>
#include <thread>

void task(int id, int duration) {
  for (int i = 0; i < duration; i++) {
    std::cout << "Task: " << id << " iteration: " << i << "\n";
    std::this_thread::sleep_for(std::chrono::seconds(1));
  }
}

int main() {
    std::thread t1(task, 1, 5);
    std::thread t2(task, 2, 2);
    std::thread t3(task, 3, 7);

    t1.join();
    t2.join();
    t3.join();
}
