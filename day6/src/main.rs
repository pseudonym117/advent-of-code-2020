use std::borrow::Cow;
use std::collections::HashMap;
use std::collections::HashSet;
use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    let input_file = &fs::read("input.txt")?;
    let input_as_str = String::from_utf8_lossy(&input_file);
    part1(&input_as_str);
    part2(&input_as_str);
    Ok(())
}

fn part1(input_as_str: &Cow<str>) {
    let mut counter = 0;

    let mut set = HashSet::new();

    for line in input_as_str.lines() {
        if line.trim().is_empty() {
            counter += set.len();
            set.clear();
        } else {
            for c in line.trim().chars() {
                set.insert(c);
            }
        }
    }

    counter += set.len();

    println!("Part1: count: {}", counter);
}

fn all_yes(people: usize, map: &HashMap<char, usize>) -> usize {
    let mut counter = 0;
    for (_, val) in map {
        if val == &people {
            counter += 1;
        }
    }

    counter
}

fn part2(input_as_str: &Cow<str>) {
    let mut counter = 0;
    let mut map = HashMap::new();
    let mut people_count = 0;
    for line in input_as_str.lines() {
        if line.trim().is_empty() {
            counter += all_yes(people_count, &map);
            map.clear();
            people_count = 0;
        } else {
            for c in line.trim().chars() {
                if let Some(count) = map.get_mut(&c) {
                    *count = *count + 1;
                } else {
                    map.insert(c, 1);
                }
            }

            people_count += 1;
        }
    }
    counter += all_yes(people_count, &map);
    println!("Part2: count: {}", counter);
}
