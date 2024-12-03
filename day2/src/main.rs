
use std::fs;

fn file_to_lines(filename: &str) -> Vec<String> {
    fs::read_to_string(filename)
        .expect("Cannot read file")
        .lines()
        .map(String::from)
        .collect()
}

fn check_line(line: &mut Vec<i32>) -> bool {
    let diff = line[1] - line[0];
    if diff < 0 {
        return check_line_decreasing(line, false);
    }
    else {
        return check_line_increasing(line, false);
    }
}

fn check_line_decreasing(line: &mut Vec<i32>, removed: bool) -> bool {
    for i in 0..(line.len() - 1) {
        let diff = line[i + 1] - line[i];
        if diff >= 0 || diff.abs() > 3 {
            
        }
    }
    true
}

fn check_line_increasing(line: &mut Vec<i32>, removed: bool) -> bool {
    for i in 0..(line.len() - 1) {
        let diff = line[i + 1] - line[i];
        if diff <= 0 || diff.abs() > 3 {
            
        }
    }
    true
}


fn main() {
    let line_input: Vec<Vec<i32>> = file_to_lines("input.txt").into_iter().map(|line: String| -> Vec<i32> {
        line.split_whitespace().map(|word: &str| (*word).parse().expect("Cannnot parse string to i32")).collect()
        }).collect();
    println!("Safe Reports: {}", line_input.into_iter().map(|mut line| check_line(&mut line)).filter(|x| (*x) == true).count());

}
