#![allow(dead_code)]

mod day6;
use day6::{read_input, part_one};

fn main() {
    let (p, m) = read_input("src/day6/input.txt");
    let steps = part_one(p, m);
    println!("Steps: {steps}");
}
