mod hot_springs;

use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let input: Vec<&str> = input.lines().collect();

    println!("{}", puzzle_one(&input));
    println!("{}", puzzle_two(&input));
}

fn puzzle_one(input: &[&str]) -> u32 {
    0
}

fn puzzle_two(input: &[&str]) -> u32 {
    0
}