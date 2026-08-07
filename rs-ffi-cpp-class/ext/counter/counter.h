#pragma once

class Counter {
public:
    explicit Counter(int initial_value);

    void increment();
    void increment_by(int amount);

    int value() const;

private:
    int value_;
};
