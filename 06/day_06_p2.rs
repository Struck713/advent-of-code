
use std::fs;
use std::vec::Vec;

fn main() {
    let file_path = "day6.input";
    let file = fs::read_to_string(file_path).unwrap();
    
    let data = file.lines().collect::<Vec<&str>>();
    let time_of_race = data[0].split_whitespace()
                       .into_iter()
                       .skip(1)
                       .map(|s| String::from(s))
                       .collect::<Vec<String>>()
                       .join("")
                       .parse::<u64>()
                       .unwrap();
    let distance_to_beat = data[1].split_whitespace()
                       .into_iter()
                       .skip(1)
                       .map(|s| String::from(s))
                       .collect::<Vec<String>>()
                       .join("")
                       .parse::<u64>()
                       .unwrap();

    let mut score = 0;
    for time_held in 0..time_of_race + 1 {
        let distance = time_held * (time_of_race - time_held);
        if distance > distance_to_beat {
            score += 1;
        }
    }

    println!("Score: {score}");
}