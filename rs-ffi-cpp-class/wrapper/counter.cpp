#include "counter.h"

extern "C" Counter* counter_new(int initial_value) {
    return new Counter(initial_value);
}

extern "C" void counter_increment(Counter* counter) {
    counter->increment();
}

extern "C" void counter_increment_by(Counter* counter, int amount) {
    counter->increment_by(amount);
}

extern "C" int counter_value(Counter* counter) {
    return counter->value();
}

extern "C" void counter_delete(Counter* counter) {
    delete counter;
}
