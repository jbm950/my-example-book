cmake_minimum_required(VERSION 3.20)

project(cpp-mpp-thread LANGUAGES CXX)

set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED)

add_executable(cpp-mp-thread
    src/main.cpp
)

target_compile_options(cpp-mp-thread PRIVATE
    -Wall
    -Wextra
    -Wpedantic
)
