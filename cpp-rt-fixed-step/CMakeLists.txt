cmake_minimum_required(VERSION 3.20)

project(cpp-rt-fixed-step LANGUAGES CXX)

set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED)

set(CMAKE_EXPORT_COMPILE_COMMANDS ON)

add_executable(cpp-rt-fixed-step
    src/main.cpp
)

target_compile_options(cpp-rt-fixed-step PRIVATE
    -Wall
    -Wextra
    -Wpedantic
)
