use itertools::sorted;
use std::borrow::Cow;
use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    let input_file = &fs::read("input.txt")?;
    let input_as_str = String::from_utf8_lossy(&input_file);
    part1(&input_as_str);
    part2(&input_as_str);
    Ok(())
}

fn parse_coord_id(input: &str) -> i16 {
    let mut id: i16 = 0;
    let mut mask: i16 = 0b01000000000;

    for i in input.chars().take(10) {
        if i == 'B' || i == 'R' {
            id |= mask;
        }
        mask >>= 1;
    }

    id
}

fn part1(input_as_str: &Cow<str>) {
    let largest_id = input_as_str.lines().map(parse_coord_id).max().unwrap();
    println!("Part1: largest id: {}", largest_id);
}

fn part2(input_as_str: &Cow<str>) {
    let taken_seats = sorted(input_as_str.lines().map(parse_coord_id));

    let mut prev: i16 = 0;

    for seat in taken_seats {
        if prev != 0 && prev != seat - 1 {
            break;
        }

        prev = seat;
    }

    prev += 1;

    println!("Part2: my seat: {}", prev);
}
