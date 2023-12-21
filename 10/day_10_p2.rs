
use std::fs;
use std::vec::Vec;
use std::collections::HashSet;

fn main() {
    let file_path = "day10.input";
    let file = fs::read_to_string(file_path).unwrap();
    let data = file.lines().collect::<Vec<&str>>();

    // map out file to a 2d vector
    let matrix: Vec<Vec<char>> = data.iter()
                                     .map(|x| x.chars().collect::<Vec<char>>())
                                     .collect::<Vec<Vec<char>>>();

    let mut starting_point: (usize, usize) = (0, 0);
    for i in 0..matrix.len() {
        for j in 0..matrix[i].len() {
            if  matrix[i][j] == 'S' {
                starting_point = (j, i);
                break;
            }
        }
    } 

    // make big polygon (basically)
    let mut points: Vec<(usize, usize)> = Vec::new();
    let mut point = (starting_point.0, starting_point.1);
    let mut prev_point = (starting_point.0, starting_point.1);

    loop {
        if point == starting_point && points.len() != 0 {
            break;
        }
        
        points.push(point);

        let next_move = find_next_move(&matrix, point, prev_point);
        prev_point = point;
        point = next_move;
    }

    let mut point_max = (0usize, 0usize);
    let mut point_min = (usize::MAX, usize::MAX);

    for point in &points {
        if point > &point_max {
            point_max = *point;
        }
        if point < &point_min {
            point_min = *point;
        }
    }    

    // super slow algo!! hopefully that means it works
    let mut score = 0;
    for i in 0..matrix.len() {
        let mut count = 0;
        for j in 0..matrix[i].len() {
            let dot = (j, i);
            let symbol = matrix[i][j];
            if points.iter().any(|other| &dot == other) {
                if symbol =='S' || symbol == 'J' || symbol == 'L' || symbol == '|' {
                    count += 1;
                }
                continue;
            }
            if count % 2 != 0 {
                score += 1;
            }
            // if counting && dot > point_min && dot < point_max {
            //     score += 1;
            // }
        }
    } 

    println!("Score: {}", score); 
}

// this could be cleaned up but it works!
// tuple is (x, y, corner)
fn find_next_move(matrix: &Vec<Vec<char>>, point: (usize, usize), prev_point: (usize, usize)) -> (usize, usize) {
    let (curr_x, curr_y) = point;
    let (prev_x, prev_y) = prev_point;
    match matrix[curr_y][curr_x] {
        'S' => (curr_x + 1, curr_y),
        'F' => (if prev_y.wrapping_sub(1) == curr_y { curr_x + 1 } else { curr_x }, if prev_x.wrapping_sub(1) == curr_x { curr_y + 1 } else { curr_y }),
        '7' => (if prev_y.wrapping_sub(1) == curr_y { curr_x.wrapping_sub(1) } else { curr_x }, if prev_x + 1 == curr_x { curr_y + 1 } else { curr_y }),
        'J' => (if prev_y + 1 == curr_y { curr_x.wrapping_sub(1) } else { curr_x }, if prev_x + 1 == curr_x { curr_y.wrapping_sub(1) } else { curr_y }),
        'L' => (if prev_y + 1 == curr_y { curr_x + 1 } else { curr_x }, if prev_x.wrapping_sub(1) == curr_x { curr_y.wrapping_sub(1) } else { curr_y }),
        '-' => (if prev_x + 1 == curr_x { curr_x + 1 } else { curr_x - 1 }, curr_y),
        '|' => (curr_x, if prev_y + 1 == curr_y { curr_y + 1 } else { curr_y - 1 }),
        _ => (0, 0)
    }
}