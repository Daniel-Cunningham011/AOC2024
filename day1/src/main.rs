
use std::fs;

fn file_to_lists(filename: &str) -> (Vec<i32>, Vec<i32>) {
    let data: Vec<String> = fs::read_to_string(filename)
    .expect("Unable to read file")
    .lines()
    .map(String::from)
    .collect();

    let (left, right) : (Vec<i32>, Vec<i32>) = data.iter().map(|a: &String| -> (i32, i32) {
        let i32vec: Vec<i32> = a.split_whitespace().map(|word: &str| -> i32 {
            word.parse().expect("Cannot parse word into number")
        }).collect();
        (i32vec[0], i32vec[1])
    }).unzip();

    (left, right)
}

fn find_diff(left: &mut Vec<i32>, right: &mut Vec<i32>) -> i32 {
    left.sort();
    right.sort();

    left.iter().zip(right.iter()).map(|a: (&i32, &i32)| -> i32 {
        let (first, second) = a;
        (first - second).abs()
    }).rfold(0, |acc, x| acc + x)
}

fn sim_score(left: &mut Vec<i32>, right: &mut Vec<i32>) -> i32 {
    left.iter().map(|x: &i32| -> i32 {
        let amount = right.iter().filter(|y| *y == x).count();
        x * (amount as i32)
    }).rfold(0, |acc,z| acc + z)
}

fn main() {
    let (mut left, mut right) = file_to_lists("input.txt");
    let diff = find_diff(&mut left.clone(), &mut right.clone());
    let sim_score: i32 = sim_score(&mut left, &mut right);

    println!("Diff: {}, Similarity Score: {}", diff.to_string(), sim_score.to_string())

}
