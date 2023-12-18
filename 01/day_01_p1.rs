use std::fs;
use std::vec::Vec;

fn main() {
    let file_path = "day1.input";
    
    let mut score = 0;
    for line in fs::read_to_string(file_path).unwrap().lines() {
        let mut ints: Vec<u32> = Vec::new();
        for char in line.chars() {
            if char.is_digit(10) {
                ints.push(char.to_digit(10)
                              .expect("Checked if it was a digit and it wasn't?"));
            }
        }

        let first = ints.first().expect("There should be a digit");
        let last = ints.last().expect("There should be a digit");
        score += (first * 10) + last;
    }
    

    println!("Score: {}", score);

}