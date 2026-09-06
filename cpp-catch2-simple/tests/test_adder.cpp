#include "adder.h"
#include <catch2/catch_test_macros.hpp>


TEST_CASE("add_numbers adds two positive numbers", "[adder]") {
    REQUIRE(add_numbers(1, 2) == 3);
    REQUIRE(add_numbers(5, 2) == 7);
    REQUIRE(add_numbers(1, 5) == 6);
}

TEST_CASE("bad_add should fail these tests", "[adder]") {
    REQUIRE(bad_add(1, 2) == 3);
    REQUIRE(bad_add(5, 2) == 7);
    REQUIRE(bad_add(1, 5) == 6);
}
