use std::{
    collections::{HashMap, VecDeque},
    fmt::Display,
};

use aoc::{Cardinal, Grid, Solver};
use hashbrown::HashSet;
use itertools::Itertools;
use log::debug;

type Answer = usize;

const PART_ONE_SAMPLE_ANSWER: Answer = 7036;
const PART_TWO_SAMPLE_ANSWER: Answer = 45;

#[derive(Debug, Clone, Eq, PartialEq, Default, Copy)]
enum GridObject {
    #[default]
    Wall,
    Empty,
    Position(Cardinal),
    End,
}

impl GridObject {
    fn from_char(c: char) -> Self {
        match c {
            '#' => GridObject::Wall,
            '.' => GridObject::Empty,
            'S' => GridObject::Position(Cardinal::East),
            'E' => GridObject::End,
            _ => panic!("Unknown character "),
        }
    }
}

impl Display for GridObject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GridObject::Wall => write!(f, "#"),
            GridObject::Empty => write!(f, "."),
            GridObject::Position(c) => write!(f, "{}", c.to_char()),
            GridObject::End => write!(f, "E"),
        }
    }
}

struct Solution {}
impl Solver<'_, Answer> for Solution {
    fn solve_part_one(&self, lines: &[&str]) -> Answer {
        let grid = Grid::from_lines(lines, &GridObject::from_char);

        let start_position = grid
            .positions()
            .find(|pos| matches!(grid.at(*pos), GridObject::Position(Cardinal::East)))
            .unwrap();

        let end_position = grid
            .positions()
            .find(|pos| matches!(grid.at(*pos), GridObject::End))
            .unwrap();

        let mut min_cost = usize::MAX;
        let mut visited_cost = HashMap::new();
        let mut queue = VecDeque::new();
        queue.push_back((start_position, Cardinal::East, 0));
        visited_cost.insert((start_position, Cardinal::East), 0);
        // visited.insert(start_position);

        while let Some((vertex, direction, cost)) = queue.pop_front() {
            // let mut temp = grid.clone();
            // *temp.mut_at(start_position) = GridObject::Empty;
            // *temp.mut_at(vertex) = GridObject::Position(direction);
            // debug!(
            //     "Visiting {:?} with direction {:?} with cost {} and grid: {}",
            //     vertex, direction, cost, temp
            // );
            if grid.at(vertex) == &GridObject::Wall {
                panic!("Wall");
            }
            if vertex == end_position && min_cost > cost {
                debug!("Found an end with cost: {}", cost);
                min_cost = cost
            } else {
                // Check forward
                let mut maybe_add =
                    |new_position: (usize, usize), new_direction: Cardinal, delta_cost: usize| {
                        if matches!(grid.at(new_position), &GridObject::Empty | &GridObject::End)
                            && (!visited_cost.contains_key(&(new_position, new_direction))
                                || visited_cost.get(&(new_position, new_direction))
                                    > Some(&(cost + delta_cost)))
                        // && !visited.contains(&new_position)
                        // && !visited.contains(&(new_position, new_direction))
                        {
                            // visited.insert((new_position, new_direction));
                            // visited.insert(new_position);
                            visited_cost.insert((new_position, new_direction), cost + delta_cost);
                            queue.push_back((new_position, new_direction, cost + delta_cost));
                            return Some(new_direction);
                        }
                        None
                    };
                let mut added_neighbors = Vec::new();
                if let Some(new_position) = grid.get_neighbor_position(vertex, direction) {
                    added_neighbors.push(maybe_add(new_position, direction, 1));
                }
                // check clockwise / counterclockwise
                let clockwise = direction.clockwise();
                if let Some(new_position) = grid.get_neighbor_position(vertex, clockwise) {
                    added_neighbors.push(maybe_add(new_position, clockwise, 1001));
                }
                let counter_clockwise = direction.counter_clockwise();
                if let Some(new_position) = grid.get_neighbor_position(vertex, counter_clockwise) {
                    added_neighbors.push(maybe_add(new_position, counter_clockwise, 1001));
                }
                debug!(
                    "Added neighbors: {:?}",
                    added_neighbors.into_iter().flatten().collect_vec()
                );
            }
        }

        min_cost
    }

    fn solve_part_two(&self, lines: &[&str]) -> Answer {
        let grid = Grid::from_lines(lines, &GridObject::from_char);

        let start_position = grid
            .positions()
            .find(|pos| matches!(grid.at(*pos), GridObject::Position(Cardinal::East)))
            .unwrap();

        let end_position = grid
            .positions()
            .find(|pos| matches!(grid.at(*pos), GridObject::End))
            .unwrap();

        let mut min_cost = usize::MAX;
        let mut best_paths = Vec::new();
        let mut visited_cost = HashMap::new();
        let mut queue = VecDeque::new();
        queue.push_back((vec![start_position], Cardinal::East, 0));
        visited_cost.insert((start_position, Cardinal::East), 0);
        // visited.insert(start_position);

        while let Some((path, direction, cost)) = queue.pop_front() {
            // let mut temp = grid.clone();
            // *temp.mut_at(start_position) = GridObject::Empty;
            // *temp.mut_at(vertex) = GridObject::Position(direction);
            // debug!(
            //     "Visiting {:?} with direction {:?} with cost {} and grid: {}",
            //     vertex, direction, cost, temp
            // );
            let vertex = *path.last().unwrap();
            if grid.at(vertex) == &GridObject::Wall {
                panic!("Wall");
            }
            if vertex == end_position {
                debug!("Found an end with cost: {}", cost);
                match cost.cmp(&min_cost) {
                    std::cmp::Ordering::Less => {
                        min_cost = cost;
                        best_paths = vec![path.clone()];
                    }
                    std::cmp::Ordering::Equal => {
                        best_paths.push(path.clone());
                    }
                    std::cmp::Ordering::Greater => {}
                }
            } else {
                // Check forward
                let mut maybe_add =
                    |new_position: (usize, usize), new_direction: Cardinal, delta_cost: usize| {
                        if matches!(grid.at(new_position), &GridObject::Empty | &GridObject::End)
                            && (!visited_cost.contains_key(&(new_position, new_direction))
                                || visited_cost.get(&(new_position, new_direction))
                                    >= Some(&(cost + delta_cost)))
                        // && !visited.contains(&new_position)
                        // && !visited.contains(&(new_position, new_direction))
                        {
                            // visited.insert((new_position, new_direction));
                            // visited.insert(new_position);
                            visited_cost.insert((new_position, new_direction), cost + delta_cost);
                            let mut new_path = path.clone();
                            new_path.push(new_position);
                            queue.push_back((new_path, new_direction, cost + delta_cost));
                            return Some(new_direction);
                        }
                        None
                    };
                let mut added_neighbors = Vec::new();
                if let Some(new_position) = grid.get_neighbor_position(vertex, direction) {
                    added_neighbors.push(maybe_add(new_position, direction, 1));
                }
                // check clockwise / counterclockwise
                let clockwise = direction.clockwise();
                if let Some(new_position) = grid.get_neighbor_position(vertex, clockwise) {
                    added_neighbors.push(maybe_add(new_position, clockwise, 1001));
                }
                let counter_clockwise = direction.counter_clockwise();
                if let Some(new_position) = grid.get_neighbor_position(vertex, counter_clockwise) {
                    added_neighbors.push(maybe_add(new_position, counter_clockwise, 1001));
                }
                debug!(
                    "Added neighbors: {:?}",
                    added_neighbors.into_iter().flatten().collect_vec()
                );
            }
        }

        best_paths
            .iter()
            .fold(HashSet::new(), |mut set, path| {
                for pos in path {
                    set.insert(*pos);
                }
                set
            })
            .len()
    }
}

fn main() {
    let sample = include_str!("../../samples/16.txt");
    let sample_1 = include_str!("../../samples/16_1.txt");
    let sample_2 = include_str!("../../samples/16_2.txt");
    let input = include_str!("../../inputs/16.txt");
    let part_one_problems = [
        aoc::Input::new_sample(sample, PART_ONE_SAMPLE_ANSWER),
        aoc::Input::new_sample(sample_1, 11048),
        aoc::Input::new_sample(sample_2, 21148),
        aoc::Input::new_final(input), // 82376 too high
    ];

    let part_two_problems = [
        aoc::Input::new_sample(sample, PART_TWO_SAMPLE_ANSWER),
        aoc::Input::new_sample(sample_1, 64),
        aoc::Input::new_final(input),
    ];

    Solution {}.run(&part_one_problems, &part_two_problems);
}
