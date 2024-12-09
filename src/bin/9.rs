use std::{fmt::Display, iter};

use aoc::Solver;
use itertools::Itertools;
use log::debug;

type Answer = usize;

const PART_ONE_SAMPLE_ANSWER: Answer = 1928;
const PART_TWO_SAMPLE_ANSWER: Answer = 2858;

#[derive(Debug, Clone, Eq, PartialEq)]
enum NodeType {
    Empty,
    Filled { id: usize },
}
#[derive(Debug, Clone, Eq, PartialEq)]
struct Node {
    node_type: NodeType,
    size: u32,
}

impl Node {
    fn expand(&self) -> impl Iterator<Item = usize> {
        match self.node_type {
            NodeType::Empty => iter::repeat(0).take(self.size as usize),
            NodeType::Filled { id } => iter::repeat(id).take(self.size as usize),
        }
    }
}

impl Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.node_type {
            NodeType::Empty => write!(f, "{}", ".".repeat(self.size as usize)),
            NodeType::Filled { id } => write!(f, "{}", id.to_string().repeat(self.size as usize)),
        }
    }
}

// Use display to concat each node to a single string and return
fn nodes_to_string(nodes: &[Node]) -> String {
    nodes.iter().map(|n| n.to_string()).join("")
}

fn swap(front_index: usize, back_index: usize, nodes: &mut Vec<Node>) {
    match nodes[front_index].size.cmp(&nodes[back_index].size) {
        std::cmp::Ordering::Less => {
            nodes[front_index].node_type = nodes[back_index].node_type.clone();
            nodes[back_index].size -= nodes[front_index].size;
        }
        std::cmp::Ordering::Equal => {
            nodes[front_index] = nodes[back_index].clone();
            nodes[back_index].node_type = NodeType::Empty;
        }
        std::cmp::Ordering::Greater => {
            let copy = nodes[back_index].clone();
            nodes[back_index].node_type = NodeType::Empty;
            nodes[front_index].size -= nodes[back_index].size;
            nodes.insert(front_index, copy);
        }
    }
}

fn compress(nodes: &mut Vec<Node>) {
    let mut front_index = 0;
    let mut back_index = nodes.len() - 1;

    while front_index < back_index {
        while front_index < back_index
            && matches!(nodes[front_index].node_type, NodeType::Filled { id: _ })
        {
            front_index += 1;
        }

        while back_index > 0
            && back_index > front_index
            && nodes[back_index].node_type == NodeType::Empty
        {
            back_index -= 1;
        }

        if front_index < back_index {
            swap(front_index, back_index, nodes);
        }
    }
}

fn compress2(nodes: &mut Vec<Node>) {
    let mut back_index = nodes.len() - 1;
    while back_index > 0 {
        if nodes[back_index].node_type == NodeType::Empty {
            back_index -= 1;
            continue;
        }
        for front_index in 0..back_index {
            if nodes[front_index].node_type == NodeType::Empty {
                let mut end_index = front_index + 1;
                let mut cumulative_size = nodes[front_index].size;
                while nodes[end_index].node_type == NodeType::Empty && end_index < back_index {
                    cumulative_size += nodes[end_index].size;
                    end_index += 1;
                }
                if cumulative_size >= nodes[back_index].size {
                    let remainder = cumulative_size - nodes[back_index].size;
                    let copy = nodes[back_index].clone();
                    nodes[back_index].node_type = NodeType::Empty;
                    for i in (front_index..end_index).rev() {
                        nodes.remove(i);
                    }
                    nodes.insert(front_index, copy);
                    nodes.insert(
                        front_index + 1,
                        Node {
                            node_type: NodeType::Empty,
                            size: remainder,
                        },
                    );
                    back_index -= end_index - front_index - 1;
                    break;
                }
            }
        }
        back_index -= 1;
    }
}

struct Solution {}
impl Solver<'_, usize> for Solution {
    fn solve_part_one(&self, lines: &[&str]) -> Answer {
        let line = lines.first().unwrap();
        let mut id = 0;
        let mut nodes = line
            .chars()
            .map(|c| c.to_digit(10).unwrap())
            .chunks(2)
            .into_iter()
            .fold(Vec::new(), |mut nodes, mut chunk| {
                let file_size = chunk.nth(0).unwrap();
                nodes.push(Node {
                    size: file_size,
                    node_type: NodeType::Filled { id },
                });
                id += 1;

                if let Some(empty_size) = chunk.nth(0) {
                    nodes.push(Node {
                        size: empty_size,
                        node_type: NodeType::Empty,
                    });
                }

                nodes
            });
        compress(&mut nodes);

        let mut position = 0;

        nodes
            .iter()
            .map(|node| {
                node.expand()
                    .map(|id| {
                        let out = id * position;
                        position += 1;
                        out
                    })
                    .sum::<usize>()
            })
            .sum()
    }

    fn solve_part_two(&self, lines: &[&str]) -> Answer {
        let line = lines.first().unwrap();
        let mut id = 0;
        let mut nodes = line
            .chars()
            .map(|c| c.to_digit(10).unwrap())
            .chunks(2)
            .into_iter()
            .fold(Vec::new(), |mut nodes, mut chunk| {
                let file_size = chunk.nth(0).unwrap();
                nodes.push(Node {
                    size: file_size,
                    node_type: NodeType::Filled { id },
                });
                id += 1;

                if let Some(empty_size) = chunk.nth(0) {
                    nodes.push(Node {
                        size: empty_size,
                        node_type: NodeType::Empty,
                    });
                }

                nodes
            });
        debug!("start:\t{:?}", nodes);
        compress2(&mut nodes);
        for node in nodes.iter() {
            debug!("\t{:?}", node);
        }

        let mut position = 0;

        nodes
            .iter()
            .map(|node| {
                node.expand()
                    .map(|id| {
                        let out = id * position;
                        position += 1;
                        out
                    })
                    .sum::<usize>()
            })
            .sum()
    }
}

fn main() {
    let sample = include_str!("../../samples/9.txt");
    let input = include_str!("../../inputs/9.txt");
    let part_one_problems = [
        aoc::Input::new_sample(sample, PART_ONE_SAMPLE_ANSWER),
        aoc::Input::new_final(input),
    ];

    // 8415835004926 is too high
    // 7054796111080 is too high
    // 7360113614470 is not right
    let part_two_problems = [
        aoc::Input::new_sample(sample, PART_TWO_SAMPLE_ANSWER),
        aoc::Input::new_final(input),
    ];

    Solution {}.run(&part_one_problems, &part_two_problems);
}
