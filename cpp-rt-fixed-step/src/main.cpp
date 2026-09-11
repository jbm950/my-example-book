#include <chrono>
#include <iostream>

using Clock = std::chrono::steady_clock;

constexpr auto FRAME_PERIOD = std::chrono::milliseconds(10);
const double DT = std::chrono::duration<double, std::milli>(FRAME_PERIOD).count();
constexpr int NUM_STEPS = 5;

class Sim {
public:
  void update(double dt) {
    velocity_ += acceleration_ * dt;
    position_ += velocity_ * dt;

    std::cout << "dt: " << dt << " velocity: " << velocity_
              << " position: " << position_ << "\n";
  }

private:
  constexpr static double acceleration_ = 0.5;
  double velocity_{0};
  double position_{0};
};

int main() {

  // Example 1
  std::cout << "Unpaced Fixed Timestep ---------------------\n";

  auto sim1_start = Clock::now();
  double sim1_time = 0.0;

  Sim sim1{};
  for (int i = 0; i < NUM_STEPS; ++i) {
    sim1.update(DT);
    sim1_time += DT;
  }

  auto sim1_end = Clock::now();
  std::chrono::duration<double, std::milli> sim1_duration =
      sim1_end - sim1_start;

  std::cout << "Wall time: " << sim1_duration << "\n";
  std::cout << "Sim time: " << sim1_time << "ms\n";

  // Example 2
  std::cout << "\nVariable Timestep + pacing -----------------\n";

  Sim sim2{};
  double sim2_time = 0.0;
  auto next_frame = Clock::now();
  auto prev_frame = next_frame;

  auto sim2_start = Clock::now();
  for (int i = 0; i < NUM_STEPS; ++i) {
    next_frame += FRAME_PERIOD;

    auto frame_start = Clock::now();
    while (frame_start < next_frame) {
      frame_start = Clock::now();
    }

    std::chrono::duration<double, std::milli> dt = frame_start - prev_frame;

    sim2.update(dt.count());
    sim2_time += dt.count();
    prev_frame = frame_start;
  }

  auto sim2_end = Clock::now();
  std::chrono::duration<double, std::milli> sim2_duration =
      sim2_end - sim2_start;

  std::cout << "Wall time: " << sim2_duration << "\n";
  std::cout << "Sim time: " << sim2_time << "ms\n";

  // Example 3
  std::cout << "\nFixed Timestep + accumulator -----------------\n";

  Sim sim3{};
  double accumulator = 0.0;
  double sim3_time = 0.0;

  auto sim3_start = Clock::now();
  auto prev_time = sim3_start;
  unsigned int steps_taken = 0;
  while (steps_taken < NUM_STEPS) {
    while (accumulator < DT) {
      auto current_time = Clock::now();
      std::chrono::duration<double, std::milli> elapsed =
          current_time - prev_time;
      accumulator += elapsed.count();
      prev_time = current_time;
    }

    while (accumulator >= DT) {
      sim3.update(DT);
      sim3_time += DT;
      accumulator -= DT;
      ++steps_taken;
    }
  }

  auto sim3_end = Clock::now();
  std::chrono::duration<double, std::milli> sim3_duration =
      sim3_end - sim3_start;

  std::cout << "Wall time: " << sim3_duration << "\n";
  std::cout << "Sim time: " << sim3_time << "ms\n";
}
