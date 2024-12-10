use std::collections::{HashMap, HashSet, VecDeque};

use aoc::Solver;
use log::debug;

type Answer = usize;

const PART_ONE_SAMPLE_ANSWER: Answer = 36;
const PART_TWO_SAMPLE_ANSWER: Answer = 81;

#[derive(Debug, Clone, Eq, PartialEq, Default)]
enum GridObject {
    #[default]
    Empty,
    Height(u8),
}

impl GridObject {
    fn from_char(c: char) -> Self {
        match c {
            '.' => GridObject::Empty,
            _ => GridObject::Height(c.to_digit(10).unwrap() as u8),
        }
    }
}

struct Solution {}
impl Solver<'_, usize> for Solution {
    fn solve_part_one(&self, lines: &[&str]) -> Answer {
        let grid = aoc::Grid::from_lines(lines, &GridObject::from_char);
        grid.positions()
            .filter_map(|pos| match grid.at(pos) {
                GridObject::Empty => None,
                GridObject::Height(0) => {
                    let mut queue = VecDeque::new();
                    let mut visited = HashSet::new();
                    let mut trailends = HashSet::new();
                    visited.insert(pos);
                    queue.push_back(pos);
                    while let Some(current) = queue.pop_front() {
                        if let GridObject::Height(current_height) = grid.at(current) {
                            let neighbors = grid.cardinal_neighbor_positions(current);
                            neighbors.iter().for_each(|neighbor| {
                                if !visited.contains(neighbor) {
                                    if let GridObject::Height(height) = grid.at(*neighbor) {
                                        if *height == current_height + 1 {
                                            visited.insert(*neighbor);
                                            if *height == 9 {
                                                trailends.insert(*neighbor);
                                            } else {
                                                queue.push_back(*neighbor);
                                            }
                                        }
                                    }
                                }
                            });
                        }
                    }

                    Some((pos, trailends))
                }
                GridObject::Height(_) => None,
            })
            .map(|(start, trailends)| {
                debug!("start: {:?}, ends: {:?}", start, trailends);
                trailends.len()
            })
            .sum()
    }

    fn solve_part_two(&self, lines: &[&str]) -> Answer {
        let grid = aoc::Grid::from_lines(lines, &GridObject::from_char);
        grid.positions()
            .filter_map(|pos| match grid.at(pos) {
                GridObject::Empty => None,
                GridObject::Height(0) => {
                    let mut queue = VecDeque::new();
                    // let mut visited = HashSet::new();
                    let mut trailpaths = HashMap::<(usize, usize), usize>::new();
                    // visited.insert(pos);
                    queue.push_back(pos);
                    while let Some(current) = queue.pop_front() {
                        if let GridObject::Height(current_height) = grid.at(current) {
                            let neighbors = grid.cardinal_neighbor_positions(current);
                            neighbors.iter().for_each(|neighbor| {
                                if let GridObject::Height(height) = grid.at(*neighbor) {
                                    if *height == current_height + 1 {
                                        if *height == 9 {
                                            *trailpaths.entry(*neighbor).or_default() += 1;
                                        } else {
                                            queue.push_back(*neighbor);
                                        }
                                    }
                                }
                            });
                        }
                    }

                    Some((pos, trailpaths))
                }
                GridObject::Height(_) => None,
            })
            .map(|(start, trailpaths)| {
                debug!("start: {:?}, ends: {:?}", start, trailpaths);
                trailpaths.values().sum::<usize>()
            })
            .sum()
    }
}

fn main() {
    let sample = include_str!("../../samples/10.txt");
    let input = include_str!("../../inputs/10.txt");
    let part_one_problems = [
        aoc::Input::new_sample(sample, PART_ONE_SAMPLE_ANSWER),
        aoc::Input::new_sample(include_str!("../../samples/10_1.txt"), 2),
        aoc::Input::new_sample(include_str!("../../samples/10_2.txt"), 4),
        aoc::Input::new_sample(include_str!("../../samples/10_3.txt"), 3),
        aoc::Input::new_final(input),
    ];

    let part_two_problems = [
        aoc::Input::new_sample(sample, PART_TWO_SAMPLE_ANSWER),
        aoc::Input::new_sample(include_str!("../../samples/10_4.txt"), 3),
        aoc::Input::new_final(input),
    ];

    Solution {}.run(&part_one_problems, &part_two_problems);
}
