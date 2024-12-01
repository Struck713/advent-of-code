
use std::fs;
use std::vec::Vec;
use std::collections::HashMap;

struct Range {
    source: u64,
    destination: u64,
    range: u64
}

fn main() {
    let file_path = "day5.input";
    let file = fs::read_to_string(file_path).unwrap();
    let lines = file.lines().collect::<Vec<&str>>();

    let seeds = lines[0].split_whitespace()
        .into_iter()
        .skip(1) // skip the "seeds:" part
        .map(|s| s.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();

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

    let mut lowest_location_number: u64 = u64::MAX;
    for seed in seeds {
        let soil = process_ranges(&mut maps, "seed-to-soil map:", seed);
        let fertil = process_ranges(&mut maps, "soil-to-fertilizer map:", soil);
        let water = process_ranges(&mut maps, "fertilizer-to-water map:", fertil);
        let light = process_ranges(&mut maps, "water-to-light map:", water);
        let temp = process_ranges(&mut maps, "light-to-temperature map:", light);
        let humid = process_ranges(&mut maps, "temperature-to-humidity map:", temp);
        let location = process_ranges(&mut maps, "humidity-to-location map:", humid);
        if location < lowest_location_number {
            lowest_location_number = location;
        }
    }

    println!("Score: {lowest_location_number}");
}

fn process_ranges<'a>(maps: &mut HashMap<&'a str, Vec<Range>>, name: &'a str, seed: u64) -> u64 {
    // let mut process = seed;
    for range in maps.entry(name).or_default() {
        let seed_process = process_seed(seed, range);
        if seed_process != seed {
            return seed_process;
        }
    }
    seed
}

fn process_seed(seed: u64, range: &Range) -> u64 {
    if seed >= range.source && seed < range.source + range.range {
        range.destination + (seed - range.source)
    } else {
        seed
    }
}