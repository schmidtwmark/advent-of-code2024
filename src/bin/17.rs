use aoc::Solver;
use core::panic;
use itertools::Itertools;
use log::{debug, info, trace};
use regex::Regex;
use std::{
    collections::{HashMap, VecDeque},
    ops::BitXor,
    process::Output,
};

type Answer = String;

#[derive(Debug, Clone)]
struct Input {
    registers: HashMap<char, isize>,
    program: Vec<isize>,
    instruction_ptr: usize,
    output: Vec<isize>,
}

fn operator_to_string(op: isize) -> &'static str {
    match op {
        0 => "adv",
        1 => "bxl",
        2 => "bst",
        3 => "jnz",
        4 => "bxc",
        5 => "out",
        6 => "bdv",
        7 => "cdv",
        _ => panic!(),
    }
}

impl Input {
    fn from_lines(lines: &[&str]) -> Self {
        let (registers, program) = lines.split(|s| s.is_empty()).collect_tuple().unwrap();

        let registers = registers.iter().fold(HashMap::new(), |mut map, line| {
            let regex = Regex::new(r"Register ([A-Z]+): ([0-9]+)").unwrap();
            let captures = regex.captures(line).unwrap();
            let name = captures.get(1).unwrap().as_str().chars().next().unwrap();
            let value = captures.get(2).unwrap().as_str().parse().unwrap();
            map.insert(name, value);

            map
        });

        let (_, program) = program[0].split_once(": ").unwrap();

        let program = program
            .split(",")
            .map(|line| line.parse().unwrap())
            .collect();

        Input {
            registers,
            program,
            instruction_ptr: 0,
            output: Vec::new(),
        }
    }

    // output: true if should continue, false if should stop
    // output: None if no output, isize if output
    fn process(&mut self) -> (bool, Option<isize>) {
        if self.instruction_ptr >= self.program.len() {
            return (false, None);
        }

        let operator = self.program[self.instruction_ptr];
        let operand = self.program[self.instruction_ptr + 1];
        let a = 'A';
        let b = 'B';
        let c = 'C';

        let combo_operand = match operand {
            0..4 => operand,
            4 => *self.registers.get(&a).unwrap(),
            5 => *self.registers.get(&b).unwrap(),
            6 => *self.registers.get(&c).unwrap(),
            7 => panic!("Reserved operand, invalid"),
            _ => panic!("Unknown operand"),
        };

        let mut divide = |out_register: &char| {
            let numerator = self.registers.get(&a).unwrap();
            let denominator = 2isize.pow(combo_operand as u32);
            *self.registers.entry(*out_register).or_default() = numerator / denominator;
        };
        let mut out = None;

        match operator {
            0 => {
                // adv
                divide(&a);
            }
            1 => {
                // bxl
                let b_val = self.registers.get(&b).unwrap();
                *self.registers.entry(b).or_default() = b_val.bitxor(operand);
            }
            2 => {
                // bst
                *self.registers.entry(b).or_default() = combo_operand % 8;
            }
            3 => {
                // jnz
                if self.registers.get(&a).unwrap() != &0 {
                    self.instruction_ptr = operand as usize;
                } else {
                    self.instruction_ptr += 2;
                }
            }
            4 => {
                // bxc
                let b_val = self.registers.get(&b).unwrap();
                let c_val = self.registers.get(&c).unwrap();
                *self.registers.entry(b).or_default() = b_val.bitxor(c_val);
            }
            5 => {
                // out
                out = Some(combo_operand % 8);
                self.output.push(combo_operand % 8);
            }
            6 => {
                // bdv
                divide(&b);
            }
            7 => {
                // cdv
                divide(&c);
            }
            _ => panic!("Unknown operator"),
        }

        match operator {
            0..3 => self.instruction_ptr += 2,
            3 => {}
            4..8 => self.instruction_ptr += 2,
            _ => {}
        }
        trace!(
            "After running instruction: {} {} with combo_operand {} , registers: {:?}, output: {:?}",
            operator_to_string(operator),
            operand,
            combo_operand,
            self.registers,
            self.output
        );

        (true, out)
    }
}

struct Solution {}
impl Solver<'_, Answer> for Solution {
    fn solve_part_one(&self, lines: &[&str]) -> Answer {
        let mut input = Input::from_lines(lines);

        while input.process().0 {}
        input.output.iter().join(",")
    }

    fn solve_part_two(&self, lines: &[&str]) -> Answer {
        let mut input = Input::from_lines(lines);

        // let mut a_val = 0;
        // let mut current_target_output = Vec::new();

        let mut queue = VecDeque::new();
        queue.push_back(0);

        let mut i = 0;
        while !queue.is_empty() {
            let expected_output = input
                .program
                .iter()
                .skip(input.program.len() - i - 1)
                .copied()
                .collect_vec();
            if i == input.program.len() {
                return queue.iter().min().unwrap().to_string();
            }

            for _ in 0..queue.len() {
                let value = queue.pop_front().unwrap();
                for k in 0..8 {
                    let a = 8 * value + k;
                    let mut input_clone = input.clone();
                    input_clone.registers.insert('A', a);
                    while input_clone.process().0 {}
                    if input_clone.output == expected_output {
                        queue.push_back(a);
                    }
                }
            }

            i += 1;
        }

        "".to_string()
    }
}

fn main() {
    let part_one_sample_answer: Answer = "4,6,3,5,6,3,5,2,1,0".to_owned();

    let sample = include_str!("../../samples/17.txt");
    let input = include_str!("../../inputs/17.txt");
    let part_one_problems = [
        aoc::Input::new_sample(sample, part_one_sample_answer),
        aoc::Input::new_final(input),
    ];

    let part_two_problems = [aoc::Input::new_final(input)];

    Solution {}.run(&part_one_problems, &part_two_problems);
}
