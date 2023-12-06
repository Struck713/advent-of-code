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

    int totalPart1 = 0;
    int totalPart2 = 0;
    string line;

    while (getline(file, line)) {

        // PART 1
        int rightmost = 0, leftmost = 0;
        for (int i = 0; i < line.length(); i++) {
            if (isdigit(line[i])) {
                rightmost = line[i] - int('0');
                break;
            }
        }
        for (int i = line.length() - 1; i >= 0; i--) {
            if (isdigit(line[i])) {
                leftmost = line[i] - int('0');
                break;
            }
        }
        // std::cerr << line << " => " << ((rightmost * 10) + leftmost) << std::endl;
        totalPart1 += (rightmost * 10) + leftmost;

        int rightmost = 0, leftmost = 0;
        for (int i = 0; i < line.length(); i++) {
            if (isdigit(line[i])) {
                rightmost = line[i] - int('0');
                break;
            }
        }
        for (int i = line.length() - 1; i >= 0; i--) {
            if (isdigit(line[i])) {
                leftmost = line[i] - int('0');
                break;
            }
        }
        // std::cerr << line << " => " << ((rightmost * 10) + leftmost) << std::endl;
        totalPart2 += (rightmost * 10) + leftmost;


    }
    std::cout << "Part 1: " << totalPart1 << std::endl; 

    return 0;
}