cmake_minimum_required(VERSION 3.16)

project(counter LANGUAGES CXX)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

add_library(counter STATIC
    counter.cpp
)

target_include_directories(counter PUBLIC ${CMAKE_CURRENT_SOURCE_DIR})

install(TARGETS counter ARCHIVE DESTINATION lib)
