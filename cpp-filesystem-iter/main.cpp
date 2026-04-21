#include <iostream>
#include <filesystem>

namespace fs = std::filesystem;

int main() {
    const fs::path top_path { fs::current_path() };

    for (const auto& entry : fs::directory_iterator(top_path)) {
        const auto dir_name_str = entry.path().filename().string();
        if (entry.is_directory()) {
            std::cout << "Dir: " << dir_name_str << std::endl;
        } else if (entry.is_regular_file()) {
            std::cout << "File: " << dir_name_str << std::endl;
        } else {
            std::cout << "???: " << dir_name_str << std::endl;
        }
    }
}
