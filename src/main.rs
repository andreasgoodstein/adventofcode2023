mod day1;

use std::fs::read_to_string;
use std::time::Instant;

fn main() {
    let input = read_lines("./src/puzzles/puzzle1.txt");

    let now = Instant::now();
    day1::solve2(input);
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
