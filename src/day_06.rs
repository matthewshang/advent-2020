use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::{ HashMap, HashSet };

#[aoc_generator(day6)]
pub fn input_generator(input: &str) -> Vec<String> {
    input
        .split("\n\n")
        .map(str::to_owned)
        .collect()
}

#[aoc(day6, part1)]
pub fn solve_part1(input: &Vec<String>) -> u32 {
    input
        .into_iter()
        .map(|group| {
            let mut chars: HashSet<char> = HashSet::new();
            group
                .lines()
                .for_each(|line| {
                    line
                        .chars()
                        .for_each(|c| { chars.insert(c); });
                });
            chars.len() as u32
        })
        .fold(0, |acc, sz| acc + sz)
}

#[aoc(day6, part2)]
pub fn solve_part2(input: &Vec<String>) -> usize {
    input
        .into_iter()
        .map(|group| {
            let mut char_count: HashMap<char, usize> = HashMap::new();
            group
                .lines()
                .for_each(|line| {
                    line
                        .chars()
                        .for_each(|c| { 
                            if !char_count.contains_key(&c) {
                                char_count.insert(c, 0);
                            }
                            let count = char_count.get_mut(&c).unwrap();
                            *count += 1;
                        });
                });
            let num_people = group.lines().count();
            char_count
                .iter()
                .filter(|(_, v)| **v == num_people)
                .count()
        })
        .fold(0, |acc, sz| acc + sz)
}
