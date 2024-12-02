
use std::fs;

fn main() {
    let data: Vec<String> = fs::read_to_string("input.txt")
        .expect("Unable to read file")
        .lines()
        .map(String::from)
        .collect();
    
    let (mut left, mut right) : (Vec<i32>, Vec<i32>) = data.iter().map(|a: &String| -> (i32, i32) {
        let i32vec: Vec<i32> = a.split_whitespace().map(|word: &str| -> i32 {
            word.parse().expect("Cannot parse word into number")
        }).collect();
        (i32vec[0], i32vec[1])
    }).unzip();

    left.sort();
    right.sort();

    let diff= left.iter().zip(right.iter()).map(|a: (&i32, &i32)| -> i32 {
        let (first, second) = a;
        (first - second).abs()
    }).rfold(0, |acc, x| acc + x);
    println!("{}", diff.to_string())

}
