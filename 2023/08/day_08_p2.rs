
use std::fs;
use std::vec::Vec;
use std::time::SystemTime;
use std::collections::HashMap;

fn main() {    
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
    
    // This video was super helpful to solve this: https://www.youtube.com/watch?v=_nnxLcrwO_U
    let mut nums: Vec<u64> = Vec::new();
    for mut node in nodes {
        let mut index: usize = 0;
        let mut steps: u64 = 0;
        while steps == 0 || !node.ends_with("Z") {
            steps += 1;
            
            let (left, right) = map.get(&node).unwrap();
            node = if &input[index..index+1] == "L" { left } else { right };
            index = if index + 1 >= input.len() { 0 } else { index + 1 };
        }
        
        nums.push(steps);
    }
    
    let mut score: u64 = nums.pop().unwrap();
    for num in nums {
        score = (score * num) / gcd(score, num);
    }

    println!("Score: {score}");
}

// https://gist.github.com/victor-iyi/8a84185c1d52419b0d4915a648d5e3e1
pub fn gcd(mut n: u64, mut m: u64) -> u64 {
    assert!(n != 0 && m != 0);
    while m != 0 {
      if m < n {
        std::mem::swap(&mut m, &mut n);
      }
      m %= n;
    }
    n
}