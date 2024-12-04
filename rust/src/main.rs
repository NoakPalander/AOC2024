#![allow(dead_code)]

mod day4;
use day4::{read_input, part_one, part_two};

fn main() {
    let data = read_input("src/day4/input.txt");

    println!("Part one: {}", part_one(data.clone()));
    println!("Part two: {}", part_two(data.clone()));
}
