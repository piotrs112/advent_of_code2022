use regex::Regex;
use std::fmt::Debug;
use std::fs;

#[derive(Debug)]
struct RearrangeProc {
    how_many: u32,
    from: usize,
    to: usize,
}

fn main() {
    let file = fs::read_to_string("/home/piotr/Documents/advent2022/dec5/src/input.txt").unwrap();
    println!("First star: {}", first_star(&file));
    println!("Second star: {}", second_star(&file));
}

fn parse_input(file: &String) -> (Vec<Vec<char>>, Vec<RearrangeProc>) {
    let two_parts: Vec<&str> = file.split("\n\n").collect();
    let mut start_stacks: Vec<&str> = two_parts[0].lines().collect();
    let n_stacks: usize = start_stacks
        .pop()
        .expect("No last line!")
        .chars()
        .max()
        .unwrap()
        .to_digit(10)
        .unwrap() as usize;

    let mut stacks: Vec<Vec<char>> = vec![Vec::new(); n_stacks];

    start_stacks.reverse();
    for line in start_stacks {
        let positions = (1..n_stacks * 4).step_by(4);
        for (i, pos) in positions.enumerate() {
            let container = line.chars().nth(pos).unwrap();
            if container.is_alphabetic() {
                stacks[i].push(container);
            }
            // println!("{:?}", stacks[i]);
        }
        // println!();
    }

    let moves: Vec<&str> = two_parts[1].lines().collect();
    let pattern = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
    let procedure: Vec<RearrangeProc> = moves
        .into_iter()
        .map(|m| {
            let cap = pattern.captures(m).unwrap();
            RearrangeProc {
                how_many: cap.get(1).unwrap().as_str().parse::<u32>().unwrap(),
                from: cap.get(2).unwrap().as_str().parse::<usize>().unwrap(),
                to: cap.get(3).unwrap().as_str().parse::<usize>().unwrap(),
            }
        })
        .collect();

    (stacks, procedure)
}

fn stacks_to_output_string(stacks: &Vec<Vec<char>>) -> String {
    stacks
        .into_iter()
        .map(|s| *s.last().unwrap())
        .collect::<String>()
}

fn first_star(file: &String) -> String {
    let (mut stacks, procedures) = parse_input(&file);
    for p in procedures {
        for _ in 0..p.how_many {
            if let Some(moved_crate) = stacks[p.from - 1].pop() {
                stacks[p.to - 1].push(moved_crate);
            }
        }
    }

    stacks_to_output_string(&stacks)
}

fn second_star(file: &String) -> String {
    let (mut stacks, procedures) = parse_input(&file);
    for p in procedures {
        let mut moved_stack: Vec<char> = vec![];
        for _ in 0..p.how_many {
            if let Some(moved_crate) = stacks[p.from - 1].pop() {
                moved_stack.push(moved_crate);
            }
        }
        moved_stack.reverse();
        for c in moved_stack {
            stacks[p.to - 1].push(c);
        }
    }

    stacks_to_output_string(&stacks)
}

#[cfg(test)]
mod tests {
    use std::fs;

    use crate::first_star;
    use crate::second_star;

    #[test]
    fn test_first_star() {
        let file =
            fs::read_to_string("/home/piotr/Documents/advent2022/dec5/src/test.txt").unwrap();
        assert_eq!(first_star(&file), "CMZ");
    }
    #[test]
    fn test_second_star() {
        let file =
            fs::read_to_string("/home/piotr/Documents/advent2022/dec5/src/test.txt").unwrap();
        assert_eq!(second_star(&file), "MCD");
    }
}
