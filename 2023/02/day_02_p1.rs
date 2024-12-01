use std::fs;
use std::vec::Vec;

fn parse_int_and_unit(string: &str) -> (u32, &str) {
    let parts = string.split(" ").collect::<Vec<&str>>();
    let num = parts[0];
    let unit = parts[1];
    (num.parse::<u32>().expect("Checked if it was a digit and it wasn't?"), unit)
}

fn main() {

    const RED: u32 = 12;
    const GREEN: u32 = 13;
    const BLUE: u32 = 14;

    let file_path = "day2.input";
    
    let mut score = 0;
    let mut index = 1;
    for line in fs::read_to_string(file_path).unwrap().lines() {
        let game = line.split(": ").collect::<Vec<&str>>();

        let mut possible = true;
        for subset in game[1].split("; ") {
            for item in subset.split(", ") {
                let (num, unit) = parse_int_and_unit(item);
                match unit {
                    "green" => {
                        if num > GREEN {
                            possible = false;
                            break;
                        }
                    },
                    "blue" => {
                        if num > BLUE {
                            possible = false;
                            break;
                        }
                    },
                    "red" => {
                        if num > RED {
                            possible = false;
                            break;
                        }
                    },
                    _ => {
                        possible = false;
                        break;
                    }
                }
            }
        }
        
        if possible {
            score += index;
        }

        index += 1;
    }
    

    println!("Score: {}", score);

}