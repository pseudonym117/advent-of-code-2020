use regex::Regex;
use std::collections::HashMap;
use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    let input_file = &fs::read("input.txt")?;
    let input_as_str = String::from_utf8_lossy(&input_file);
    part1(&input_as_str)?;
    part2(&input_as_str)
}

fn valid_passport(passport: &HashMap<String, String>) -> bool {
    let required = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];

    for field in required.iter() {
        if !passport.contains_key(&field.to_string()) {
            return false;
        }
    }

    return true;
}

fn part1(input_as_str: &std::borrow::Cow<str>) -> Result<(), Box<dyn Error>> {
    let input_lines = input_as_str.lines();

    let prop_re = Regex::new(r"([a-zA-Z]{3}):([a-zA-Z0-9#]+)")?;

    let mut valid_count = 0;
    let mut fields: HashMap<String, String> = HashMap::new();

    for line in input_lines {
        if line.trim().is_empty() {
            if valid_passport(&fields) {
                valid_count += 1;
            }

            fields.clear();
            continue;
        }

        for capture in prop_re.captures_iter(line) {
            let prop = &capture[1];
            let val = &capture[2];
            fields.insert(prop.to_string(), val.to_string());
        }
    }

    if valid_passport(&fields) {
        valid_count += 1;
    }

    println!("Part1: valid: {}", valid_count);

    Ok(())
}

fn valid_year(year: &String, min: i32, max: i32) -> bool {
    let year_val: i32 = year.parse().unwrap();
    min <= year_val && year_val <= max
}

fn valid_height(height: &String) -> bool {
    let re = Regex::new(r"([0-9]+)(cm|in)").unwrap();
    let captures = re.captures(&height);
    match captures {
        Some(cap) => {
            let measure: i32 = cap[1].parse().unwrap();
            let unit = &cap[2];

            match unit {
                "cm" => 150 <= measure && measure <= 193,
                "in" => 59 <= measure && measure <= 76,
                _ => false,
            }
        }
        None => false,
    }
}

fn valid_hair(hair: &String) -> bool {
    let re = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
    re.is_match(&hair)
}

fn valid_eye(eye: &String) -> bool {
    let valid = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
    valid.contains(&eye.as_str())
}

fn valid_passport_id(passport_id: &String) -> bool {
    let re = Regex::new(r"^[0-9]{9}$").unwrap();
    re.is_match(passport_id)
}

fn strictly_valid_passport(passport: &HashMap<String, String>) -> bool {
    if !valid_passport(passport) {
        return false;
    }

    let birth_year = passport.get("byr").unwrap();
    let issue_year = passport.get("iyr").unwrap();
    let expiration_year = passport.get("eyr").unwrap();
    let height = passport.get("hgt").unwrap();
    let hair_color = passport.get("hcl").unwrap();
    let eye_color = passport.get("ecl").unwrap();
    let passport_id = passport.get("pid").unwrap();

    valid_year(&birth_year, 1920, 2002)
        && valid_year(&issue_year, 2010, 2020)
        && valid_year(&expiration_year, 2020, 2030)
        && valid_height(&height)
        && valid_hair(&hair_color)
        && valid_eye(&eye_color)
        && valid_passport_id(&passport_id)
}

fn part2(input_as_str: &std::borrow::Cow<str>) -> Result<(), Box<dyn Error>> {
    let input_lines = input_as_str.lines();

    let prop_re = Regex::new(r"([a-zA-Z]{3}):([a-zA-Z0-9#]+)")?;

    let mut valid_count = 0;
    let mut fields: HashMap<String, String> = HashMap::new();

    for line in input_lines {
        if line.trim().is_empty() {
            if strictly_valid_passport(&fields) {
                valid_count += 1;
            }

            fields.clear();
            continue;
        }

        for capture in prop_re.captures_iter(line) {
            let prop = &capture[1];
            let val = &capture[2];
            fields.insert(prop.to_string(), val.to_string());
        }
    }

    if strictly_valid_passport(&fields) {
        valid_count += 1;
    }

    println!("Part2: strictly valid: {}", valid_count);

    Ok(())
}
