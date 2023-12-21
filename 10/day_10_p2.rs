
use std::fs;
use std::vec::Vec;

fn main() {
    let file_path = "day10.input";
    let file = fs::read_to_string(file_path).unwrap();
    let data = file.lines().collect::<Vec<&str>>();

    // map out file to a 2d vector
    let matrix: Vec<Vec<char>> = data.iter()
                                     .map(|x| x.chars().collect::<Vec<char>>())
                                     .collect::<Vec<Vec<char>>>();

    let mut dots: Vec<(usize, usize)> = Vec::new();
    let mut starting_point: (usize, usize) = (0, 0);
    for i in 0..matrix.len() {
        for j in 0..matrix[i].len() {
            let symbol = matrix[i][j];
            if symbol == 'S' {
                starting_point = (j, i);
            } else if symbol == '.' {
                dots.push((j, i));
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

    dbg!(point_max, point_min);
    
    let mut score = 0;
    for dot in dots {
        if dot > point_min && dot < point_max && inside_polygon(dot, &points) {
            score += 1;
        }
    }

    for y in 0..11 {
        for x in 0..11 {
            if points.iter().any(|other| x == other.0 && y == other.1) {
                print!("x");
            } else {
                print!(".");
            }
        }
        println!();
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

// really lazy way to check this!
fn inside_polygon(point: (usize, usize), points: &Vec<(usize, usize)>) -> bool {
    let mut count_x = 0;
    for index in 0..point.0 {
        if points.iter().any(|other| index == other.0 && point.1 == other.1) {
            count_x += 1;
        }
    }
    count_x % 2 != 0
}