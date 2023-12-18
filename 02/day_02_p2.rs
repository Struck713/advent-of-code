use std::fs;
use std::vec::Vec;

fn parse_int_and_unit(string: &str) -> (u32, &str) {
    let parts = string.split(" ").collect::<Vec<&str>>();
    let num = parts[0];
    let unit = parts[1];
    (num.parse::<u32>().expect("Checked if it was a digit and it wasn't?"), unit)
}

fn main() {

    let file_path = "day2.input";
    
    let mut score = 0;
    for line in fs::read_to_string(file_path).unwrap().lines() {
        let game = line.split(": ").collect::<Vec<&str>>();

        let mut red: u32 = 1;
        let mut green: u32 = 1;
        let mut blue: u32 = 1;

        for subset in game[1].split("; ") {
            for item in subset.split(", ") {
                let (num, unit) = parse_int_and_unit(item);
                match unit {
                    "green" => {
                        if num > green {
                            green = num;
                        }
                    },
                    "blue" => {
                        if num > blue {
                            blue = num;
                        }
                    },
                    "red" => {
                        if num > red {
                            red = num;
                        }
                    },
                    _ => break
                }
            }
        }

        score += red * green * blue;
    }
    

    println!("Score: {}", score);

}