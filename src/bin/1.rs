use aoc::Solver;
use itertools::Itertools;

fn get_lists(lines: &[&str]) -> (Vec<usize>, Vec<usize>) {
    lines
        .iter()
        .filter_map(|l| l.split_once("   "))
        .map(|(a, b)| (a.parse::<usize>().unwrap(), b.parse::<usize>().unwrap()))
        .unzip()
}

struct Solution {}
impl Solver<'_, usize> for Solution {
    fn solve_part_one(&self, lines: &[&str]) -> usize {
        let (mut a, mut b) = get_lists(lines);

        a.sort();
        b.sort();

        a.iter().zip(b).map(|(x, y)| x.abs_diff(y)).sum()
    }

    fn solve_part_two(&self, lines: &[&str]) -> usize {
        let (a, b) = get_lists(lines);
        let b_counts = b.iter().counts();

        a.iter().map(|x| b_counts.get(x).unwrap_or(&0) * x).sum()
    }
}

fn main() {
    let sample = include_str!("../../samples/1.txt");
    let input = include_str!("../../inputs/1.txt");
    let part_one_problems = [
        aoc::Input::new_sample(sample, 11),
        aoc::Input::new_final(input),
    ];

    let part_two_problems = [
        aoc::Input::new_sample(sample, 31),
        aoc::Input::new_final(input),
    ];

    Solution {}.run(&part_one_problems, &part_two_problems);
}
