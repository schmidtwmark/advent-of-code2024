use std::{
    collections::{HashMap, HashSet},
    ops::BitXor,
};

use aoc::Solver;
use itertools::Itertools;
use log::{debug, info};

type Answer = usize;

const PART_ONE_SAMPLE_ANSWER: Answer = 37327623;
const PART_TWO_SAMPLE_ANSWER: Answer = 25;

fn process(mut n: usize) -> usize {
    // Calculate the result of multiplying the secret number by 64.
    // Then, mix this result into the secret number.
    // Finally, prune the secret number.
    let a = 64 * n;
    n = a.bitxor(n);
    n %= 16777216;

    // Calculate the result of dividing the secret number by 32.
    // Round the result down to the nearest integer.
    //Then, mix this result into the secret number.
    //Finally, prune the secret number.
    let b = n / 32;
    n = b.bitxor(n);
    n %= 16777216;

    // Calculate the result of multiplying the secret number by 2048.
    // Then, mix this result into the secret number.
    // Finally, prune the secret number
    let c = 2048 * n;
    n = c.bitxor(n);
    n %= 16777216;

    n
}

fn step(n: usize, level: usize) -> usize {
    if level == 0 {
        return n;
    }

    let n = process(n);
    step(n, level - 1)
}

fn sequence(n: usize, level: usize) -> Vec<(i16, Option<i16>)> {
    let mut x = n;
    let mut prev = (n % 10) as i16;
    let s = (0..level).map(|_| {
        x = process(x);
        let modulo = (x % 10) as i16;
        let out = (modulo, Some(modulo - prev));
        prev = modulo;
        out
    });
    let s = [((n % 10) as i16, None)]
        .iter()
        .copied()
        .chain(s)
        .collect_vec();

    debug!("{n} => {:?}", s);
    s
}
struct Solution {}
impl Solver<'_, Answer> for Solution {
    fn solve_part_one(&self, lines: &[&str]) -> Answer {
        let nums = lines.iter().map(|s| s.parse::<usize>().unwrap());

        let steps = 2000;

        nums.map(|n| step(n, steps)).sum()
    }

    fn solve_part_two(&self, lines: &[&str]) -> Answer {
        let steps = 2000;
        let sequences = lines
            .iter()
            .map(|s| {
                let sequence = sequence(s.parse().unwrap(), steps);
                let map = sequence.iter().tuple_windows().fold(
                    HashMap::new(),
                    |mut map, (a, b, c, d)| {
                        let prefix = [a.1, b.1, c.1, d.1];
                        if !map.contains_key(&prefix) {
                            *map.entry(prefix).or_default() = d.0;
                        }
                        map
                    },
                );

                map
            })
            .collect_vec();

        let mut checked = HashSet::new();
        let mut best = 0;
        let mut best_sequence = [Some(0), Some(0), Some(0), Some(0)];
        for sequence in sequences.iter() {
            for prefix in sequence.keys() {
                if prefix.iter().any(|x| x.is_none()) {
                    continue;
                }
                if checked.contains(&prefix) {
                    continue;
                }

                checked.insert(prefix);

                let sum = sequences
                    .iter()
                    .map(|s| s.get(prefix).unwrap_or(&0))
                    .sum::<i16>();

                if sum > best {
                    best_sequence = *prefix;
                    best = sum;
                }
            }
        }
        info!("Best sequence: {:?} => {}", best_sequence, best);

        best as usize
    }
}

fn main() {
    let sample = include_str!("../../samples/22.txt");
    let sample_1 = include_str!("../../samples/22_1.txt");
    let sample_2 = include_str!("../../samples/22_2.txt");
    let input = include_str!("../../inputs/22.txt");
    let part_one_problems = [
        aoc::Input::new_sample(sample, PART_ONE_SAMPLE_ANSWER),
        aoc::Input::new_sample(sample_1, 1110806),
        aoc::Input::new_final(input),
    ];

    let part_two_problems = [
        aoc::Input::new_sample(sample, PART_TWO_SAMPLE_ANSWER),
        aoc::Input::new_sample(sample_1, 9),
        aoc::Input::new_sample(sample_2, 23),
        aoc::Input::new_final(input),
        // 1464 too high
        // 1459 too high
        // 1450 too high
    ];

    Solution {}.run(&part_one_problems, &part_two_problems);
}
