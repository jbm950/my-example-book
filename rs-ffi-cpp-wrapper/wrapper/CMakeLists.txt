add_library(wrapper STATIC
    proj1.cpp
)

target_include_directories(wrapper PRIVATE
    ${CMAKE_CURRENT_SOURCE_DIR}/../ext/proj1
)

target_link_libraries(wrapper PRIVATE hello)

install(TARGETS wrapper ARCHIVE DESTINATION lib)
