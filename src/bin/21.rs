use cached::proc_macro::cached;
use core::panic;
use std::{
    collections::{HashMap, HashSet, VecDeque},
    sync::OnceLock,
};

use aoc::{Cardinal, Graph, Solver};
use itertools::Itertools;
use log::debug;

type Answer = usize;

const PART_ONE_SAMPLE_ANSWER: Answer = 126384;
const PART_TWO_SAMPLE_ANSWER: Answer = 154115708116294;

#[derive(Debug, Clone, Eq, PartialEq, Copy, Hash)]
enum Button {
    Activate,
    Direction(Cardinal),
    Number(usize),
}

impl Button {
    pub fn from_char(c: char) -> Button {
        match c {
            '0'..='9' => Button::Number(c.to_digit(10).unwrap() as usize),
            'A' => Button::Activate,
            _ => Button::Direction(Cardinal::from_char(c)),
        }
    }

    pub fn _to_char(&self) -> char {
        match self {
            Button::Activate => 'A',
            Button::Direction(c) => c.to_char(),
            Button::Number(n) => n.to_string().chars().next().unwrap(),
        }
    }
}

fn bidirectional(graph: &mut Graph<Button, Cardinal>, from: char, to: char, cardinal: Cardinal) {
    let from = Button::from_char(from);
    let to = Button::from_char(to);
    graph.add_edge(from, to, cardinal);
    graph.add_edge(to, from, cardinal.opposite());
}

fn build_numeric_keypad() -> Graph<Button, Cardinal> {
    let mut numeric_keypad = Graph::new();
    bidirectional(&mut numeric_keypad, '7', '8', Cardinal::East);
    bidirectional(&mut numeric_keypad, '7', '4', Cardinal::South);
    bidirectional(&mut numeric_keypad, '8', '9', Cardinal::East);
    bidirectional(&mut numeric_keypad, '8', '5', Cardinal::South);
    bidirectional(&mut numeric_keypad, '9', '6', Cardinal::South);

    bidirectional(&mut numeric_keypad, '4', '5', Cardinal::East);
    bidirectional(&mut numeric_keypad, '4', '1', Cardinal::South);
    bidirectional(&mut numeric_keypad, '5', '6', Cardinal::East);
    bidirectional(&mut numeric_keypad, '5', '2', Cardinal::South);
    bidirectional(&mut numeric_keypad, '6', '3', Cardinal::South);

    bidirectional(&mut numeric_keypad, '1', '2', Cardinal::East);
    bidirectional(&mut numeric_keypad, '2', '3', Cardinal::East);
    bidirectional(&mut numeric_keypad, '2', '0', Cardinal::South);
    bidirectional(&mut numeric_keypad, '3', 'A', Cardinal::South);

    bidirectional(&mut numeric_keypad, '0', 'A', Cardinal::East);
    numeric_keypad.debug_connections();
    numeric_keypad
}

fn build_directional_keypad() -> Graph<Button, Cardinal> {
    let mut directional_keypad = Graph::new();
    bidirectional(&mut directional_keypad, '^', 'A', Cardinal::East);
    bidirectional(&mut directional_keypad, '^', 'v', Cardinal::South);
    bidirectional(&mut directional_keypad, 'A', '>', Cardinal::South);

    bidirectional(&mut directional_keypad, '<', 'v', Cardinal::East);
    bidirectional(&mut directional_keypad, 'v', '>', Cardinal::East);

    directional_keypad.debug_connections();
    directional_keypad
}

fn build_path_map(graph: &Graph<Button, Cardinal>) -> HashMap<(Button, Button), Vec<Vec<Button>>> {
    graph
        .all_vertices()
        .cartesian_product(graph.edges.keys())
        .fold(HashMap::new(), |mut acc, (start, end)| {
            let mut queue = VecDeque::new();
            queue.push_back((start, Vec::new(), HashSet::new()));

            let mut paths = Vec::new();
            let mut lowest = usize::MAX;

            while let Some((node, path, mut visited)) = queue.pop_front() {
                if node == end {
                    if path.len() <= lowest {
                        lowest = path.len();
                        paths.push(path);
                    }
                    continue;
                }

                if visited.contains(&node) {
                    continue;
                }
                visited.insert(node);

                for (neighbor, direction) in graph.edges.get(node).unwrap() {
                    let mut path = path.clone();
                    path.push(Button::Direction(*direction));
                    queue.push_back((neighbor, path, visited.clone()));
                }
            }

            acc.insert((*start, *end), paths);

            acc
        })
}

type PathMap = HashMap<(Button, Button), Vec<Vec<Button>>>;

fn numeric_map() -> &'static PathMap {
    static NUMERIC_PATHS: OnceLock<PathMap> = OnceLock::new();
    NUMERIC_PATHS.get_or_init(|| {
        let numeric_keypad = build_numeric_keypad();
        build_path_map(&numeric_keypad)
    })
}

fn direction_map() -> &'static PathMap {
    static DIRECTION_PATHS: OnceLock<PathMap> = OnceLock::new();
    DIRECTION_PATHS.get_or_init(|| {
        let direction_keypad = build_directional_keypad();
        build_path_map(&direction_keypad)
    })
}

struct Solution {}
impl Solver<'_, Answer> for Solution {
    fn solve_part_one(&self, lines: &[&str]) -> Answer {
        lines.iter().map(|s| get_complexity(s, 2)).sum()
    }

    fn solve_part_two(&self, lines: &[&str]) -> Answer {
        lines.iter().map(|s| get_complexity(s, 25)).sum()
    }
}

#[cached]
fn get_shortest(sequence: Vec<Button>, depth: usize, number_keypad: bool) -> usize {
    let map = if number_keypad {
        numeric_map()
    } else {
        direction_map()
    };

    [Button::Activate]
        .iter()
        .chain(sequence.iter())
        .tuple_windows()
        .map(|(a, b)| {
            let shortest_paths = map.get(&(*a, *b)).unwrap();

            if depth == 0 {
                shortest_paths[0].len() + 1
            } else {
                shortest_paths
                    .iter()
                    .cloned()
                    .map(|mut path| {
                        path.push(Button::Activate);
                        get_shortest(path, depth - 1, false)
                    })
                    .min()
                    .unwrap()
            }
        })
        .sum::<usize>()
}

fn get_complexity(s: &&str, levels: usize) -> usize {
    let numeric = s[0..3].parse::<usize>().unwrap();

    let buttons = s.chars().map(Button::from_char).collect_vec();

    debug!(
        "Numeric: {}, Buttons: {:?}, Levels: {}",
        numeric, buttons, levels
    );

    numeric * get_shortest(buttons, levels, true)
}

fn main() {
    let sample = include_str!("../../samples/21.txt");
    let input = include_str!("../../inputs/21.txt");
    let part_one_problems = [
        aoc::Input::new_sample(sample, PART_ONE_SAMPLE_ANSWER),
        aoc::Input::new_final(input), // 138560 too high
    ];

    let part_two_problems = [
        aoc::Input::new_sample(sample, PART_TWO_SAMPLE_ANSWER),
        aoc::Input::new_final(input),
    ];

    Solution {}.run(&part_one_problems, &part_two_problems);
}
