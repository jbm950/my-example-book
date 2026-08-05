#include <iostream>

extern "C" void hello_from_cpp(void) {
    std::cout << "Hello from C++!\n";
}
