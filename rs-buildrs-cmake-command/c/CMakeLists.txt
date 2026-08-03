cmake_minimum_required(VERSION 3.16)

project(hello C)

add_library(hello STATIC
    hello.c
    math.c
)

install(TARGETS hello ARCHIVE DESTINATION lib)
