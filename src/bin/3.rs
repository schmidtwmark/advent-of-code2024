use aoc::Solver;
use log::debug;
use regex::Regex;

struct Solution {}
impl Solver<'_, usize> for Solution {
    fn solve_part_one(&self, lines: &[&str]) -> usize {
        lines
            .iter()
            .map(|line| {
                let regex = Regex::new(r"mul\(([0-9]+),([0-9]+)\)").unwrap();
                regex
                    .captures_iter(line)
                    .map(|c| {
                        let a: usize = c.get(1).unwrap().as_str().parse().unwrap();
                        let b: usize = c.get(2).unwrap().as_str().parse().unwrap();
                        a * b
                    })
                    .sum::<usize>()
            })
            .sum()
    }

    fn solve_part_two(&self, lines: &[&str]) -> usize {
        let mut enabled = true;
        lines
            .iter()
            .map(|line| {
                let regex = Regex::new(r"(do\(\))|(don't\(\))|(mul\(([0-9]+),([0-9]+)\))").unwrap();
                regex
                    .captures_iter(line)
                    .map(|c| {
                        debug!("{:?}", c);
                        if let Some(enable) = c.get(1) {
                            if enable.as_str() == "do()" {
                                enabled = true
                            }
                        }

                        if let Some(disable) = c.get(2) {
                            if disable.as_str() == "don't()" {
                                enabled = false
                            }
                        }

                        if enabled && c.get(3).is_some() {
                            let a: usize = c.get(4).unwrap().as_str().parse().unwrap();
                            let b: usize = c.get(5).unwrap().as_str().parse().unwrap();
                            a * b
                        } else {
                            0
                        }
                    })
                    .sum::<usize>()
            })
            .sum()
    }
}

fn main() {
    let sample = include_str!("../../samples/3.txt");
    let sample_2 = include_str!("../../samples/3_2.txt");
    let input = include_str!("../../inputs/3.txt");
    let part_one_problems = [
        aoc::Input::new_sample(sample, 161),
        aoc::Input::new_final(input),
        // 24286181 too low
    ];

    let part_two_problems = [
        aoc::Input::new_sample(sample_2, 48),
        aoc::Input::new_final(input),
    ];

    Solution {}.run(&part_one_problems, &part_two_problems);
}
