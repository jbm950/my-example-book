cmake_minimum_required(VERSION 3.20)

project(cpp-mt-spsc-ring-buffer LANGUAGES CXX)

set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED)

set(CMAKE_EXPORT_COMPILE_COMMANDS ON)

add_executable(cpp-mt-spsc-ring-buffer
    src/main.cpp
)

target_compile_options(cpp-mt-spsc-ring-buffer PRIVATE
    -Wall
    -Wextra
    -Wpedantic
)
