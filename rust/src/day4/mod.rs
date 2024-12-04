use std::fs;
use std::str;
use diagonal::{diagonal_pos_neg, diagonal_pos_pos};

pub fn read_input(path: &str) -> Vec<String> {
    fs::read_to_string(path).unwrap().split("\n").map(str::to_string).collect()
}

fn transpose(v: &Vec<String>) -> Vec<String> {
    (0..v[0].len())
        .map(|i| {
            v.iter().map(|s| s.chars().nth(i).unwrap()).collect()
        }).collect()
}

pub fn part_one(data: Vec<String>) -> usize {
    let rows = || {
        data.iter().fold(0, |acc, row| {
            acc + row.matches("XMAS").count() + row.matches("SAMX").count()
        })
    };

    let cols = || {
        transpose(&data).iter().fold(0, |acc, col| {
            acc + col.matches("XMAS").count() + col.matches("SAMX").count()
        })
    };

    let diagonals = || {
        let matrix: Vec<Vec<char>> = data.iter().map(|s| s.chars().collect()).collect();
        let pos_diags = diagonal_pos_pos(&matrix).iter().map(|v| v.iter().copied().collect())
            .fold(0, |acc, diag: String| {
                acc + diag.matches("XMAS").count() + diag.matches("SAMX").count()
        });

        let neg_diags = diagonal_pos_neg(&matrix).iter().map(|v| v.iter().copied().collect())
            .fold(0, |acc, diag: String| {
                acc + diag.matches("XMAS").count() + diag.matches("SAMX").count()
            });

        pos_diags + neg_diags
    };

    rows() + cols() + diagonals()
}

pub fn part_two(data: Vec<String>) -> usize {
    let cols = data.len();
    let rows = data[0].len();

    (0..rows - 2).fold(0, |acc, row| {
        acc + (0..cols - 2).filter(|col| {
            let x = (row..row + 3).map(|r| {
                (data[r][*col..col + 3]).chars().collect::<Vec<char>>()
            }).collect::<Vec<_>>();

            let diag1 = format!("{}{}{}", x[0][0], x[1][1], x[2][2]);
            let diag2 = format!("{}{}{}", x[0][2], x[1][1], x[2][0]);

            (diag1 == "MAS" || diag1 == "SAM") && (diag2 == "MAS" || diag2 == "SAM")
        }).count()
    })
}