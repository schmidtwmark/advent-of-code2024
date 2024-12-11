use std::{collections::HashMap, fmt::Debug};

use aoc::Solver;
use itertools::Itertools;
use log::debug;

type Answer = usize;

const PART_ONE_SAMPLE_ANSWER: Answer = 55312;
const PART_TWO_SAMPLE_ANSWER: Answer = 0;

struct Stone {
    val: usize,
    next: Option<Box<Stone>>,
}

impl Stone {
    fn new(items: &[usize]) -> Option<Box<Self>> {
        let (front, mut remainder) = items.split_first()?;
        let mut first = Box::new(Stone {
            val: *front,
            next: None,
        });
        let mut current = &mut first;
        while let Some((next, rem)) = remainder.split_first() {
            remainder = rem;
            let new = Box::new(Stone {
                val: *next,
                next: None,
            });
            current.next = Some(new);
            current = current.next.as_mut().unwrap();
        }
        Some(first)
    }

    fn count(start: Box<Stone>) -> usize {
        let mut count = 1;
        let mut current = &start;
        while let Some(next) = &current.next {
            count += 1;
            current = next;
        }
        count
    }
}

fn blink(stones: &mut Box<Stone>) {
    let mut current = stones;
    loop {
        if current.val == 0 {
            current.val = 1;
        } else if (current.val.ilog10() + 1) % 2 == 0 {
            // even digits, split in half
            let str = current.val.to_string();
            let left = str[..str.len() / 2].parse().unwrap();
            let right = str[str.len() / 2..].parse().unwrap();
            let new_next = Box::new(Stone {
                val: right,
                next: current.next.take(),
            });

            current.val = left;
            current.next = Some(new_next);

            current = current.next.as_mut().unwrap();
        } else {
            current.val *= 2024;
        }

        if let Some(next) = &mut current.next {
            current = next;
        } else {
            break;
        }
    }
}

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
fn process_value(value: usize, start_iteration: usize, end_iteration: usize) -> usize {
    // Map a value to the iteration it was generated on
    let mut map = HashMap::new();
    let mut current = value;
    for i in start_iteration..end_iteration {
        let (new_value, extra) = blink_value(current);
        if let Some(extra) = extra {
            map.insert(extra, i);
        }
        current = new_value;
    }

    end_iteration - start_iteration
        + map
            .into_iter()
            .map(|(start_val, iteration)| process_value(start_val, iteration, end_iteration))
            .sum::<usize>()
}

struct Solution {}
impl Solver<'_, usize> for Solution {
    fn solve_part_one(&self, lines: &[&str]) -> Answer {
        let mut stones = Stone::new(
            lines[0]
                .split(" ")
                .map(|s| s.parse().unwrap())
                .collect_vec()
                .as_slice(),
        )
        .unwrap();

        for i in 0..25 {
            debug!("On iteration {}", i);
            blink(&mut stones);
        }

        Stone::count(stones)
    }

    fn solve_part_two(&self, lines: &[&str]) -> Answer {
        let values: Vec<usize> = lines[0]
            .split(" ")
            .map(|s| s.parse().unwrap())
            .collect_vec();
        values.into_iter().map(|v| process_value(v, 0, 6)).sum()
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
