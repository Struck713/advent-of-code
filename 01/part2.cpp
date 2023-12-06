#include <iostream>
#include <fstream>
#include <string>
#include <vector>
#include <array>

using std::string; using std::vector; using std::array;

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

    const array<string, 10> numbers = { "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine" };

    int score = 0;
    string line;

    while (getline(file, line)) {
        vector<int> ints;
        for (int i = 0; i < line.length(); i++) {
            if (isdigit(line[i])) ints.push_back(line[i] - int('0'));
            for (int j = 0; j < numbers.size(); j++) {
                if (line.substr(i, numbers[j].length()) == numbers[j]) ints.push_back(j);
            }
        }

        score += (ints.front() * 10) + ints.back();
    }
    std::cout << score << std::endl; 
    return 0;
}