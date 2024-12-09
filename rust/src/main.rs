#![allow(dead_code)]

mod day8;

use day8::{read_input, part_one, part_two};

fn main() {
    let data = read_input("src/day8/input.txt");
    println!("Part one: {}", part_one(data.clone()));
}
