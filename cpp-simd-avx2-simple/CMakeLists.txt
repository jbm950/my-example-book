cmake_minimum_required(VERSION 3.20)

project(cpp-simd-avx2-simple LANGUAGES CXX)

set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED)

set(CMAKE_EXPORT_COMPILE_COMMANDS ON)

add_executable(cpp-simd-avx2-simple
    src/main.cpp
)

target_compile_options(cpp-simd-avx2-simple PRIVATE
    -mavx2
    -Wall
    -Wextra
    -Wpedantic
)
