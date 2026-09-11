cmake_minimum_required(VERSION 3.20)

project(cpp-mt-memory-ordering LANGUAGES CXX)

set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED)

set(CMAKE_EXPORT_COMPILE_COMMANDS ON)

add_executable(cpp-mt-memory-ordering
    src/main.cpp
)

target_compile_options(cpp-mt-memory-ordering PRIVATE
    -Wall
    -Wextra
    -Wpedantic
)
