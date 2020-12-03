use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashSet;

#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> HashSet<i32> {
    input
        .lines()
        .map(|l| l.parse().unwrap())
        .collect()
}

#[aoc(day1, part1)]
pub fn solve_part1(input: &HashSet<i32>) -> i32 {
    for x in input {
        let y = 2020 - x;
        if input.contains(&y) {
            return x * y;
        }
    }
    
    0
}

#[aoc(day1, part2)]
pub fn solve_part2(input: &HashSet<i32>) -> i32 {
    for x in input {
        for y in input {
            let z = 2020 - x - y;
            if input.contains(&z) {
                return x * y * z;
            }
        }
    }

    0
}
