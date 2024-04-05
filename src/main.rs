// mod day1;
// mod day2;
// mod day3;
mod day4;

use std::fs::read_to_string;
use std::time::Instant;

fn main() {
    let input = read_lines("./src/inputs/puzzle4.txt");

    let now = Instant::now();
    day4::solve2(input);
    let end = now.elapsed();

    println!("{:.2?}", end)
}

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }

    result
}
