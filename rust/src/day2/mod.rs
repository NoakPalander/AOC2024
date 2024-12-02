use std::fs;
use std::str;

pub fn read_input(path: &str) -> Vec<Vec<i32>> {
    let contents = fs::read_to_string(path).unwrap();

    contents.split("\n").map(|line| {
        line.split(" ")
            .flat_map(str::parse)
            .collect::<Vec<i32>>()
    }).collect::<Vec<Vec<i32>>>()
}

pub fn part_one(report: Vec<Vec<i32>>) -> i32 {
    report.iter().map(|levels| {
        let checks = levels.windows(2).fold(Vec::new(), |mut acc, pair| {
            let difference = pair[1] - pair[0];
            let abs = difference.abs();
            let positive = difference > 0;

            let safe = match acc.last() {
                Some((p, a)) => *p == positive && abs > 0 && abs <= 3,
                None => abs > 0 && abs <= 3
            };

            acc.push((positive, safe));
            acc
        });

        let safe = checks.iter().all(|(positive, safe)| *positive == checks[0].0 && *safe);
        safe as i32
    }).sum()
}

fn check_groups(levels: &Vec<i32>) -> i32 {
    let mut positive: Option<bool> = None;

    for i in 0..levels.len() - 1 {
        let current = levels[i];
        let next = levels[i + 1];

        let difference = next - current;
        let abs = difference.abs();

        let same_direction = match positive {
            Some(dir) => dir == (difference > 0),
            None => {
                positive = Some(difference > 0);
                true
            }
        };

        let safe_distance = abs > 0 && abs <= 3;
        if !(safe_distance && same_direction) {
            return i as i32;
        }
    }

    -1
}

fn check_levels(levels: &Vec<i32>) -> bool {
    let mut safe = false;

    if check_groups(levels) != -1 {
        for i in 0..levels.len() {
            let mut new = levels.clone();
            new.remove(i);

            if check_groups(&new) == -1 {
                safe = true;
                break
            }
        }
    }
    else {
        safe = true
    }

    safe
}

pub fn part_two(report: Vec<Vec<i32>>) -> i32 {
    report.iter().map(|levels| {
        check_levels(&levels) as i32
    }).sum()
}