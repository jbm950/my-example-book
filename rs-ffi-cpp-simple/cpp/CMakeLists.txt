cmake_minimum_required(VERSION 3.16)

project(hello LANGUAGES CXX)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

add_library(hello STATIC
    hello.cpp
)

install(TARGETS hello ARCHIVE DESTINATION lib)
