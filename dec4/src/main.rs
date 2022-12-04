use std::fs;
use std::{collections::HashSet, ops::RangeInclusive};

fn main() {
    let file = fs::read_to_string("/home/piotr/Documents/advent2022/dec4/src/input.txt").unwrap();

    let input: Vec<&str> = file.split_terminator("\n").collect();

    let output: usize = input
        .into_iter()
        .map(|l| Vec::from_iter(l.split(",")))
        .map(|pair| {
            let bounds1: Vec<u32> = pair[0]
                .split("-")
                .map(|n| n.parse::<u32>().unwrap())
                .collect();

            let bounds2: Vec<u32> = pair[1]
                .split("-")
                .map(|n| n.parse::<u32>().unwrap())
                .collect();

            ((bounds1[0]..=bounds1[1]), (bounds2[0]..=bounds2[1]))
        })
        // .map(|ranges| does_range_contain_other(ranges.0, ranges.1)) // Star 1
        .map(|ranges| do_ranges_overlap(ranges.0, ranges.1)) // Star 2
        .filter(|x| *x)
        .count();

    println!("{:?}", output);
}

fn does_range_contain_other(r1: RangeInclusive<u32>, r2: RangeInclusive<u32>) -> bool {
    let hs1 = HashSet::<u32>::from_iter(r1);
    let hs2 = HashSet::<u32>::from_iter(r2);

    hs1.is_subset(&hs2) || hs2.is_subset(&hs1)
}

fn do_ranges_overlap(r1: RangeInclusive<u32>, r2: RangeInclusive<u32>) -> bool {
    let hs1 = HashSet::<u32>::from_iter(r1);
    let hs2 = HashSet::<u32>::from_iter(r2);

    let intersection: HashSet<&u32> = hs1.intersection(&hs2).collect();

    !intersection.is_empty()
}
