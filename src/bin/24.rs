use aoc::Solver;
use itertools::Itertools;
use log::debug;
use regex::Regex;
use std::{
    borrow::Borrow,
    collections::{HashMap, HashSet, VecDeque},
    fs::File,
    io::{Result, Write},
};

type Answer = String;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum Operator {
    And,
    Or,
    Xor,
}

impl Operator {
    fn from_str(s: &str) -> Option<Self> {
        match s {
            "AND" => Some(Operator::And),
            "OR" => Some(Operator::Or),
            "XOR" => Some(Operator::Xor),
            _ => None,
        }
    }

    fn apply(self, a: bool, b: bool) -> bool {
        match self {
            Operator::And => a && b,
            Operator::Or => a || b,
            Operator::Xor => a ^ b,
        }
    }
}

fn read_value(values: &HashMap<&str, bool>, name: &str) -> usize {
    let mut i = 0;
    let mut out = 0;
    while let Some(z) = values.get(&format!("{name}{:02}", i).borrow()) {
        let v = if *z { 1 } else { 0 };
        out |= v << i;
        i += 1;
    }

    out
}

fn generate_graphviz(operations: &Operations, file_path: &str) -> std::io::Result<()> {
    let mut file = File::create(file_path)?;

    writeln!(file, "digraph Operations {{")?;
    writeln!(file, "  rankdir=LR;")?; // Layout graph left-to-right
    writeln!(file, "  node [shape=circle];")?;

    for (key, ops) in operations {
        for (_, operator, result) in ops {
            writeln!(
                file,
                "  \"{}\" -> \"{}\" [label=\"{:?}\"];",
                key, result, operator
            )?;
            // writeln!(
            //     file,
            //     "  \"{}\" -> \"{}\" [label=\"{:?}\"];",
            //     operand, result, operator
            // )?;
        }
    }

    writeln!(file, "}}")?;
    Ok(())
}

type Values<'a> = HashMap<&'a str, bool>;
type Operations<'a> = HashMap<&'a str, Vec<(&'a str, Operator, &'a str)>>;

fn parse_input<'a>(lines: &'a [&str]) -> (Values<'a>, Operations<'a>) {
    let (input_values, operations) = lines.split(|line| line.is_empty()).collect_tuple().unwrap();

    let values = input_values.iter().fold(HashMap::new(), |mut map, line| {
        let (name, value) = line.split_once(": ").unwrap();
        debug!("'{}': '{}'", name, value);
        map.insert(name, value == "1");
        map
    });

    debug!("{:?}", values);

    let operations: HashMap<&str, Vec<(&str, Operator, &str)>> =
        operations.iter().fold(HashMap::new(), |mut map, line| {
            let regex = Regex::new(r"([^ ]+) ([A-Z]+) ([^ ]+) -> ([^ ]+)").unwrap();

            let captures = regex.captures(line).unwrap();
            let a = captures.get(1).unwrap().as_str();
            let op = captures.get(2).unwrap().as_str();
            let b = captures.get(3).unwrap().as_str();
            let out = captures.get(4).unwrap().as_str();

            debug!("{:?}", (a, op, b, out));

            let op = Operator::from_str(op).unwrap();

            map.entry(a).or_default().push((b, op, out));
            map.entry(b).or_default().push((a, op, out));

            map
        });

    debug!("{:?}", operations);

    (values, operations)
}

fn simulate<'a>(values: &'a Values<'a>, operations: &'a Operations) -> Values<'a> {
    let mut values = values.clone();
    loop {
        debug!("Values {:?}", values);
        let mut new_values = HashMap::new();
        for (a, a_val) in values.iter() {
            if let Some(ops) = operations.get(a) {
                for (b, op, out) in ops.iter() {
                    if values.contains_key(out) {
                        continue;
                    }
                    if let Some(b_val) = values.get(b) {
                        new_values.insert(*out, op.apply(*a_val, *b_val));
                    }
                }
            }
        }

        debug!("New values {:?}", new_values);
        if new_values.is_empty() {
            break;
        } else {
            values.extend(new_values.drain());
        }
    }

    values
}

struct Solution {}
impl Solver<'_, Answer> for Solution {
    fn solve_part_one(&self, lines: &[&str]) -> Answer {
        let (values, operations) = parse_input(lines);

        let values = simulate(&values, &operations);
        read_value(&values, "z").to_string()
    }

    fn solve_part_two(&self, lines: &[&str]) -> Answer {
        let (start_values, operations) = parse_input(lines);
        // let values = simulate(&start_values, &operations);

        let x = read_value(&start_values, "x");
        let y = read_value(&start_values, "y");
        let target_z = x + y;
        if let Err(e) = generate_graphviz(&operations, "operations_graph.dot") {
            eprintln!("Failed to generate Graphviz file: {}", e);
        } else {
            println!("Graphviz file generated: operations_graph.dot");
        }

        // TODO: visualize the operations

        // let all_zs: HashSet<_> = operations
        //     .iter()
        //     .filter_map(|(k, v)| {
        //         for (b, op, out) in v.iter() {
        //             if out.contains("z") {
        //                 return Some(out);
        //             }
        //         }
        //         None
        //     })
        //     .collect();
        // let all_zs = all_zs.into_iter().sorted().collect_vec();
        // debug!("{:?}", all_zs);
        // let z_dependencies: HashMap<_, _> = all_zs
        //     .iter()
        //     .copied()
        //     .map(|z| {
        //         // For each z, go up the chain of dependencies
        //         let mut dependencies = HashSet::new();
        //         let mut queue = VecDeque::new();
        //         queue.push_back(z);
        //         while let Some(node) = queue.pop_front() {
        //             // Find ops that depend on this node
        //             operations.iter().for_each(|(k, v)| {
        //                 for (b, op, out) in v.iter() {
        //                     if out == node {
        //                         queue.push_back(k);
        //                         dependencies.insert(k);
        //                     }
        //                 }
        //             });
        //         }
        //         (z, dependencies)
        //     })
        //     .collect();

        // debug!("Z dependencies: {:?}", z_dependencies);

        // let incorrect_zs = all_zs
        //     .iter()
        //     .enumerate()
        //     .filter(|(i, z)| (target_z & (1 << i) == 1) == *values.get(**z).unwrap())
        //     .collect_vec();

        // debug!("Incorrect zs: {:?}", incorrect_zs);

        Answer::default()
    }
}

fn main() {
    let sample = include_str!("../../samples/24.txt");
    let sample_1 = include_str!("../../samples/24_1.txt");
    let sample_2 = include_str!("../../samples/24_2.txt");
    let input = include_str!("../../inputs/24.txt");
    let part_one_problems = [
        aoc::Input::new_sample(sample, "4".to_owned()),
        aoc::Input::new_sample(sample_1, "2024".to_owned()),
        aoc::Input::new_final(input),
    ];

    let part_two_problems = [
        aoc::Input::new_sample(sample_2, "z00,z01,z02,z05".to_owned()),
        aoc::Input::new_final(input),
    ];

    Solution {}.run(&part_one_problems, &part_two_problems);
}
