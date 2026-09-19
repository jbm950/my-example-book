cmake_minimum_required(VERSION 3.20)

project(cpp-perf-false-sharing LANGUAGES CXX)

set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED)

set(CMAKE_EXPORT_COMPILE_COMMANDS ON)

add_executable(cpp-perf-false-sharing
    src/main.cpp
)

target_compile_options(cpp-perf-false-sharing PRIVATE
    -Wall
    -Wextra
    -Wpedantic
)
