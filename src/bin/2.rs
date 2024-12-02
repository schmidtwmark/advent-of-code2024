use std::str::FromStr;

use aoc::Solver;
use itertools::Itertools;

struct Report {
    levels: Vec<u32>,
}

impl Report {
    fn is_safe(&self) -> bool {
        let ascending_or_descending =
            self.levels.is_sorted() || self.levels.is_sorted_by(|a, b| b <= a);
        if !ascending_or_descending {
            return false;
        }

        self.levels.iter().tuple_windows().all(|(a, b)| {
            let diff = a.abs_diff(*b);
            (1..=3).contains(&diff)
        })
    }

    fn modulated(&self) -> Vec<Report> {
        self.levels
            .iter()
            .enumerate()
            .map(|(idx, _)| {
                let mut copy = self.levels.clone();
                copy.remove(idx);
                Report { levels: copy }
            })
            .collect()
    }
}

impl FromStr for Report {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Report {
            levels: s.split(" ").filter_map(|s| s.parse().ok()).collect(),
        })
    }
}

struct Solution {}
impl Solver<'_, usize> for Solution {
    fn solve_part_one(&self, lines: &[&str]) -> usize {
        let reports: Vec<Report> = lines.iter().map(|s| s.parse().unwrap()).collect_vec();

        reports.iter().filter(|r| r.is_safe()).count()
    }

    fn solve_part_two(&self, lines: &[&str]) -> usize {
        let reports: Vec<Report> = lines.iter().map(|s| s.parse().unwrap()).collect_vec();

        reports
            .iter()
            .filter(|r| r.modulated().iter().any(|r| r.is_safe()))
            .count()
    }
}

fn main() {
    let sample = include_str!("../../samples/2.txt");
    let input = include_str!("../../inputs/2.txt");
    let part_one_problems = [
        aoc::Input::new_sample(sample, 2),
        aoc::Input::new_final(input),
    ];

    let part_two_problems = [
        aoc::Input::new_sample(sample, 4),
        aoc::Input::new_final(input),
    ];

    Solution {}.run(&part_one_problems, &part_two_problems);
}
