use std::collections::HashMap;

use aoc::Solver;
use itertools::Itertools;
use log::debug;

type Answer = usize;

const PART_ONE_SAMPLE_ANSWER: Answer = 55312;
const PART_TWO_SAMPLE_ANSWER: Answer = 0;

fn blink_value(value: usize) -> (usize, Option<usize>) {
    if value == 0 {
        (1, None)
    } else if (value.ilog10() + 1) % 2 == 0 {
        // even digits, split in half
        let str = value.to_string();
        let left = str[..str.len() / 2].parse().unwrap();
        let right = str[str.len() / 2..].parse().unwrap();
        (left, Some(right))
    } else {
        (value * 2024, None)
    }
}

// return the number of values generated from this one by the end of iteration
fn process_value(
    value: usize,
    start_iteration: usize,
    end_iteration: usize,
    seen: &mut HashMap<(usize, usize), usize>,
) -> usize {
    // Map a value to the iteration it was generated on
    let mut extras = Vec::new();
    let mut current = value;
    let mut count = 1;
    for i in start_iteration..end_iteration {
        let (new_value, extra) = blink_value(current);
        // debug!("Iteration {}: {} -> {:?}", i, current, (new_value, extra));
        if let Some(extra) = extra {
            if let Some(extra_count) = seen.get(&(extra, i + 1)) {
                count += extra_count;
            } else {
                extras.push((extra, i + 1));
            }
        }
        current = new_value;
    }
    count += extras
        .into_iter()
        .map(|(start_val, iteration)| process_value(start_val, iteration, end_iteration, seen))
        .sum::<usize>();
    seen.insert((value, start_iteration), count);
    count
}

struct Solution {}
impl Solver<'_, usize> for Solution {
    fn solve_part_one(&self, lines: &[&str]) -> Answer {
        let mut seen = HashMap::new();
        let values: Vec<usize> = lines[0]
            .split(" ")
            .map(|s| s.parse().unwrap())
            .collect_vec();
        values
            .into_iter()
            .map(|v| process_value(v, 0, 25, &mut seen))
            .sum()
    }

    fn solve_part_two(&self, lines: &[&str]) -> Answer {
        let mut seen = HashMap::new();
        let values: Vec<usize> = lines[0]
            .split(" ")
            .map(|s| s.parse().unwrap())
            .collect_vec();
        values
            .into_iter()
            .map(|v| process_value(v, 0, 75, &mut seen))
            .sum()
    }
}

fn main() {
    let sample = include_str!("../../samples/11.txt");
    let input = include_str!("../../inputs/11.txt");
    let part_one_problems = [
        aoc::Input::new_sample(sample, PART_ONE_SAMPLE_ANSWER),
        aoc::Input::new_final(input),
    ];

    let part_two_problems = [
        aoc::Input::new_sample(sample, PART_TWO_SAMPLE_ANSWER),
        aoc::Input::new_final(input),
    ];

    Solution {}.run(&part_one_problems, &part_two_problems);
}
