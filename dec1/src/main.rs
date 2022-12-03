use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    // Read file into lines
    let file = File::open("/home/piotr/Documents/advent2022/dec1/src/input.txt")
        .expect("Cannot read file!");
    let lines = BufReader::new(file).lines();

    let mut calories: Vec<i32> = Vec::new();
    let mut calories_sum = 0;

    for line in lines {
        if let Ok(lp) = line {
            if lp == "" {
                calories.push(calories_sum);
                calories_sum = 0;
            } else {
                calories_sum += lp.parse::<i32>().unwrap();
            }
        }
    }

    calories.sort();
    calories.reverse();
    println!("First answer: {}", calories.first().unwrap());
    calories.truncate(3);
    println!("Second answer: {}", calories.iter().sum::<i32>());
}
