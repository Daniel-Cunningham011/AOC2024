use std::fs;
type Direction = (i32, i32);

/*
 * Technique recursive flood search
 */

// Get the last character checked (previous_char)
// Check if the char at position plus direction is in bounds
    // If so check if the character is the next character
        // If so, Flood again
        // Else, false
    // Else, False
// Count the amount of trues

fn check_bound((x_bound, y_bound) : (i32, i32), (x, y): (i32, i32)) -> bool {
    if y >= 0 && y < y_bound {
        if x >= 0 && x < x_bound {
            return true;
        }
    }
    false
}

fn check_position(arr: Vec<Vec<char>>, (x_i32, y_i32): (i32, i32), (x_dir, y_dir): Direction) -> bool {
    let xmas = "XMAS";
    let previous_char = arr[y_i32 as usize][x_i32 as usize];
    let option_index = xmas.find(previous_char);
    match option_index {
        Some(index) => {
            if index + 1 == xmas.len() {
                return true;
            }
            let (x_new, y_new) : (i32, i32) = ((x_i32 + x_dir), (y_i32 + y_dir));
            if check_bound((arr[y_i32 as usize].len() as i32, arr.len() as i32), (x_new, y_new)) {
                let next_char = arr[y_new as usize][x_new as usize];
                if next_char == xmas.chars().collect::<Vec<char>>()[index + 1] {
                    check_position(arr, (x_new, y_new), (x_dir, y_dir))
                }
                else {
                    false
                }
            } else {
                return false;
            }
        },
        None => {
            false
        }
    }
}


// Function to count up the number of XMAS in a search
fn start_search(arr: Vec<Vec<char>>, position: (i32, i32)) -> usize {
    let list: Vec<bool> = vec![
        check_position(arr.clone(), position, (-1, -1)),
        check_position(arr.clone(), position, (-1, 0)),
        check_position(arr.clone(), position, (-1, 1)),
        check_position(arr.clone(), position, (0, -1)),
        check_position(arr.clone(), position, (0, 1)),
        check_position(arr.clone(), position, (1, -1)),
        check_position(arr.clone(), position, (1, 0)),
        check_position(arr.clone(), position, (1, 1)),
    ];
    list.iter().filter(|x| *(*x) == true).count()
}

fn x_mas_check_position(arr: Vec<Vec<char>>, (x_i32, y_i32): (i32, i32), (x_dir, y_dir): Direction) -> bool {
    let mut xmas = "MS".to_string();
    // Create a new Direction (inverse of (x_dir, y_dir))
    let (inv_pos_x, inv_pos_y) = (x_i32 - x_dir, y_i32 - y_dir);
    let (x_pos_dir, y_pos_dir) = (x_i32 + x_dir, y_i32 + y_dir);
    let bounds = (arr[y_i32 as usize].len() as i32, arr.len() as i32);
    // Check arr[y_dir][x_dir] for one of the characters in xmas
        // if exists, check inverse for the other.
        // else false
    if check_bound(bounds, (x_pos_dir, y_pos_dir)) {
        let option_index = xmas.find(arr[y_pos_dir as usize][x_pos_dir as usize]);
        match option_index {
            Some(index) => {
                xmas.remove(index);
                if check_bound(bounds, (inv_pos_x, inv_pos_y)) {
                    if arr[inv_pos_y as usize][inv_pos_x as usize] == xmas.chars().collect::<Vec<char>>()[0] {
                        true
                    }
                    else {
                        false
                    }
                }
                else {
                    false
                }
            },
            None => {
                false
            }
        }
    }
    else {
        false
    }
}

fn x_mas_search(arr: Vec<Vec<char>>, position: (i32, i32)) -> usize {
    let list: Vec<bool> = vec![
        x_mas_check_position(arr.clone(), position, (1, 1)),
        x_mas_check_position(arr.clone(), position, (1, -1))
    ];
    if list.iter().filter(|x| *(*x) == false).count() > 0 {
        return 0;
    }
    1
}

fn file_to_lines(filename: &str) -> Vec<Vec<char>> {
    fs::read_to_string(filename)
        .expect("Cannot read file")
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>()
}

// Check position for X then flood
fn main() {
    let mut word_search: usize = 0;
    let mut mas_search: usize = 0;
    let input = file_to_lines("input.txt");
    for (line_index, line) in input.iter().enumerate() {
        for (c_index, c_elem) in (*line).iter().enumerate() {
            let pos = (c_index as i32, line_index as i32);
            if (*c_elem) == 'X' {
                word_search += start_search(input.clone(), pos);
            }

            if (*c_elem) == 'A' {
                mas_search += x_mas_search(input.clone(), pos);
            }
        }
    }

    println!("Word Search: {}\nX-MAS Search: {}", word_search, mas_search);
}
