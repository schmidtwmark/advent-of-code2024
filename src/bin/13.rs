use aoc::Solver;
use itertools::Itertools;
use regex::Regex;

type Answer = isize;

const PART_ONE_SAMPLE_ANSWER: Answer = 480;
const PART_TWO_SAMPLE_ANSWER: Answer = 875318608908;

fn get_x_y(captures: regex::Captures) -> (isize, isize) {
    let x = captures.get(1).unwrap().as_str().parse().unwrap();
    let y = captures.get(2).unwrap().as_str().parse().unwrap();
    (x, y)
}

fn parse_button(line: &str) -> (isize, isize) {
    let button_regex = Regex::new(r"Button [A|B]: X\+([0-9]+), Y\+([0-9]+)").unwrap();
    get_x_y(button_regex.captures(line).unwrap())
}

fn parse_prize(line: &str) -> (isize, isize) {
    let prize_regex = Regex::new(r"Prize: X=([0-9]+), Y=([0-9]+)").unwrap();
    get_x_y(prize_regex.captures(line).unwrap())
}

fn _simple_solve(
    (a_x, a_y): (isize, isize),
    (b_x, b_y): (isize, isize),
    (p_x, p_y): (isize, isize),
) -> Option<isize> {
    (0isize..100isize)
        .cartesian_product(0isize..100isize)
        .filter_map(|(a, b)| {
            if a_x * a + b_x * b == p_x && a_y * a + b_y * b == p_y {
                Some(3 * a + b)
            } else {
                None
            }
        })
        .min()
}

fn solve(
    (a_x, a_y): (isize, isize),
    (b_x, b_y): (isize, isize),
    (p_x, p_y): (isize, isize),
) -> Option<isize> {
    let a = a_x;
    let b = b_x;
    let c = p_x;
    let d = a_y;
    let e = b_y;
    let f = p_y;

    let x_numerator = c * e - b * f;
    let x_denominator = a * e - b * d;
    let y_numerator = a * f - d * c;
    let y_denominator = a * e - b * d;

    if x_denominator == 0 || y_denominator == 0 {
        None
    } else if x_numerator % x_denominator == 0 && y_numerator % y_denominator == 0 {
        let x = x_numerator / x_denominator;
        let y = y_numerator / y_denominator;
        Some(3 * x + y)
    } else {
        None
    }
}

struct Solution {}
impl Solver<'_, Answer> for Solution {
    fn solve_part_one(&self, lines: &[&str]) -> Answer {
        lines
            .split(|line| line.is_empty())
            .filter_map(|chunk| {
                let (a_x, a_y) = parse_button(chunk[0]);
                let (b_x, b_y) = parse_button(chunk[1]);
                let (p_x, p_y) = parse_prize(chunk[2]);

                solve((a_x, a_y), (b_x, b_y), (p_x, p_y))
            })
            .sum::<isize>()
    }

    fn solve_part_two(&self, lines: &[&str]) -> Answer {
        lines
            .split(|line| line.is_empty())
            .filter_map(|chunk| {
                let (a_x, a_y) = parse_button(chunk[0]);
                let (b_x, b_y) = parse_button(chunk[1]);
                let (p_x, p_y) = parse_prize(chunk[2]);
                let offset = 10000000000000;

                solve((a_x, a_y), (b_x, b_y), (p_x + offset, p_y + offset))
            })
            .sum::<isize>()
    }
}

fn main() {
    let sample = include_str!("../../samples/13.txt");
    let input = include_str!("../../inputs/13.txt");
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
