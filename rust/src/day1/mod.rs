#![allow(dead_code)]

use std::collections::HashMap;
use std::fs;
use std::str;

pub fn read_input(path: &str) -> (Vec<i32>, Vec<i32>) {
    let contents = fs::read_to_string(path).unwrap();
    contents
        .split("\n")
        .map(|line| {
            let pair = line.split("   ")
                .flat_map(str::parse::<i32>)
                .collect::<Vec<i32>>();

            (pair[0], pair[1])
        }).unzip()
}

pub fn part_one(data: (Vec<i32>, Vec<i32>)) -> i32 {
    let (mut first, mut second) = data;
    first.sort();
    second.sort();

    first.iter().zip(second.iter()).map(|(f, s)| {
        (f - s).abs()
    }).sum()
}

pub fn part_two(data: (Vec<i32>, Vec<i32>)) -> i32 {
    let counts = data.1.iter().fold(HashMap::new(), |mut acc, &item| {
        *acc.entry(item).or_insert(0) += 1;
        acc
    });

    data.0.iter().map(|id| {
        match counts.get(id) {
            Some(&count) => count * id,
            None => 0
        }
    }).sum()
}