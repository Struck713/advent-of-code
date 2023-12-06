#include <iostream>
#include <fstream>
#include <string>

using std::string;

int main(int argc, char** argv) {
    if (argc != 2) {
        std::cout << "Usage: " << argv[0] << " <file>"  << std::endl;
        std::cout << "ERROR: no input file provided." << std::endl;
        return 1;
    }

    std::ifstream file(argv[1]);
    if (!file) {
        std::cout << "Usage: " << argv[0] << " <file>"  << std::endl;
        std::cout << "ERROR: could not open input file." << std::endl;
        return 1;
    }

    string line;
    while (getline(file, line)) {
    }

    return 0;
}