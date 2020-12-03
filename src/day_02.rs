use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day2)]
fn input_generator(input: &str) -> Vec<String> {
    input
        .lines()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .map(|l| l.parse().unwrap())
        .collect()
}

#[aoc(day2, part1)]
pub fn solve_part1(input: &Vec<String>) -> u32 {
    let mut valid_count: u32 = 0;
    for line in input {
        let split = line.split(" ");
        let words: Vec<&str> = split.collect();
        assert!(words.len() == 3);

        let mut split = words[0].split("-");
        let policy_low: u32 = split.next().unwrap().parse().unwrap();
        let policy_high: u32 = split.next().unwrap().parse().unwrap();
        let policy_letter: char = words[1].chars().nth(0).unwrap();

        let mut letter_count: u32 = 0;
        for letter in words[2].chars() {
            if letter == policy_letter {
                letter_count += 1;
            }
        }

        if policy_low <= letter_count && letter_count <= policy_high {
            valid_count += 1;
        }
    }

    valid_count
}

#[aoc(day2, part2)]
pub fn solve_part2(input: &Vec<String>) -> u32 {
    let mut valid_count: u32 = 0;
    for line in input {
        let split = line.split(" ");
        let words: Vec<&str> = split.collect();
        assert!(words.len() == 3);

        let mut split = words[0].split("-");
        let policy_first: usize = split.next().unwrap().parse().unwrap();
        let policy_second: usize = split.next().unwrap().parse().unwrap();
        let policy_letter: char = words[1].chars().nth(0).unwrap();

        let chars: Vec<char> = words[2].chars().collect();
        let first_has = chars[policy_first - 1] == policy_letter;
        let second_has = chars[policy_second - 1] == policy_letter;
        if first_has ^ second_has {
            valid_count += 1;
        }
    }

    valid_count
}
