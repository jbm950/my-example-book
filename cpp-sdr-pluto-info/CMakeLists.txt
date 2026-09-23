cmake_minimum_required(VERSION 3.20)

project(cpp-sdr-pluto-info LANGUAGES CXX)

set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED)

set(CMAKE_EXPORT_COMPILE_COMMANDS ON)

find_package(PkgConfig REQUIRED)
pkg_check_modules(LIBIIO REQUIRED IMPORTED_TARGET libiio)

add_executable(cpp-sdr-pluto-info
    src/main.cpp
)
target_link_libraries(cpp-sdr-pluto-info PRIVATE PkgConfig::LIBIIO)

target_compile_options(cpp-sdr-pluto-info PRIVATE
    -Wall
    -Wextra
    -Wpedantic
)
