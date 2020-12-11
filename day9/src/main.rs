use std::borrow::Cow;
use std::collections::HashSet;
use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    let input_file = &fs::read("input.txt")?;
    let input_as_str = String::from_utf8_lossy(&input_file);
    let first_to_break = part1(&input_as_str, 25).unwrap();
    println!(
        "Part1: first number to not meet condition: {}",
        first_to_break
    );
    part2(&input_as_str, first_to_break);
    Ok(())
}

fn part1(input_as_str: &Cow<str>, preamble: usize) -> Option<i64> {
    let mut rolling_buffer = vec![];
    let mut current_numbers = HashSet::new();

    for line in input_as_str.lines() {
        if let Ok(as_int) = line.parse::<i64>() {
            if rolling_buffer.len() >= preamble {
                let mut found = false;
                for prev_val in &rolling_buffer {
                    let target_val = as_int - prev_val;
                    if current_numbers.contains(&target_val) {
                        found = true;
                        break;
                    }
                }

                if !found {
                    return Some(as_int);
                }

                let removed = rolling_buffer.remove(0);
                current_numbers.remove(&removed);
            }

            rolling_buffer.push(as_int);
            current_numbers.insert(as_int);
        } else {
            println!("ERROR reading line: \"{}\"", line);
        }
    }

    None
}

fn part2(input_as_str: &Cow<str>, target_value: i64) {
    let as_ints: Vec<i64> = input_as_str
        .lines()
        .map(|line| line.parse().unwrap())
        .filter(|val| val < &target_value)
        .collect();
    for low in 0..(as_ints.len() - 1) {
        for high in low..(as_ints.len() - 1) {
            let sum: i64 = as_ints[low..high].iter().sum();
            if sum == target_value {
                let low_val = as_ints[low..high].iter().min().unwrap();
                let high_val = as_ints[low..high].iter().max().unwrap();

                let sum = low_val + high_val;

                println!("Part2: found range. Sum of min and max vals: {}", sum);
                return;
            }
        }
    }
}
