use std::borrow::Cow;
use std::convert::TryInto;
use std::error::Error;
use std::fs;

#[derive(PartialEq)]
enum SeatState {
    Floor,
    Empty,
    Filled,
}

impl Copy for SeatState {}

impl Clone for SeatState {
    fn clone(&self) -> Self {
        *self
    }
}

fn char_to_seat(c: char) -> Result<SeatState, ()> {
    match c {
        '.' => Ok(SeatState::Floor),
        'L' => Ok(SeatState::Empty),
        '#' => Ok(SeatState::Filled),
        _ => Err(()),
    }
}

#[warn(dead_code)]
fn seat_to_char(seat: &SeatState) -> char {
    match seat {
        SeatState::Floor => '.',
        SeatState::Empty => 'L',
        SeatState::Filled => '#',
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let input_file = &fs::read("input.txt")?;
    let input_as_str = String::from_utf8_lossy(&input_file);
    let board = to_board(&input_as_str).unwrap();
    part1(board.clone());
    part2(board);
    Ok(())
}

fn to_board(input_as_str: &Cow<str>) -> Result<Vec<Vec<SeatState>>, ()> {
    Ok(input_as_str
        .lines()
        .map(|line| {
            line.chars()
                .map(char_to_seat)
                .map(|state_opt| state_opt.unwrap())
                .collect()
        })
        .collect())
}

fn adjacent_filled(state: &Vec<Vec<SeatState>>, row: usize, col: usize) -> usize {
    let mut sum = 0;

    let max_row = state.len() - 1;
    let row_start = match row {
        0 => 0,
        _ => row - 1,
    };
    let row_end = if row >= max_row { max_row } else { row + 1 };
    for row_pos in row_start..=row_end {
        let row_vals = &state[row_pos];

        let max_col = row_vals.len() - 1;
        let col_start = match col {
            0 => 0,
            _ => col - 1,
        };
        let col_end = if col >= max_col { max_col } else { col + 1 };
        for col_pos in col_start..=col_end {
            if row_pos == row && col_pos == col {
                continue;
            }

            match row_vals[col_pos] {
                SeatState::Filled => sum += 1,
                _ => {}
            }
        }
    }

    sum
}

fn can_see(state: &Vec<Vec<SeatState>>, row: usize, col: usize) -> usize {
    let row_int: i32 = row.try_into().unwrap();
    let col_int: i32 = col.try_into().unwrap();

    let directions = vec![
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    let mut visible = 0;

    for (row_diff, col_diff) in directions {
        let mut cur_row = row_int + row_diff;
        let mut cur_col = col_int + col_diff;

        while cur_row >= 0
            && (cur_row as usize) < state.len()
            && cur_col >= 0
            && (cur_col as usize) < state[cur_row as usize].len()
        {
            match state[cur_row as usize][cur_col as usize] {
                SeatState::Filled => {
                    visible += 1;
                    break;
                }
                SeatState::Empty => break,
                SeatState::Floor => {}
            }

            cur_row += row_diff;
            cur_col += col_diff;
        }
    }

    visible
}

fn next_frame(
    current: &Vec<Vec<SeatState>>,
    neighbor_calculator: fn(&Vec<Vec<SeatState>>, usize, usize) -> usize,
    will_vacate: fn(usize) -> bool,
) -> Option<Vec<Vec<SeatState>>> {
    let mut next = current.to_vec();
    let mut changed = false;

    for (row_ind, row) in current.iter().enumerate() {
        for (col_ind, col) in row.iter().enumerate() {
            let new = match col {
                SeatState::Empty => match neighbor_calculator(current, row_ind, col_ind) {
                    0 => SeatState::Filled,
                    _ => *col,
                },
                SeatState::Filled => {
                    if will_vacate(neighbor_calculator(current, row_ind, col_ind)) {
                        SeatState::Empty
                    } else {
                        *col
                    }
                }
                _ => *col,
            };

            if new != *col {
                changed = true;
            }

            next[row_ind][col_ind] = new;
        }
    }
    match changed {
        true => Some(next),
        false => None,
    }
}

#[warn(dead_code)]
fn print_board(board: &Vec<Vec<SeatState>>) {
    for row in board {
        let line_str: String = row.iter().map(seat_to_char).collect();
        println!("{}", line_str);
    }
}

fn part1(mut board: Vec<Vec<SeatState>>) {
    let mut iteration = 0;
    loop {
        // println!("\niteration {}\n", iteration);
        // print_board(&board);
        if let Some(next) = next_frame(&board, adjacent_filled, |adjacent| adjacent >= 4) {
            board = next;
        } else {
            break;
        }
        iteration += 1;
    }

    let total_filled: usize = board
        .iter()
        .map(|row| {
            row.iter()
                .filter(|state| match state {
                    SeatState::Filled => true,
                    _ => false,
                })
                .count()
        })
        .sum();
    println!(
        "Part1: seats filled after {} iterations: {}",
        iteration, total_filled
    );
}

fn part2(mut board: Vec<Vec<SeatState>>) {
    let mut iteration = 0;

    loop {
        // println!("\niteration {}\n", iteration);
        // print_board(&board);
        if let Some(next) = next_frame(&board, can_see, |visible| visible >= 5) {
            board = next;
        } else {
            break;
        }
        iteration += 1;
    }

    let total_filled: usize = board
        .iter()
        .map(|row| {
            row.iter()
                .filter(|state| match state {
                    SeatState::Filled => true,
                    _ => false,
                })
                .count()
        })
        .sum();
    println!(
        "Part2: seats filled after {} iterations: {}",
        iteration, total_filled
    );
}
