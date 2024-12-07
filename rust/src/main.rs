#![allow(dead_code)]

mod day7;
use day7::{read_input, part_one, part_two};

fn main() {
    let data = read_input("src/day7/input.txt");
    println!("Part one: {}", part_one(data.clone()));
    println!("Part two: {}", part_two(data.clone()));
}
