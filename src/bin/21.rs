use core::panic;
use std::collections::{HashMap, HashSet, VecDeque};

use aoc::{Cardinal, Graph, Solver};
use itertools::{Diff, Itertools};
use log::debug;

type Answer = usize;

const PART_ONE_SAMPLE_ANSWER: Answer = 126384;
const PART_TWO_SAMPLE_ANSWER: Answer = 0;

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

    pub fn to_char(&self) -> char {
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
fn find_shortest_path(graph: &Graph<Button, Cardinal>, start: Button, end: Button) -> Vec<Button> {
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();
    queue.push_back((start, Vec::new()));

    while let Some((vertex, path)) = queue.pop_front() {
        // debug!("Visiting vertex {:?} with path {:?})", vertex, path);
        if vertex == end {
            return path;
        }

        if let Some(edges) = graph.edges.get(&vertex) {
            for (neighbor, direction) in edges {
                if visited.contains(&neighbor) {
                    continue;
                }
                visited.insert(neighbor);
                let mut new_path = path.clone();
                new_path.push(Button::Direction(*direction));
                queue.push_back((*neighbor, new_path));
            }
        }
    }
    panic!("No path found");
}

fn find_all_paths(graph: &Graph<Button, Cardinal>, start: Button, end: Button) -> Vec<Vec<Button>> {
    let mut queue = VecDeque::new();
    queue.push_back((start, Vec::<(Button, Cardinal)>::new()));
    let mut all_paths = Vec::new();
    let mut lowest = usize::MAX;

    while let Some((vertex, path)) = queue.pop_front() {
        if vertex == end && path.len() < lowest {
            lowest = path.len();
            all_paths.push(path.iter().map(|b| Button::Direction(b.1)).collect_vec());
            continue;
        }

        if let Some(edges) = graph.edges.get(&vertex) {
            for (neighbor, direction) in edges {
                if path.iter().any(|(b, _)| b == neighbor) || *neighbor == start {
                    continue;
                }
                let mut new_path = path.clone();
                new_path.push((*neighbor, *direction));
                queue.push_back((*neighbor, new_path));
            }
        }
    }
    all_paths
}

fn build_path_map(
    graph_a: &Graph<Button, Cardinal>,
    up_dimension_shortest_path: &dyn Fn(Button, Button) -> Vec<Button>,
) -> HashMap<(Button, Button), Vec<Button>> {
    graph_a
        .all_vertices()
        .cartesian_product(graph_a.edges.keys())
        .fold(HashMap::new(), |mut acc, (start, end)| {
            let all_paths = find_all_paths(graph_a, *start, *end);

            let (best_path, best_commands) = all_paths
                .into_iter()
                .map(|target_path| {
                    let shortest_path =
                        target_path
                            .iter()
                            .tuple_windows()
                            .fold(Vec::new(), |mut acc, (a, b)| {
                                // let mut directional_path = find_shortest_path(graph_b, *a, *b);
                                let mut directional_path = up_dimension_shortest_path(*a, *b);
                                acc.append(&mut directional_path);
                                acc.push(Button::Activate);
                                acc
                            });
                    (target_path, shortest_path)
                })
                .min_by_key(|(_, shortest)| shortest.len())
                .unwrap();

            debug!("Best path from {:?} to {:?}: {:?}", start, end, best_path);
            debug!(
                "Best commands from {:?} to {:?}: {:?}",
                start, end, best_commands
            );

            *acc.entry((*start, *end)).or_default() = best_path;

            acc
        })
}

struct Solution {}
impl Solver<'_, Answer> for Solution {
    fn solve_part_one(&self, lines: &[&str]) -> Answer {
        let numeric_keypad = build_numeric_keypad();
        let directional_keypad = build_directional_keypad();

        let direction_map = build_path_map(&directional_keypad, &|a, b| {
            find_shortest_path(&directional_keypad, a, b)
        });
        let number_map = build_path_map(&numeric_keypad, &|a, b| {
            let best_path = &direction_map[&(a, b)];
            best_path.clone()
        });

        lines
            .iter()
            .map(|s| get_complexity(&direction_map, &number_map, s, 2))
            .sum()
    }

    fn solve_part_two(&self, lines: &[&str]) -> Answer {
        let numeric_keypad = build_numeric_keypad();
        let directional_keypad = build_directional_keypad();

        let direction_map = build_path_map(&directional_keypad, &|a, b| {
            find_shortest_path(&directional_keypad, a, b)
        });
        let number_map = build_path_map(&numeric_keypad, &|a, b| {
            let best_path = &direction_map[&(a, b)];
            best_path.clone()
        });
        lines
            .iter()
            .map(|s| get_complexity(&direction_map, &number_map, s, 25))
            .sum()
    }
}

fn get_complexity(
    direction_map: &HashMap<(Button, Button), Vec<Button>>,
    number_map: &HashMap<(Button, Button), Vec<Button>>,
    s: &&str,
    levels: usize,
) -> usize {
    let numeric = s[0..3].parse::<usize>().unwrap();

    let mut buttons = s.chars().map(Button::from_char).collect_vec();
    buttons.insert(0, Button::Activate);

    let shortest_path = buttons
        .iter()
        .tuple_windows()
        .fold(Vec::new(), |mut acc, (a, b)| {
            let numeric_path = &number_map[&(*a, *b)];
            acc.append(&mut numeric_path.clone());
            acc.push(Button::Activate);
            acc
        });
    debug!(
        "{s} Shortest path: {}",
        shortest_path.iter().map(|c| c.to_char()).join("")
    );

    let mut target_path = shortest_path.clone();

    for level in 0..levels {
        target_path.insert(0, Button::Activate);
        let shortest_path =
            target_path
                .iter()
                .tuple_windows()
                .fold(Vec::new(), |mut acc, (a, b)| {
                    let numeric_path = &direction_map[&(*a, *b)];
                    acc.append(&mut numeric_path.clone());
                    acc.push(Button::Activate);
                    acc
                });
        debug!(
            "Level {level} with length {len} Shortest path: {path}",
            len = shortest_path.len(),
            path = shortest_path.iter().map(|c| c.to_char()).join("")
        );
        target_path = shortest_path;
    }

    debug!(
        "Code {s} with numeric {numeric} has path of length {}",
        target_path.len()
    );

    numeric * target_path.len()
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
