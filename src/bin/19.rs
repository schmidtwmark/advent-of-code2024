use std::collections::{HashMap, HashSet, VecDeque};

use aoc::Solver;
use itertools::Itertools;
use log::{debug, info};

type Answer = usize;

const PART_ONE_SAMPLE_ANSWER: Answer = 6;
const PART_TWO_SAMPLE_ANSWER: Answer = 16;

fn parse<'a>(lines: &'a [&str]) -> (HashSet<&'a str>, Vec<&'a str>) {
    let designs = lines[0];

    // Add empty string
    let designs: HashSet<&str> = designs.split(", ").collect();

    let targets = lines
        .iter()
        .skip(2)
        .map(std::ops::Deref::deref)
        .collect_vec();

    (designs, targets)
}

fn is_valid<'a>(
    pattern: &'a str,
    designs: &HashSet<&'a str>,
    mut used_patterns: Vec<&'a str>,
) -> Option<Vec<&'a str>> {
    // debug!("Checking pattern: {}", pattern);
    if designs.contains(pattern) {
        used_patterns.push(pattern);
        return Some(used_patterns);
    }
    (1..pattern.len()).find_map(|i| {
        let front = &pattern[0..i];
        let back = &pattern[i..];
        // debug!("Checking front: {}, back: {}", front, back);
        if designs.contains(front) {
            let next_pattern = used_patterns.clone();
            is_valid(back, designs, next_pattern)
        } else {
            None
        }
    })
}

fn count_is_valid<'a>(pattern: &'a str, designs: &HashMap<&'a str, usize>) -> usize {
    // if let Some(count) = designs.get(pattern) {
    //     *count
    // } else {
    //     for i in 1..pattern.len() {
    //         let front = &pattern[0..i];
    //         let back = &pattern[i..];
    //         if let Some(count) = designs.get(front).copied() {
    //             let back_count = count_is_valid(back, designs);
    //             if back_count > 0 {
    //                 designs.insert(pattern, 1 + count * back_count);
    //             }
    //             return 0;
    //         }
    //     }
    //     0
    // }
    debug!("Checking pattern: {}", pattern);
    if pattern.is_empty() {
        return 1;
    }
    if let Some(count) = designs.get(pattern) {
        return *count;
    }
    let mut total = 0;
    for (design, _) in designs.iter() {
        if let Some(back) = pattern.strip_prefix(design) {
            debug!("Found combination: {} + {}", design, back);
            total += count_is_valid(back, designs);
        }
    }
    total
}

struct Solution {}
impl Solver<'_, Answer> for Solution {
    fn solve_part_one(&self, lines: &[&str]) -> Answer {
        let (designs, targets) = parse(lines);

        targets
            .iter()
            .filter(|pattern| {
                let result = is_valid(pattern, &designs, Vec::new());
                if let Some(result) = result {
                    debug!("Found result for pattern: {}: {:?}", pattern, result);
                    true
                } else {
                    false
                }
            })
            .count()
    }

    fn solve_part_two(&self, lines: &[&str]) -> Answer {
        let (designs, targets) = parse(lines);

        let designs = designs.iter().sorted_by(|a, b| a.len().cmp(&b.len())).fold(
            HashMap::new(),
            |mut map, design| {
                let count = count_is_valid(design, &map);
                map.insert(design, count + 1);
                map
            },
        );

        targets
            .iter()
            .map(|pattern| {
                info!("Checking pattern: {}", pattern);
                let count = count_is_valid(pattern, &designs);
                // let mut queue = VecDeque::new();
                // let mut count = 0;
                // queue.push_back((*pattern, 1));

                // while let Some((pattern, p_count)) = queue.pop_front() {
                //     for i in 1usize..pattern.len() {
                //         let front = &pattern[0..i];
                //         let back = &pattern[i..];

                //         match (designs.get(front).copied(), designs.get(back).copied()) {
                //             (Some(front_count), Some(back_count)) => {
                //                 debug!(
                //                     "Found result for pattern: {}: {} * {}",
                //                     pattern, front_count, back_count,
                //                 );
                //                 let total_count = front_count * back_count * p_count;
                //                 designs.insert(pattern, total_count);
                //                 count += total_count;
                //             }
                //             (Some(_front_count), None) => {
                //                 queue.push_back((back, p_count));
                //             }
                //             _ => {}
                //         }
                //     }
                // }

                info!("Found result for pattern: {}: {:?}", pattern, count);
                count
            })
            .sum()
    }
}

fn main() {
    let sample = include_str!("../../samples/19.txt");
    let input = include_str!("../../inputs/19.txt");
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
