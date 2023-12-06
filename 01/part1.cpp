#include <iostream>
#include <fstream>
#include <string>
#include <vector>

using std::string; using std::vector;

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

    int score = 0;
    string line;

    while (getline(file, line)) {

        vector<int> ints;

        // PART 1
        for (int i = 0; i < line.length(); i++) {
            if (isdigit(line[i])) ints.push_back(line[i] - int('0'));
        }

        score += (ints.front() * 10) + ints.back();
    }
    std::cout << score << std::endl; 

    return 0;
}