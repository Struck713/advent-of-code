
use std::fs;
use std::vec::Vec;
use std::collections::HashMap;

struct Range {
    source: u64,
    destination: u64,
    range: u64
}

fn main() {
    let file_path = "day5.simple.input";
    let file = fs::read_to_string(file_path).unwrap();
    let lines = file.lines().collect::<Vec<&str>>();

    let mut seeds: Vec<(u64, u64, bool)> = Vec::new();
    let seeds_raw = lines[0].split_whitespace()
        .into_iter()
        .skip(1) // skip the "seeds:" part
        .map(|s| s.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();

    let mut index = 0;
    while index < seeds_raw.len() {
        seeds.push((seeds_raw[index], seeds_raw[index + 1], false));
        index += 2;
    }

    let mut maps: HashMap<&str, Vec<Range>> = HashMap::new();
    let mut index = 1;
    let mut section_header = "";
    while index < lines.len() {
        let line = lines[index];

        if line.is_empty() {
            section_header = lines[index + 1];
            index += 2;
            continue;
        }

        let parts = line.split_whitespace().into_iter().map(|s| s.parse::<u64>().unwrap()).collect::<Vec<u64>>();
        let range = Range {
            source: parts[1],
            destination: parts[0],
            range: parts[2]
        };
        match maps.get_mut(&section_header) {
            Some(ranges) => {
                ranges.push(range);
            }
            None => {
                maps.insert(section_header, vec![range]);
            }
        }
        index += 1;
    }

    process_ranges(&mut maps, &mut seeds, "seed-to-soil map:");
    process_ranges(&mut maps, &mut seeds, "soil-to-fertilizer map:");
    process_ranges(&mut maps, &mut seeds, "fertilizer-to-water map:");
    process_ranges(&mut maps, &mut seeds, "water-to-light map:");
    process_ranges(&mut maps, &mut seeds, "light-to-temperature map:");
    process_ranges(&mut maps, &mut seeds, "temperature-to-humidity map:");
    process_ranges(&mut maps, &mut seeds, "humidity-to-location map:");
    
    let mut lowest_location_number: u64 = u64::MAX;
    for seed in seeds {
        if seed.0 < lowest_location_number {
            lowest_location_number = seed.0;
        }
    }

    println!("Score: {lowest_location_number}");
}

fn process_ranges<'a>(maps: &mut HashMap<&'a str, Vec<Range>>, seeds: &mut Vec<(u64, u64, bool)>, name: &'a str) {
    // let mut process = seed;
    let mut index = 0;
    while index < seeds.len() {
        let seed = seeds[index];
        for range in maps.entry(name).or_default() {
            if seed.0 >= range.source && seed.0 < range.source + range.range {
                // positive: seed.1 > range.range
                // 0: seed.1 <= range.range
                
                let diff = seed.1.saturating_sub(range.range); 
                if diff == 0 {
                    seeds[index] = (range.destination + (seed.0 - range.source), seed.1, true);
                    break;
                } else {
                    seeds[index] = (range.destination + (seed.0 - range.source), diff, true);
                    seeds.push((range.source + range.range, diff, false));
                    break;
                }
            }
        }
        
        index += 1;
    }

    for index in 0..seeds.len() {
        seeds[index].2 = false;
    }
    
}