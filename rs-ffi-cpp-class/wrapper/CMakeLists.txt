add_library(wrapper STATIC
    counter.cpp
)

target_link_libraries(wrapper PRIVATE counter)

install(TARGETS wrapper ARCHIVE DESTINATION lib)
