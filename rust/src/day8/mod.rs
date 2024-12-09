use std::collections::{BTreeSet, HashMap, HashSet, VecDeque};
use std::fs;
use itertools::Itertools;

pub fn read_input(path: &str) -> Vec<Vec<char>> {
    fs::read_to_string(path)
        .unwrap()
        .split("\n")
        .map(|line| {
            line.chars().collect()
        }).collect()
}

type Position = (i32, i32);

#[derive(Debug)]
struct Task {
    current: Option<Position>,
    ignored: Vec<Position>
}

impl Task {
    fn new() -> Task {
        Task { current: None, ignored: Vec::new() }
    }
}

fn solve(matrix: &mut Vec<Vec<char>>, p1: Position, p2: Position, positions: &mut HashSet<Position>) {
    let rows = matrix.len() as i32;
    let cols = matrix[0].len() as i32;

    let dy = p1.0 - p2.0;
    let dx = p1.1 - p2.1;

    let new_y = p1.0 + dy;
    let new_x = p1.1 + dx;
    if new_y >= 0 && new_y < rows && new_x >= 0 && new_x < cols {
        let cell = &mut matrix[new_y as usize][new_x as usize];
        if *cell == '.' || *cell == '#' {
            *cell = '#';
        }

        positions.insert((new_y, new_x));
    }
}

pub fn part_one(mut data: Vec<Vec<char>>) -> i32 {
    let rows = data.len();
    let cols = data[0].len();

    let mut map: HashMap<char, Task> = HashMap::new();
    let mut positions: HashSet<(i32, i32)> = HashSet::new();

    data.clone()
        .iter()
        .flatten()
        .filter(|&key| key.is_alphanumeric())
        .unique()
        .for_each(|freq| {
            loop {
                for row in 0..rows {
                    for col in 0..cols {
                        let cell = data[row][col];
                        let position = (row as i32, col as i32);

                        if cell == *freq {
                            let task = map.entry(cell).or_insert(Task::new());
                            if task.current.is_none() && !task.ignored.contains(&position) {
                                task.current = Some(position);
                                continue;
                            }

                            if let Some(current) = task.current {
                                solve(&mut data, current, position , &mut positions);
                                solve(&mut data, position, current, &mut positions);
                            }
                        }
                    }
                }

                let task = map.get_mut(&freq).unwrap();
                match task.current {
                    None => break,
                    Some(current) => {
                        task.current = None;
                        task.ignored.push(current);

                    }
                }
            }
        });

    positions.len() as i32
}

pub fn part_two(data: &Vec<Vec<char>>) -> i32 {
    0
}
