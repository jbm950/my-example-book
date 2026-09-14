cmake_minimum_required(VERSION 3.20)

project(cpp-linux-page-fault-first-touch LANGUAGES CXX)

set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED)

set(CMAKE_EXPORT_COMPILE_COMMANDS ON)

add_executable(cpp-linux-page-fault-first-touch
    src/main.cpp
)

target_compile_options(cpp-linux-page-fault-first-touch PRIVATE
    -Wall
    -Wextra
    -Wpedantic
)
