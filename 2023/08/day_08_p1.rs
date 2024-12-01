
use std::fs;
use std::vec::Vec;
use std::collections::HashMap;

fn main() {
    let file_path = "day8.input";
    let file = fs::read_to_string(file_path).unwrap();
    
    let data = file.lines().collect::<Vec<&str>>();
    let input = data[0];

    let mut map: HashMap<&str, (&str, &str)> = HashMap::new();
    let mut index = 2;
    while index < data.len() {
        let entry = data[index];
        let entry_key_value = entry.split(" = ").collect::<Vec<&str>>();
        
        let key = entry_key_value[0];
        let left = &entry_key_value[1][1..4];
        let right = &entry_key_value[1][6..9];
        
        map.insert(key, (left, right));
        index += 1;
    }
    
    let mut current = "AAA";
    let mut index: usize = 0;
    let mut score: u64 = 0;
    loop {
        if index >= input.len() {
            index = 0;
        }

        if current == "ZZZ" {
            break;
        }
        
        let instruction = &input[index..index + 1];
        let (left, right) = map.get(current).unwrap();
        if instruction == "L" {
            current = left;
        } else {
            current = right;
        }

        score += 1;
        index += 1;
    }

    println!("Score: {score}");
}