use std::fs;
use itertools::{Itertools};

pub fn read_input(path: &str) -> Vec<(i64, Vec<i64>)> {
    fs::read_to_string(&path)
        .unwrap()
        .split("\n")
        .map(|line| {
            let pos = line.find(":").unwrap();
            let target = line[..pos].parse::<i64>().unwrap();
            let rest = line[pos + 2..line.len()].split(" ").map(|item| {
                str::parse::<i64>(item).unwrap()
            }).collect();

            (target, rest)
        }).collect()
}

fn eval(op: i32, x: i64, y: i64) -> i64 {
    match op {
        0 => x + y,
        1 => x * y,
        2 => {
            let digits = y.ilog10() + 1;
            x * 10_i64.pow(digits) + y
        }
        _ => panic!("Invalid operation")
    }
}

fn solve(op_count: i32, data: &Vec<(i64, Vec<i64>)>) -> i64 {
    data.iter().fold(0, |sum, &(target, ref operands)| {
        let valid = (0..operands.len() - 1)
            .map(|_| 0..op_count)
            .multi_cartesian_product()
            .any(|combination| {
                target == combination
                    .iter()
                    .zip(operands[1..].iter())
                    .fold(operands[0], |acc, (&op, &num)|{
                        eval(op, acc, num)
                    })
            });


        if valid { sum + target } else { sum }
    })
}

pub fn part_one(data: &Vec<(i64, Vec<i64>)>) -> i64 {
    solve(2, &data)
}

pub fn part_two(data: &Vec<(i64, Vec<i64>)>) -> i64 {
    solve(3, &data)
}