
use std::fs;
use std::vec::Vec;
use std::time::SystemTime;
use std::collections::HashMap;

fn main() {
    let program_start = SystemTime::now();
    
    let file_path = "day8.input";
    let file = fs::read_to_string(file_path).unwrap();
    
    let data = file.lines().collect::<Vec<&str>>();
    let input = data[0];
    
    let mut nodes: Vec<&str> = Vec::new();
    let mut map: HashMap<&str, (&str, &str)> = HashMap::new();
    let mut index = 2;
    while index < data.len() {
        let entry = data[index];
        let entry_key_value = entry.split(" = ").collect::<Vec<&str>>();
        
        let key = entry_key_value[0];
        if key.ends_with("A") {
            nodes.push(key);
        } 

        let left = &entry_key_value[1][1..4];
        let right = &entry_key_value[1][6..9];
        
        map.insert(key, (left, right));
        index += 1;
    }
    
    let mut index: usize = 0;
    let mut score: u64 = 0;
    let size = nodes.len();
    loop {
        if index >= input.len() {
            index = 0;
        }

        if nodes.iter().all(|x| x.ends_with("Z")) {
            break;
        }

        let instruction = &input[index..index + 1];
        if instruction == "L" {
            for node_index in 0..size {
                let (left, _) = map.get(nodes[node_index]).unwrap();
                nodes[node_index] = left;
            }
        } else {
            for node_index in 0..size {
                let (_, right) = map.get(nodes[node_index]).unwrap();
                nodes[node_index] = right;
            }
        }

        score += 1;
        index += 1;
    }

    println!(":: Score: {score}");
    if let Ok(elapsed) = program_start.elapsed() {
        println!(":: Program finished in {}s", elapsed.as_secs());
    }
}