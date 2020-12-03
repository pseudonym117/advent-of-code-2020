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

fn trees_in_slope(input_as_str: &Cow<str>, right: usize, down: usize) -> usize {
    let rows = input_as_str.lines();
    let mut trees = 0;

    let mut x_coord = 0;
    let mut y_coord = 0;

    for row in rows.skip(1) {
        y_coord += 1;
        if y_coord % down == 0 {
            x_coord = (x_coord + right) % row.len();
            if row.chars().nth(x_coord).unwrap() == '#' {
                trees += 1;
            }
        }
    }

    trees
}

fn part1(input_as_str: &Cow<str>) {
    let trees = trees_in_slope(input_as_str, 3, 1);

    println!("Part1: trees: {}", trees);
}

fn part2(input_as_str: &Cow<str>) {
    let slopes = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];

    let mut tree_mult = 1;

    for slope in &slopes {
        let trees = trees_in_slope(input_as_str, slope.0, slope.1);
        tree_mult *= trees;
    }

    println!("Part2: trees: {}", tree_mult);
}
