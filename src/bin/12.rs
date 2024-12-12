use aoc::Solver;
use itertools::Itertools;
use log::debug;
use std::collections::{HashMap, HashSet, VecDeque};

type Answer = isize;

const PART_ONE_SAMPLE_ANSWER: Answer = 1930;
const PART_TWO_SAMPLE_ANSWER: Answer = 1206;

#[derive(Debug)]
struct Region {
    positions: HashSet<(usize, usize)>,
    perimeter: isize,
    sides: isize,
    identifier: char,
}

fn regions(lines: &[&str]) -> Vec<Region> {
    let grid = aoc::Grid::from_lines(lines, &|c| c);

    let mut unseen: HashSet<_> = grid.positions().collect();

    let mut regions = Vec::<Region>::new();
    while !unseen.is_empty() {
        // Nothing connected
        let start = *unseen.iter().next().unwrap();
        unseen.remove(&start);
        let mut region = Region {
            positions: HashSet::from([start]),
            perimeter: 4,
            sides: 4,
            identifier: *grid.at(start),
        };
        let mut to_visit = VecDeque::new();
        to_visit.push_back(start);
        debug!("Starting region {} {start:?}", region.identifier);
        while let Some(current) = to_visit.pop_front() {
            let region_neighbors = grid
                .cardinal_neighbor_positions(current)
                .into_iter()
                .filter(|pos| *grid.at(*pos) == region.identifier)
                .collect_vec();
            debug!(
                "Processing {current:?} in region {} with neighbors {:?}",
                region.identifier, region_neighbors
            );
            for neighbor in region_neighbors {
                if unseen.contains(&neighbor) {
                    unseen.remove(&neighbor);
                    let neighbor_neighbors: HashSet<_> = grid
                        .cardinal_neighbor_positions(neighbor)
                        .into_iter()
                        .filter(|pos| *grid.at(*pos) == region.identifier)
                        .collect();
                    debug!(
                            "Adding neighbor {neighbor:?} Neighbor neighbors: {:?}, region: {:?} intersection: {:?}",
                            neighbor_neighbors,
                            region,
                            neighbor_neighbors
                                .intersection(&region.positions)
                                .collect_vec()
                        );
                    region.perimeter +=
                        4 - 2 * neighbor_neighbors.intersection(&region.positions).count() as isize;
                    region.positions.insert(neighbor);
                    to_visit.push_back(neighbor);
                }
            }
        }
        // process edges

        let edges: HashSet<((usize, usize), aoc::Cardinal)> =
            region
                .positions
                .iter()
                .fold(HashSet::new(), |mut map, pos| {
                    let edge_dirs = aoc::Cardinal::all().into_iter().filter(|cardinal| {
                        if let Some(neighbor) = grid.get_neighbor_position(*pos, *cardinal) {
                            if region.positions.contains(&neighbor) {
                                return false;
                            }
                        }
                        return true;
                    });
                    for edge_dir in edge_dirs {
                        map.insert((*pos, edge_dir));
                    }
                    map
                });
        let mut unvisited_edges: HashSet<_> = edges.clone();
        let mut sides = 0;
        while !unvisited_edges.is_empty() {
            sides += 1;
            let (start_pos, edge_dir) = *unvisited_edges.iter().next().unwrap();
            unvisited_edges.remove(&(start_pos, edge_dir));
            for movement_dir in [edge_dir.clockwise(), edge_dir.counter_clockwise()] {
                let mut current = start_pos;
                while let Some(neighbor) = grid.get_neighbor_position(current, movement_dir) {
                    if edges.contains(&(neighbor, edge_dir)) {
                        unvisited_edges.remove(&(neighbor, edge_dir));
                        current = neighbor;
                    } else {
                        break;
                    }
                }
            }
        }
        region.sides = sides;

        regions.push(region);
    }
    regions
}

struct Solution {}
impl Solver<'_, isize> for Solution {
    fn solve_part_one(&self, lines: &[&str]) -> Answer {
        regions(lines)
            .iter()
            .map(|r| {
                debug!("{:?}", r);
                r.positions.len() as isize * r.perimeter
            })
            .sum()
    }

    fn solve_part_two(&self, lines: &[&str]) -> Answer {
        regions(lines)
            .iter()
            .map(|r| {
                debug!("{:?}", r);
                r.positions.len() as isize * r.sides
            })
            .sum()
    }
}

fn main() {
    let sample = include_str!("../../samples/12.txt");
    let sample_1 = include_str!("../../samples/12_1.txt");
    let input = include_str!("../../inputs/12.txt");
    let part_one_problems = [
        aoc::Input::new_sample(sample, PART_ONE_SAMPLE_ANSWER),
        aoc::Input::new_sample(sample_1, 140),
        aoc::Input::new_final(input),
    ];

    let part_two_problems = [
        aoc::Input::new_sample(sample, PART_TWO_SAMPLE_ANSWER),
        aoc::Input::new_sample(sample_1, 80),
        aoc::Input::new_final(input),
    ];

    Solution {}.run(&part_one_problems, &part_two_problems);
}
