use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    // Read file into lines
    let file = File::open("/home/piotr/Documents/advent2022/dec3/src/input.txt")
        .expect("Cannot read file!");
    let lines = BufReader::new(file).lines();
    let mut duplicate_items_priority_total: u32 = 0;

    for line in lines {
        if let Ok(lp) = line {
            let rucksack_capacity = lp.chars().count();
            let first_compartment: &str = &lp[0..rucksack_capacity / 2];
            let second_compartment: &str = &lp[(rucksack_capacity / 2)..rucksack_capacity];

            let duplicate_chars = get_duplicate_chars(first_compartment, second_compartment);
            for c in duplicate_chars {
                duplicate_items_priority_total += get_item_priority(&c);
            }
        }
    }
    println!("First star: {}", duplicate_items_priority_total);
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
