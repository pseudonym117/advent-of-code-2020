use regex::Regex;
use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    let input_file = &fs::read("input.txt")?;
    let input_as_str = String::from_utf8_lossy(&input_file);
    part1(&input_as_str)?;
    part2(&input_as_str)
}

fn part1(input_as_str: &std::borrow::Cow<str>) -> Result<(), Box<dyn Error>> {
    let re = Regex::new(r"(\d+)-(\d+) ([a-z]): ([a-z]+)")?;

    let mut counter = 0;

    for captures in re.captures_iter(input_as_str) {
        let min: usize = captures[1].parse()?;
        let max: usize = captures[2].parse()?;
        let letter = &captures[3];
        let password = &captures[4];

        let count = password
            .chars()
            .filter(|c| c == &letter.chars().nth(0).unwrap())
            .count();

        if min <= count && count <= max {
            counter += 1;
        }
    }

    println!("Part1: {}", counter);

    Ok(())
}

fn part2(input_as_str: &std::borrow::Cow<str>) -> Result<(), Box<dyn Error>> {
    let re = Regex::new(r"(\d+)-(\d+) ([a-z]): ([a-z]+)")?;

    let mut counter = 0;

    for captures in re.captures_iter(input_as_str) {
        let first: usize = captures[1].parse()?;
        let second: usize = captures[2].parse()?;
        let letter = &captures[3].chars().nth(0).unwrap();
        let password = &captures[4];

        let first_is_set = &password.chars().nth(first - 1).unwrap() == letter;
        let second_is_set = &password.chars().nth(second - 1).unwrap() == letter;

        if first_is_set ^ second_is_set {
            counter += 1;
        }
    }

    println!("Part2: {}", counter);

    Ok(())
}
