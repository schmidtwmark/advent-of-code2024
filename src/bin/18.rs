use std::collections::{HashSet, VecDeque};

use aoc::{Grid, Solver};
use itertools::Itertools;
use log::debug;

type Answer = String;

const PART_ONE_SAMPLE_ANSWER: &str = "22";
const PART_TWO_SAMPLE_ANSWER: &str = "6,1";

#[derive(Default, Debug, Clone, Eq, PartialEq)]
enum GridObject {
    #[default]
    Empty,
    Wall(usize), // Time the wall appears in the map
}

struct Solution {}
impl Solver<'_, Answer> for Solution {
    fn solve_part_one(&self, lines: &[&str]) -> Answer {
        let (number_to_drop, size) = if lines.len() < 30 {
            (12, (7, 7))
        } else {
            (1024, (71, 71))
        };

        let mut grid = Grid::<GridObject>::new_empty(size.0, size.1);
        for (t, line) in lines.iter().enumerate().take(number_to_drop) {
            let (x, y) = line.split_once(",").unwrap();
            *grid.mut_at((x.parse().unwrap(), y.parse().unwrap())) = GridObject::Wall(t);
        }

        let mut queue = VecDeque::new();
        let mut visited = HashSet::new();

        queue.push_back(((0, 0), 0));
        visited.insert((0, 0));

        while let Some((pos, length)) = queue.pop_front() {
            if pos == (size.0 - 1, size.1 - 1) {
                return length.to_string();
            }
            let neighbors = grid.cardinal_neighbor_positions(pos);
            for neighbor in neighbors {
                if !visited.contains(&neighbor) && *grid.at(neighbor) == GridObject::Empty {
                    queue.push_back((neighbor, length + 1));
                    visited.insert(neighbor);
                }
            }

            visited.insert(pos);
        }
        Answer::default()
    }

    fn solve_part_two(&self, lines: &[&str]) -> Answer {
        let size = if lines.len() < 30 { (7, 7) } else { (71, 71) };

        let wall_coords = lines
            .iter()
            .enumerate()
            .map(|(idx, line)| {
                let (x, y) = line.split_once(",").unwrap();
                (idx, (x.parse().unwrap(), y.parse().unwrap()))
            })
            .collect_vec();
        let mut grid = Grid::<GridObject>::new_empty(size.0, size.1);
        for (t, pos) in wall_coords.iter() {
            *grid.mut_at(*pos) = GridObject::Wall(*t);
        }

        let get_shortest_path = |t: usize| {
            let mut queue = VecDeque::new();
            let mut visited = HashSet::new();

            queue.push_back(((0, 0), 0));
            visited.insert((0, 0));

            while let Some((pos, length)) = queue.pop_front() {
                if pos == (size.0 - 1, size.1 - 1) {
                    return Some(length);
                }
                let neighbors = grid.cardinal_neighbor_positions(pos);
                for neighbor in neighbors {
                    if !visited.contains(&neighbor) {
                        let add = match grid.at(neighbor) {
                            GridObject::Empty => true,
                            GridObject::Wall(wt) if t < *wt => true,
                            _ => false,
                        };
                        if add {
                            queue.push_back((neighbor, length + 1));
                            visited.insert(neighbor);
                        }
                    }
                }

                visited.insert(pos);
            }
            None
        };
        let first_no_path = wall_coords.partition_point(|(idx, _)| {
            let shortest_path = get_shortest_path(*idx);
            debug!("Testing t = {}, shortest path: {:?}", idx, shortest_path);
            shortest_path.is_some()
        });

        let coord = wall_coords[first_no_path];
        debug!("First no path: {} {:?}", first_no_path, coord);
        format!("{},{}", coord.1 .0, coord.1 .1)
    }
}

fn main() {
    let sample = include_str!("../../samples/18.txt");
    let input = include_str!("../../inputs/18.txt");
    let part_one_problems = [
        aoc::Input::new_sample(sample, PART_ONE_SAMPLE_ANSWER.to_owned()),
        aoc::Input::new_final(input),
    ];

    let part_two_problems = [
        aoc::Input::new_sample(sample, PART_TWO_SAMPLE_ANSWER.to_owned()),
        aoc::Input::new_final(input),
    ];

    Solution {}.run(&part_one_problems, &part_two_problems);
}
