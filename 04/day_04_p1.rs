
use std::fs;
use std::vec::Vec;

fn main() {
    let file_path = "day4.input";
    
    let mut score = 0;
    for line in fs::read_to_string(file_path).unwrap().lines() {
        let card = line.split(": ").collect::<Vec<&str>>();
        let data = card[1].split(" | ").collect::<Vec<&str>>();
        let winning = data[0].split_whitespace().into_iter().map(|s| s.parse::<u32>().unwrap()).collect::<Vec<u32>>();
        let numbers = data[1].split_whitespace().into_iter().map(|s| s.parse::<u32>().unwrap()).collect::<Vec<u32>>();
        
        let mut card_score = 0; 
        for nums in numbers {
            if winning.iter().any(|&x| x == nums) {
                if card_score == 0 {
                    card_score = 1;
                } else {
                    card_score += card_score;
                }
            }
        }
        score += card_score;
    }

    println!("Score: {}", score);
}