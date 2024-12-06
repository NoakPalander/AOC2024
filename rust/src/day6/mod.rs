use std::fs;

enum Direction {
    Up,
    Down,
    Left,
    Right
}

pub struct Position {
    row: i32,
    col: i32,
    direction: Direction
}

impl Position {
    fn rotate(self: &mut Position) {
        self.direction = match self.direction {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up
        };
    }

    fn next(self: &Position) -> (i32, i32) {
        let row = self.row;
        let col = self.col;

        match self.direction {
            Direction::Up => (row - 1, col),
            Direction::Down => (row + 1, col),
            Direction::Left => (row, col - 1),
            Direction::Right => (row, col + 1),
        }
    }
}

pub type Matrix = Vec<Vec<char>>;

pub fn read_input(path: &str) -> (Position, Matrix) {
    let mut position = Position{row: 0, col: 0, direction: Direction::Up};
    let matrix = fs::read_to_string(path)
        .unwrap()
        .strip_suffix("\n")
        .unwrap()
        .split("\n")
        .enumerate()
        .map(|(row, line)| {
            line.chars().enumerate().map(|(col, key)| {
                if key == '^' {
                    position.row = row as i32;
                    position.col = col as i32;
                }

                key
            }).collect()
        }).collect();

    (position, matrix)
}

pub fn part_one(mut pos: Position, mut matrix: Matrix) -> i32 {
    let rows = matrix.len() as i32;
    let cols = matrix[0].len() as i32;
    let mut steps = 0;

    loop {
        let next = pos.next();

        // Escaped, done
        if (next.0 < 0 || next.0 >= rows) || (next.1 < 0 || next.1 >= cols) {
            steps += 1;
            break;
        }
        // Blocked, rotate
        else if matrix[next.0 as usize][next.1 as usize] == '#' {
            pos.rotate();
        }
        // Move forward
        else {
            let current = &mut matrix[pos.row as usize][pos.col as usize];
            if *current != 'X' {
                *current = 'X';
                steps += 1;
            }

            pos.row = next.0;
            pos.col = next.1;
        }
    }

    steps
}
