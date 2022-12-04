use std::collections::HashSet;
use std::fs::{self};
// use std::io::{BufRead, BufReader, Lines};
fn main() {
    // // Read file into lines
    // let file = File::open("/home/piotr/Documents/advent2022/dec3/src/input.txt")
    //     .expect("Cannot read file!");
    // let lines = BufReader::new(file).lines();
    // let mut input = vec![lines.into_iter()];
    let file = fs::read_to_string("/home/piotr/Documents/advent2022/dec3/src/input.txt").unwrap();
    let input: Vec<&str> = file.split("\n").collect();

    println!(
        "First star: {}",
        duplicate_compartments_priority(input.to_vec())
    );
    // println!("Second star: {:?}", group_rucksacks(lines2));
    let badges: Vec<char> = input
        .chunks(3)
        .map(|group| get_badge(group))
        .map(|b| *b.iter().last().unwrap_or(&' '))
        .collect();
    println!(
        "{:?}",
        badges
            .into_iter()
            .map(|b| get_item_priority(&b))
            .sum::<u32>()
    );
}

fn get_item_priority(item: &char) -> u32 {
    let item_ascii = *item as u32;
    if item.is_lowercase() {
        return item_ascii - 96;
    } else if item.is_uppercase() {
        return item_ascii - 38;
    } else {
        return 0;
    }
}

fn get_duplicate_chars(first: &str, second: &str) -> Vec<char> {
    let mut duplicated: Vec<char> = vec![];

    for c1 in first.chars() {
        for c2 in second.chars() {
            if c1 == c2 {
                if !duplicated.contains(&c1) {
                    duplicated.push(c1);
                }
            }
        }
    }

    duplicated
}

fn get_badge(rucksacks: &[&str]) -> HashSet<char> {
    rucksacks
        .into_iter()
        .map(|r| HashSet::<char>::from_iter(r.chars()))
        .reduce(|accum, r| {
            let common = accum.intersection(&r);
            HashSet::<char>::from_iter(common.into_iter().copied())
        })
        .unwrap()
}

fn duplicate_compartments_priority(lines: Vec<&str>) -> u32 {
    let mut duplicate_items_priority_total: u32 = 0;
    for line in lines {
        let rucksack_capacity = line.chars().count();
        let first_compartment: &str = &line[0..rucksack_capacity / 2];
        let second_compartment: &str = &line[(rucksack_capacity / 2)..rucksack_capacity];

        let duplicate_chars = get_duplicate_chars(first_compartment, second_compartment);
        for c in duplicate_chars {
            duplicate_items_priority_total += get_item_priority(&c);
        }
    }

    duplicate_items_priority_total
}
