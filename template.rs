use aoc::Solver;
use itertools::Itertools;

type Answer = usize;

const PART_ONE_SAMPLE_ANSWER: Answer = 0;
const PART_TWO_SAMPLE_ANSWER: Answer = 0;

struct Solution {}
impl Solver<'_, usize> for Solution {
    fn solve_part_one(&self, lines: &[&str]) -> Answer {
        Answer::default()
    }

    fn solve_part_two(&self, lines: &[&str]) -> Answer {
        Answer::default()
    }
}

fn main() {
    let sample = include_str!("../../samples/aaaaa.txt");
    let input = include_str!("../../inputs/aaaaa.txt");
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
