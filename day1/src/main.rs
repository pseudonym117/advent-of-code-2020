use std::collections::HashSet;
use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    let input_file = &fs::read("input.txt")?;
    let input_as_str = String::from_utf8_lossy(&input_file);
    part1(&input_as_str)?;
    part2(&input_as_str)
}

fn part1(input_as_str: &std::borrow::Cow<str>) -> Result<(), Box<dyn Error>> {
    let input_lines = input_as_str.lines();

    const TARGET_YEAR: i32 = 2020;

    let mut previous_values = HashSet::new();
    for i in input_lines {
        let line_val: i32 = i.parse()?;
        let target_val = TARGET_YEAR - line_val;

        if previous_values.contains(&target_val) {
            let prod = line_val * target_val;
            println!("Part 1: values ({}, {}): {}", line_val, target_val, prod);
            break;
        }

        previous_values.insert(line_val);
    }

    Ok(())
}

fn part2(input_as_str: &std::borrow::Cow<str>) -> Result<(), Box<dyn Error>> {
    let input_lines = input_as_str.lines();

    const TARGET_YEAR: i32 = 2020;

    let value_set: HashSet<i32> = input_lines.map(|i| i.parse::<i32>().unwrap()).collect();
    for i in &value_set {
        let new_target_year = TARGET_YEAR - i;

        for j in &value_set {
            let target_val = new_target_year - j;
            if value_set.contains(&target_val) {
                let prod = i * j * target_val;
                println!("Part2: values ({}, {}, {}): {}", i, j, target_val, prod);
                return Ok(());
            }
        }
    }

    Ok(())
}
