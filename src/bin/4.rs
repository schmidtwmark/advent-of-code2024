use std::collections::HashSet;

use aoc::{Grid, Solver};
use itertools::Itertools;
use log::debug;

fn count_string_at_position(start_pos: (usize, usize), grid: &Grid<char>, string: &str) -> usize {
    let start_char = grid.at(start_pos);
    let mut chars = string.chars();
    if let Some(first) = chars.next() {
        if first == *start_char {
            grid.neighbors_along_directions(start_pos)
                .into_iter()
                .map(|neighbors| {
                    let dir_string = neighbors
                        .take(string.len() - 1)
                        .map(|n| grid.at(n))
                        .collect::<String>();
                    if dir_string == string[1..] {
                        1
                    } else {
                        0
                    }
                })
                .sum()
        } else {
            0
        }
    } else {
        1
    }
}

fn count_x_at_position(start_pos: (usize, usize), grid: &Grid<char>) -> usize {
    let start_char = grid.at(start_pos);
    if *start_char == 'M' || *start_char == 'S' {
        let subgrid = grid.get_subgrid(start_pos, 3, 3);
        if subgrid.get((1, 1)) == Some(&'A') {
            // check diagonals
            let d1 = [subgrid.get((0, 0)), subgrid.get((2, 2))];
            let d2 = [subgrid.get((2, 0)), subgrid.get((0, 2))];
            let first = d1.iter().collect::<HashSet<_>>();
            let second = d2.iter().collect::<HashSet<_>>();
            let expected = [Some(&'M'), Some(&'S')].iter().collect::<HashSet<_>>();
            if first == expected && second == expected {
                return 1;
            } else {
                return 0;
            }
        }
        0
    } else {
        0
    }
}

struct Solution {}
impl Solver<'_, usize> for Solution {
    fn solve_part_one(&self, lines: &[&str]) -> usize {
        let grid = Grid::from_lines(lines, &|c| c);
        grid.positions()
            .map(|pos| count_string_at_position(pos, &grid, "XMAS"))
            .sum()
    }

    fn solve_part_two(&self, lines: &[&str]) -> usize {
        let grid = Grid::from_lines(lines, &|c| c);
        grid.positions()
            .map(|pos| count_x_at_position(pos, &grid))
            .sum()
    }
}

fn main() {
    let sample = include_str!("../../samples/4.txt");
    let input = include_str!("../../inputs/4.txt");
    let part_one_problems = [
        aoc::Input::new_sample(sample, 18),
        aoc::Input::new_final(input),
    ];

    let part_two_problems = [
        aoc::Input::new_sample(sample, 9),
        aoc::Input::new_final(input),
    ];

    Solution {}.run(&part_one_problems, &part_two_problems);
}
