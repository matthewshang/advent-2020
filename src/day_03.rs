use aoc_runner_derive::{aoc, aoc_generator};

pub struct Map {
    width: usize,
    height: usize,
    data: Vec<bool>,
}

impl Map {
    fn tree_at(&self, row: usize, col: usize) -> bool {
        let actual_col = col % self.width;
        return self.data[self.width * row + actual_col];
    }
}

#[aoc_generator(day3)]
pub fn input_generator(input: &str) -> Option<Map> {
    let lines: Vec<&str> = input.lines().collect();
    let height = lines.len();
    assert!(height > 0);
    let width = lines[0].len();
    assert!(width > 0);

    let mut data = vec![false; width * height];
    for row in 0..height {
        for col in 0..width {
            let is_tree = lines[row].chars().nth(col).unwrap() == '#';
            data[width * row + col] = is_tree;
        }
    }

    Some(Map {
        width: width,
        height: height,
        data: data
    })
}

struct Step {
    down: usize,
    right: usize,
}

fn count_trees(input: &Map, step: &Step) -> u64 {
    let (mut row, mut col) = (step.down, step.right);
    let mut count = 0;
    while row < input.height {
        if input.tree_at(row, col) {
            count += 1;
        }
        row += step.down;
        col += step.right;
    }

    count
}

#[aoc(day3, part1)]
pub fn solve_part1(input: &Map) -> u64 {
    count_trees(input, &Step { down: 1, right: 3 })
}

#[aoc(day3, part2)]
pub fn solve_part2(input: &Map) -> u64 {
    const STEPS: [Step; 5] = [
        Step { down: 1, right: 1 }, 
        Step { down: 1, right: 3 }, 
        Step { down: 1, right: 5 }, 
        Step { down: 1, right: 7 }, 
        Step { down: 2, right: 1 }];

    STEPS
        .iter()
        .map(|s| count_trees(input, s))
        .fold(1, |acc, count| acc * count)
}
