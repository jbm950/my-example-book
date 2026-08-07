#include "counter.h"

Counter::Counter(int initial_value) : value_(initial_value) {}

void Counter::increment() {
    ++value_;
}

void Counter::increment_by(int amount) {
    value_ += amount;
}

int Counter::value() const {
    return value_;
}
