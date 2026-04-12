#include "adder.h"
#include <gtest/gtest.h>


TEST(AdderTests, TestAddNumbers) {
    EXPECT_EQ(add_numbers(1, 2), 3);
    EXPECT_EQ(add_numbers(5, 2), 7);
    EXPECT_EQ(add_numbers(1, 5), 6);
}

TEST(AdderTests, TestBadAdd) {
    EXPECT_EQ(bad_add(1, 2), 3);
    EXPECT_EQ(bad_add(5, 2), 7);
    EXPECT_EQ(bad_add(1, 5), 6);
}
