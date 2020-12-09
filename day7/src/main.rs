use std::borrow::Cow;
use std::collections::HashMap;
use std::collections::HashSet;
use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    let input_file = &fs::read("input.txt")?;
    let input_as_str = String::from_utf8_lossy(&input_file);
    part1(&input_as_str);
    Ok(())
}

struct Bag {
    name: String,
    contains: HashMap<String, usize>,
}

fn read_bag(input: &str) -> Result<Bag, &'static str> {
    let mut contains = HashMap::new();

    let without_bags = input.replace("bags", "").replace("bag", "");

    let split: Vec<&str> = without_bags.splitn(2, "contain").collect();
    if split.len() != 2 {
        return Err("did not split at \"contain\" correctly");
    }

    let name = split
        .get(0)
        .unwrap()
        .trim()
        .split(" ")
        .collect::<Vec<&str>>()
        .join(" ");
    let rules: Vec<&str> = split.get(1).unwrap().trim().split(",").collect();

    for rule in rules {
        if rule.trim().starts_with("no") {
            break;
        }

        let split_rule: Vec<&str> = rule.trim().splitn(2, " ").collect();
        if split_rule.len() != 2 {
            return Err("did not split at \" \" correctly");
        }

        let count: usize = split_rule.get(0).unwrap().parse().unwrap();
        let inner_name = split_rule
            .get(1)
            .unwrap()
            .trim_matches(|c| !char::is_alphabetic(c));

        contains.insert(inner_name.to_string(), count);
    }

    Ok(Bag { contains, name })
}

fn all_possible_containing_bags(
    initial_bag: &str,
    bags_to_can_be_contained_by: &HashMap<&String, HashSet<String>>,
) -> HashSet<String> {
    let mut complete_set = HashSet::new();

    fn all_possible_containing_bags_rec(
        current_bag: &str,
        bags_to_can_be_contained_by: &HashMap<&String, HashSet<String>>,
        mut complete_set: &mut std::collections::HashSet<std::string::String>,
    ) {
        if complete_set.contains(current_bag) {
            return;
        }

        complete_set.insert(current_bag.to_string());

        if let Some(possible_containers) = bags_to_can_be_contained_by.get(&current_bag.to_string())
        {
            for bag in possible_containers.iter() {
                all_possible_containing_bags_rec(
                    bag,
                    &bags_to_can_be_contained_by,
                    &mut complete_set,
                )
            }
        }
    }

    all_possible_containing_bags_rec(
        &initial_bag,
        &bags_to_can_be_contained_by,
        &mut complete_set,
    );

    complete_set.remove(initial_bag);
    complete_set
}

fn bags_within_bag(
    target_bag: &str,
    bag_rules: &HashMap<&String, &HashMap<String, usize>>,
) -> usize {
    if let Some(target_rule) = bag_rules.get(&target_bag.to_string()) {
        target_rule
            .iter()
            .map(|kv| kv.1 * (1 + bags_within_bag(kv.0, &bag_rules)))
            .sum()
    } else {
        0
    }
}

fn part1(input_as_str: &Cow<str>) {
    let bag_rules_list: Vec<Result<Bag, &'static str>> =
        input_as_str.lines().map(read_bag).collect();

    let bag_rules: HashMap<&String, &HashMap<String, usize>> = bag_rules_list
        .iter()
        .filter_map(|bag_op| {
            if let Ok(bag) = bag_op {
                Some((&bag.name, &bag.contains))
            } else {
                None
            }
        })
        .collect();

    let mut bags_to_can_be_contained_by: HashMap<&String, HashSet<String>> =
        bag_rules.iter().map(|kv| (*kv.0, HashSet::new())).collect();

    for (bag, contains) in &bag_rules {
        for (inner_bag, _) in contains.iter() {
            let can_be_contained_by = bags_to_can_be_contained_by.get_mut(&inner_bag).unwrap();
            can_be_contained_by.insert(bag.to_string());
        }
    }

    let bags_that_can_contain_gold =
        all_possible_containing_bags("shiny gold", &bags_to_can_be_contained_by);

    println!(
        "Part1: shiny gold can be contained by {} / {} different bags",
        bags_that_can_contain_gold.len(),
        bags_to_can_be_contained_by.len()
    );

    let bags_within_shiny_gold = bags_within_bag("shiny gold", &bag_rules);

    println!("Part2: shiny gold contains {} bags", bags_within_shiny_gold);
}
