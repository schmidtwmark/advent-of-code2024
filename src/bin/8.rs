use std::collections::{HashMap, HashSet};

use aoc::{Grid, Solver};
use itertools::Itertools;
use log::debug;

type Answer = usize;

const PART_ONE_SAMPLE_ANSWER: Answer = 14;
const PART_TWO_SAMPLE_ANSWER: Answer = 34;

#[derive(Debug, Clone, Eq, PartialEq, Default)]
enum GridObject {
    #[default]
    Empty,
    Antenna(char),
}

struct Solution {}
impl Solver<'_, usize> for Solution {
    fn solve_part_one(&self, lines: &[&str]) -> Answer {
        let grid = Grid::from_lines(lines, &|c| match c {
            '.' => GridObject::Empty,
            c => GridObject::Antenna(c),
        });
        let antennae: HashMap<char, HashSet<(usize, usize)>> =
            grid.positions().fold(HashMap::new(), |mut map, pos| {
                if let GridObject::Antenna(c) = grid.at(pos) {
                    map.entry(*c).or_default().insert(pos);
                }
                map
            });

        let antinodes = antennae
            .iter()
            .fold(HashSet::new(), |mut map, (_c, c_antennae)| {
                debug!("c: {_c}, c_antennae: {c_antennae:?}");
                c_antennae.iter().tuple_combinations().for_each(|(a, b)| {
                    let a = (a.0 as isize, a.1 as isize);
                    let b = (b.0 as isize, b.1 as isize);
                    let run = b.0 - a.0;
                    let rise = b.1 - a.1;
                    let new_a = (a.0 - run, a.1 - rise);
                    let new_b = (b.0 + run, b.1 + rise);
                    if grid.get_isize(new_a).is_some() {
                        map.insert(new_a);
                    }
                    if grid.get_isize(new_b).is_some() {
                        map.insert(new_b);
                    }
                });
                map
            });

        antinodes.len()
    }

    fn solve_part_two(&self, lines: &[&str]) -> Answer {
        let grid = Grid::from_lines(lines, &|c| match c {
            '.' => GridObject::Empty,
            c => GridObject::Antenna(c),
        });
        let antennae: HashMap<char, HashSet<(usize, usize)>> =
            grid.positions().fold(HashMap::new(), |mut map, pos| {
                if let GridObject::Antenna(c) = grid.at(pos) {
                    map.entry(*c).or_default().insert(pos);
                }
                map
            });

        let antinodes = antennae
            .iter()
            .fold(HashSet::new(), |mut map, (_c, c_antennae)| {
                debug!("c: {_c}, c_antennae: {c_antennae:?}");
                c_antennae.iter().tuple_combinations().for_each(|(a, b)| {
                    let a = (a.0 as isize, a.1 as isize);
                    let b = (b.0 as isize, b.1 as isize);
                    let run = b.0 - a.0;
                    let rise = b.1 - a.1;
                    let mut current = a;
                    while grid.get_isize(current).is_some() {
                        map.insert(current);
                        current = (current.0 + run, current.1 + rise);
                    }
                    current = a;
                    while grid.get_isize(current).is_some() {
                        map.insert(current);
                        current = (current.0 - run, current.1 - rise);
                    }
                });
                map
            });

        antinodes.len()
    }
}

fn main() {
    let sample = include_str!("../../samples/8.txt");
    let input = include_str!("../../inputs/8.txt");
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
