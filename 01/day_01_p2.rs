use std::fs;
use std::vec::Vec;
use std::convert::TryFrom;
use std::cmp;

fn main() {
    let file_path = "day1.input";

    const NUMBERS: &'static [&'static str] = &[ "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine" ];
    
    let mut score = 0;
    for line in fs::read_to_string(file_path).unwrap().lines() {
        let mut ints: Vec<u32> = Vec::new();
        let mut i = 0;
        for char in line.chars() {
            if char.is_digit(10) {
                ints.push(char.to_digit(10)
                              .expect("Checked if it was a digit and it wasn't?"));
            }

            let len = line.len();
            for number in 0..10 {
                if &line[i..cmp::min(i + NUMBERS[number].len(), len)] == NUMBERS[number] {
                    ints.push(u32::try_from(number).unwrap());
                }
            }
            i += 1;
        }

        let first = ints.first().expect("There should be a digit");
        let last = ints.last().expect("There should be a digit");
        score += (first * 10) + last;
    }

    println!("Score: {}", score);

    // int score = 0;
    // string line;

    // while (getline(file, line)) {
    //     vector<int> ints;
    //     for (int i = 0; i < line.length(); i++) {
    //         if (isdigit(line[i])) ints.push_back(line[i] - int('0'));
    //         for (int j = 0; j < numbers.size(); j++) {
    //             if (line.substr(i, numbers[j].length()) == numbers[j]) ints.push_back(j);
    //         }
    //     }

    //     score += (ints.front() * 10) + ints.back();
    // }
    // std::cout << score << std::endl; 

}