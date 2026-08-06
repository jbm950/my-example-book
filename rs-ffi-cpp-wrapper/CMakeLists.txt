cmake_minimum_required(VERSION 3.16)

project(rust_cpp_wrapper LANGUAGES CXX)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

add_subdirectory(ext/proj1)
add_subdirectory(wrapper)
