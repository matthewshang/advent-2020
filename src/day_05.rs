use aoc_runner_derive::{aoc, aoc_generator};
use std::cmp;
use std::collections::HashSet;

#[aoc_generator(day5)]
pub fn input_generator(input: &str) -> Vec<String> {
    input
        .lines()
        .map(str::to_owned)
        .collect()
}

fn get_id(pass: &str) -> u32 {
    let (mut row_low, mut row_high) = (0, 128);
    let (mut col_low, mut col_high) = (0, 8);

    for op in pass.chars() {
        let row_mid = (row_low + row_high) / 2;
        let col_mid = (col_low + col_high) / 2;
        match op {
            'F' => row_high = row_mid,
            'B' => row_low = row_mid,
            'L' => col_high = col_mid,
            'R' => col_low = col_mid,
            _ => println!("Bad string"),
        }
    }
    return row_low * 8 + col_low;
}

#[aoc(day5, part1)]
pub fn solve_part1(input: &Vec<String>) -> u32 {
    input
        .into_iter()
        .map(|l| get_id(&l))
        .fold(0, |max, id| cmp::max(max, id))
}

#[aoc(day5, part2)]
pub fn solve_part2(input: &Vec<String>) -> u32 {
    let (min, max) = input
        .into_iter()
        .map(|l| get_id(&l))
        .fold((1024 + 128, 0), |acc, id| (cmp::min(acc.0, id), cmp::max(acc.1, id)));

    let ids: HashSet<u32> = input
        .into_iter()
        .map(|l| get_id(&l))
        .collect();
    for id in min..=max {
        if !ids.contains(&id) {
            return id;
        }
    }

    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_01() {
        let data = "FBFBBFFRLR";
        assert_eq!(solve_part1(&input_generator(data)), 357);
    }

    #[test]
    fn sample_02() {
        let data = "BFFFBBFRRR\nFFFBBBFRRR\nBBFFBBFRLL";
        assert_eq!(solve_part1(&input_generator(data)), 820);
    }
}
