use std::collections::{HashMap, HashSet};

use aoc::Solver;
use itertools::Itertools;
use log::debug;

struct Solution {}
impl Solver<'_, usize> for Solution {
    fn solve_part_one(&self, lines: &[&str]) -> usize {
        let (rules, strings) = lines.split(|s| s.is_empty()).collect_tuple().unwrap();
        let rules: Vec<(usize, usize)> = rules
            .iter()
            .map(|s| {
                // Parse rule as two ints
                let (a, b) = s.split_once("|").unwrap();
                let a = a.parse::<usize>().unwrap();
                let b = b.parse::<usize>().unwrap();
                (a, b)
            })
            .collect();

        let afters: HashMap<usize, HashSet<&usize>> =
            rules.iter().fold(HashMap::new(), |mut map, (to, from)| {
                // Add rule to map
                map.entry(*to).or_default().insert(from);
                map
            });
        let befores: HashMap<usize, HashSet<&usize>> =
            rules.iter().fold(HashMap::new(), |mut map, (to, from)| {
                // Add rule to map
                map.entry(*from).or_default().insert(to);
                map
            });

        let updates: Vec<Vec<usize>> = strings
            .iter()
            .map(|s| s.split(",").map(|s| s.parse().unwrap()).collect())
            .collect_vec();

        updates
            .iter()
            .map(|update| {
                // Check if update satisfies rules
                if update.iter().enumerate().all(|(idx, item)| {
                    let empty = HashSet::new();
                    let must_come_before = befores.get(item).unwrap_or(&empty);
                    let must_come_after = afters.get(item).unwrap_or(&empty);
                    let items_before = update[..idx].iter().collect::<HashSet<_>>();
                    let items_after = update[idx + 1..].iter().collect::<HashSet<_>>();
                    items_before.is_subset(must_come_before)
                        && items_after.is_subset(must_come_after)
                }) {
                    // middle item of update
                    update[update.len() / 2]
                } else {
                    0
                }
            })
            .sum()
    }

    fn solve_part_two(&self, lines: &[&str]) -> usize {
        let (rules, strings) = lines.split(|s| s.is_empty()).collect_tuple().unwrap();
        let rules: Vec<(usize, usize)> = rules
            .iter()
            .map(|s| {
                // Parse rule as two ints
                let (a, b) = s.split_once("|").unwrap();
                let a = a.parse::<usize>().unwrap();
                let b = b.parse::<usize>().unwrap();
                (a, b)
            })
            .collect();

        let afters: HashMap<usize, HashSet<&usize>> =
            rules.iter().fold(HashMap::new(), |mut map, (to, from)| {
                // Add rule to map
                map.entry(*to).or_default().insert(from);
                map
            });
        let befores: HashMap<usize, HashSet<&usize>> =
            rules.iter().fold(HashMap::new(), |mut map, (to, from)| {
                // Add rule to map
                map.entry(*from).or_default().insert(to);
                map
            });

        let updates: Vec<Vec<usize>> = strings
            .iter()
            .map(|s| s.split(",").map(|s| s.parse().unwrap()).collect())
            .collect_vec();

        updates
            .iter()
            .map(|update| {
                // Check if update satisfies rules
                let empty = HashSet::new();
                if update.iter().enumerate().all(|(idx, item)| {
                    let must_come_before = befores.get(item).unwrap_or(&empty);
                    let must_come_after = afters.get(item).unwrap_or(&empty);
                    let items_before = update[..idx].iter().collect::<HashSet<_>>();
                    let items_after = update[idx + 1..].iter().collect::<HashSet<_>>();
                    items_before.is_subset(must_come_before)
                        && items_after.is_subset(must_come_after)
                }) {
                    0
                } else {
                    // Fix the update and return middle item
                    let mut new_update = Vec::<usize>::new();
                    let mut update_clone = update.clone();
                    while new_update.len() < update.len() {
                        // Find something with all dependencies satisfied if it went in this position
                        let before_set = new_update.iter().collect::<HashSet<_>>();
                        if let Some((idx, next)) =
                            update_clone.iter().enumerate().find(|(idx, u)| {
                                let must_come_before = befores.get(u).unwrap_or(&empty);
                                let must_come_after = afters.get(u).unwrap_or(&empty);
                                let mut temp = update_clone.clone();
                                temp.remove(*idx);
                                let without_candidate = temp.iter().collect::<HashSet<_>>();

                                // debug!(
                                //     "Item: {u},Before set: {:?}, must come before: {:?}, must come after: {:?}, without candidate: {:?}", 
                                //     before_set, must_come_before, must_come_after, without_candidate);

                                must_come_before.is_superset(&before_set)
                                    && must_come_after.is_superset(&without_candidate)
                            })
                        {
                            new_update.push(*next);
                            update_clone.remove(idx);
                        } else {
                            panic!(
                                "Failed to find next for update: {:?} new_update: {:?}, clone: {:?}",
                                update, new_update, update_clone
                            );
                        }
                    }

                    debug!(
                        "Original update: {:?}, new update: {:?}",
                        update, new_update
                    );
                    new_update[new_update.len() / 2]
                }
            })
            .sum()
    }
}

fn main() {
    let sample = include_str!("../../samples/5.txt");
    let input = include_str!("../../inputs/5.txt");
    let part_one_problems = [
        aoc::Input::new_sample(sample, 143),
        aoc::Input::new_final(input),
    ];

    let part_two_problems = [
        aoc::Input::new_sample(sample, 123),
        aoc::Input::new_final(input),
    ];

    Solution {}.run(&part_one_problems, &part_two_problems);
}
