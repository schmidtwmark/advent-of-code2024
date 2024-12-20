use std::collections::{HashMap, HashSet};

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

fn count_is_valid_cache<'a>(
    pattern: &'a str,
    designs: &HashMap<&'a str, usize>,
    cache: &mut HashMap<&'a str, usize>,
) -> usize {
    debug!("Checking pattern: {}", pattern);
    if pattern.is_empty() {
        return 1;
    }
    if let Some(count) = cache.get(pattern) {
        return *count;
    }
    if let Some(count) = designs.get(pattern) {
        return *count;
    }
    let mut total = 0;
    for (design, _) in designs.iter() {
        if let Some(back) = pattern.strip_prefix(design) {
            debug!("Found combination: {} + {}", design, back);
            if let Some(back_count) = cache.get(pattern) {
                total += back_count
            } else {
                total += count_is_valid_cache(back, designs, cache);
            }
        }
    }
    debug!("Inserting {pattern}: {total} into cache");
    cache.insert(pattern, total);
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
                let count = count_is_valid_cache(pattern, &designs, &mut HashMap::new());
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
