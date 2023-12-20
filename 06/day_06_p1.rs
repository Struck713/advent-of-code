
use std::fs;
use std::vec::Vec;

fn main() {
    let file_path = "day6.input";
    let file = fs::read_to_string(file_path).unwrap();
    
    let data = file.lines().collect::<Vec<&str>>();
    let times = data[0].split_whitespace().into_iter().skip(1).map(|s| s.parse::<u32>().unwrap()).collect::<Vec<u32>>();
    let distances = data[1].split_whitespace().into_iter().skip(1).map(|s| s.parse::<u32>().unwrap()).collect::<Vec<u32>>();

    let mut score = 1;
    for index in 0..times.len() {
        let mut individual_score = 0;
        let distance_to_beat = distances[index];
        let time_of_race = times[index];
        for time_held in 0..time_of_race + 1 {
            let distance = time_held * (time_of_race - time_held);
            if distance > distance_to_beat {
                individual_score += 1;
            }
        }
        score *= individual_score;
    }

    println!("Score: {score}");
}