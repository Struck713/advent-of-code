
use std::fs;
use std::vec::Vec;
use std::collections::HashMap;

struct Hand {
    card_map: HashMap<char, u8>,
    cards: String,
    bid: usize
}

fn main() {
    let file_path = "day7.input";
    let file = fs::read_to_string(file_path).unwrap();

    let mut hands: Vec<Hand> = Vec::new();
    for line in file.lines() {
        let hand_parts = line.split_whitespace().collect::<Vec<&str>>();

        let mut card_map = HashMap::new();
        let mut jokers = 0;
        for card in hand_parts[0].chars() {
            if card == 'J' { 
                jokers += 1;
            } else {
                card_map.entry(card)
                    .and_modify(|e| *e += 1)
                    .or_insert(1);
            }
        }

        if let Some((max_key, _)) = card_map.iter().max_by_key(|entry | entry.1) {
            card_map.entry(*max_key)
                .and_modify(|e| *e += jokers)
                .or_insert(jokers);
        } else {
            card_map.insert('J', jokers);
        }
        

        hands.push(Hand {
            cards: hand_parts[0].chars().collect(),
            card_map: card_map,
            bid: hand_parts[1].parse::<usize>().unwrap()
        });
    }

    hands.sort_by(|a, b| {
        let mut score_a: u8 = rate_map(&a);
        let mut score_b: u8 = rate_map(&b);

        if score_a == score_b {
            let mut index = 0;
            while score_a == score_b {
                score_a = get_score_for_character(a.cards.as_bytes()[index] as char);
                score_b = get_score_for_character(b.cards.as_bytes()[index] as char);
                index += 1;
            }
        }

        return score_a.cmp(&score_b);
    });

    let mut score = 0;
    let mut index = 0;
    while index < hands.len() {
        let hand = &hands[index];
        score += (index + 1) * hand.bid;
        println!("{index}. {} => {}", hand.cards, rate_map(&hand));
        index += 1;
    }

    println!("Score: {score}");
}

fn rate_map(hand: &Hand) -> u8 {
    if find_keys_for_value(&hand.card_map, 5) == 1 {
        return 6;
    }
    if find_keys_for_value(&hand.card_map, 4) == 1 {
        return 5;
    }
    if find_keys_for_value(&hand.card_map, 3) == 1 {
        if find_keys_for_value(&hand.card_map, 2) == 1 {
            return 4;
        }
        return 3;
    }
    let twos = find_keys_for_value(&hand.card_map, 2);
    if twos == 2 {
        return 2;
    }
    if twos == 1 {
        return 1;
    }
    0
}

// thank you: https://stackoverflow.com/questions/59401720/how-do-i-find-the-key-for-a-value-in-a-hashmap
fn find_keys_for_value<'a>(map: &'a HashMap<char, u8>, value: u8) -> usize {
    map.iter()
        .filter_map(|(key, &val)| if val == value { Some(key) } else { None })
        .count()
}

fn get_score_for_character(character: char) -> u8 {
    match character {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'T' => 10,
        '9' => 9,
        '8' => 8,
        '7' => 7,
        '6' => 6,
        '5' => 5,
        '4' => 4,
        '3' => 3,
        '2' => 2,
        'J' => 1,
        _ => 0
    }
}