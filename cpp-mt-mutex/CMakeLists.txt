cmake_minimum_required(VERSION 3.20)

project(cpp-mpp-mutex LANGUAGES CXX)

set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED)

set(CMAKE_EXPORT_COMPILE_COMMANDS ON)

add_executable(cpp-mp-mutex
    src/main.cpp
)

target_compile_options(cpp-mp-mutex PRIVATE
    -Wall
    -Wextra
    -Wpedantic
)
