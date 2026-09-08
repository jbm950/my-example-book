cmake_minimum_required(VERSION 3.20)

project(cpp-mpp-atomic LANGUAGES CXX)

set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED)

set(CMAKE_EXPORT_COMPILE_COMMANDS ON)

add_executable(cpp-mp-atomic
    src/main.cpp
)

target_compile_options(cpp-mp-atomic PRIVATE
    -Wall
    -Wextra
    -Wpedantic
)
