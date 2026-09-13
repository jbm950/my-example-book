cmake_minimum_required(VERSION 3.20)

project(cpp-linux-scheduling-preemption LANGUAGES CXX)

set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED)

set(CMAKE_EXPORT_COMPILE_COMMANDS ON)

add_executable(cpp-linux-scheduling-preemption
    src/main.cpp
)

target_compile_options(cpp-linux-scheduling-preemption PRIVATE
    -Wall
    -Wextra
    -Wpedantic
)
