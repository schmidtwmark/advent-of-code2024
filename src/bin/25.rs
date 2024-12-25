use aoc::Solver;
use itertools::{Either, Itertools};

type Answer = usize;

const PART_ONE_SAMPLE_ANSWER: Answer = 3;
const PART_TWO_SAMPLE_ANSWER: Answer = 0;

struct Solution {}
impl Solver<'_, Answer> for Solution {
    fn solve_part_one(&self, lines: &[&str]) -> Answer {
        let grids = lines
            .split(|line| line.is_empty())
            .map(|chunk| aoc::Grid::from_lines(chunk, &|c| c));

        let (keys, locks): (Vec<_>, Vec<_>) =
            grids.partition_map(|grid| match *grid.at((0, 0)) == '#' {
                true => Either::Left(grid),
                false => Either::Right(grid),
            });

        keys.into_iter()
            .cartesian_product(locks.iter())
            .filter(|(key, lock)| {
                for x in 0..key.width {
                    for y in 0..key.height {
                        if *key.get((x, y)).unwrap() == '#' && *lock.get((x, y)).unwrap() == '#' {
                            return false;
                        }
                    }
                }
                true
            })
            .count()
    }

    fn solve_part_two(&self, lines: &[&str]) -> Answer {
        Answer::default()
    }
}

fn main() {
    let sample = include_str!("../../samples/25.txt");
    let input = include_str!("../../inputs/25.txt");
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
