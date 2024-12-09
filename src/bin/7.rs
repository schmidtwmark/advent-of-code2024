use aoc::Solver;
use log::debug;

fn is_valid(ans: usize, running_total: usize, nums: &[usize]) -> bool {
    // debug!("ans: {ans}, running_total: {running_total}, nums: {nums:?}");
    if let Some((num, remainder)) = nums.split_first() {
        let sum = running_total + num;
        let product = running_total * num;

        (sum <= ans && is_valid(ans, sum, remainder))
            || (product <= ans && is_valid(ans, product, remainder))

        // (sum < ans && is_valid(ans, sum, remainder))
        //     || (product < ans && is_valid(ans, product, remainder))
    } else {
        running_total == ans
    }
}

fn is_valid2(ans: usize, running_total: usize, nums: &[usize]) -> bool {
    // debug!("ans: {ans}, running_total: {running_total}, nums: {nums:?}");
    if let Some((num, remainder)) = nums.split_first() {
        let sum = running_total + num;
        let product = running_total * num;
        let string_concat = format!("{running_total}{num}");
        let concat = string_concat.parse::<usize>().unwrap();

        (sum <= ans && is_valid2(ans, sum, remainder))
            || (product <= ans && is_valid2(ans, product, remainder))
            || (concat <= ans && is_valid2(ans, concat, remainder))

        // (sum < ans && is_valid(ans, sum, remainder))
        //     || (product < ans && is_valid(ans, product, remainder))
    } else {
        running_total == ans
    }
}

struct Solution {}
impl Solver<'_, usize> for Solution {
    fn solve_part_one(&self, lines: &[&str]) -> usize {
        lines
            .iter()
            .filter_map(|line| {
                let (ans, nums) = line.split_once(": ").unwrap();
                let nums: Vec<usize> = nums
                    .split_whitespace()
                    .map(|s| s.parse().unwrap())
                    .collect();
                let ans = ans.parse::<usize>().unwrap();

                if let Some((num, remainder)) = nums.split_first() {
                    if is_valid(ans, *num, remainder) {
                        debug!("Valid! ans: {ans}, nums: {nums:?}");
                        Some(ans)
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .sum()
    }

    fn solve_part_two(&self, lines: &[&str]) -> usize {
        lines
            .iter()
            .filter_map(|line| {
                let (ans, nums) = line.split_once(": ").unwrap();
                let nums: Vec<usize> = nums
                    .split_whitespace()
                    .map(|s| s.parse().unwrap())
                    .collect();
                let ans = ans.parse::<usize>().unwrap();

                if let Some((num, remainder)) = nums.split_first() {
                    if is_valid2(ans, *num, remainder) {
                        debug!("Valid! ans: {ans}, nums: {nums:?}");
                        Some(ans)
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .sum()
    }
}

fn main() {
    let sample = include_str!("../../samples/7.txt");
    let input = include_str!("../../inputs/7.txt");
    let part_one_problems = [
        aoc::Input::new_sample(sample, 3749),
        aoc::Input::new_final(input),
    ];

    let part_two_problems = [
        aoc::Input::new_sample(sample, 11387),
        aoc::Input::new_final(input),
    ];

    Solution {}.run(&part_one_problems, &part_two_problems);
}
