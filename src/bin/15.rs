use core::panic;
use std::fmt::Display;

use aoc::{Cardinal, Grid, Solver};
use itertools::Itertools;
use log::debug;

type Answer = usize;

const PART_ONE_SAMPLE_ANSWER: Answer = 2028;
const PART_TWO_SAMPLE_ANSWER: Answer = 9021;

#[derive(Debug, Clone, Eq, PartialEq, Default, Copy)]
enum GridObject {
    Wall,
    Box,
    Lanternfish,
    #[default]
    Empty,
    BoxLeft,
    BoxRight,
}

impl Display for GridObject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GridObject::Wall => write!(f, "#"),
            GridObject::Box => write!(f, "O"),
            GridObject::Lanternfish => write!(f, "@"),
            GridObject::Empty => write!(f, "."),
            GridObject::BoxLeft => write!(f, "["),
            GridObject::BoxRight => write!(f, "]"),
        }
    }
}

impl GridObject {
    fn from_char(c: char) -> Self {
        match c {
            '#' => GridObject::Wall,
            '.' => GridObject::Empty,
            '@' => GridObject::Lanternfish,
            'O' => GridObject::Box,
            '[' => GridObject::BoxLeft,
            ']' => GridObject::BoxRight,
            _ => panic!("Unknown character "),
        }
    }
}

fn can_push_block(
    block_type: GridObject,
    block_position: (usize, usize),
    grid: &Grid<GridObject>,
    direction: Cardinal,
) -> bool {
    if block_type == GridObject::Empty {
        return true;
    }
    let block_sibling_position = if block_type == GridObject::BoxLeft {
        (block_position.0 + 1, block_position.1)
    } else {
        (block_position.0 - 1, block_position.1)
    };

    let next_position = grid
        .get_neighbor_position(block_position, direction)
        .unwrap();
    let sibling_next_position = grid
        .get_neighbor_position(block_sibling_position, direction)
        .unwrap();
    match (grid.at(next_position), grid.at(sibling_next_position)) {
        (GridObject::Empty, GridObject::Empty) => true,
        (
            GridObject::BoxLeft | GridObject::BoxRight | GridObject::Empty,
            GridObject::BoxLeft | GridObject::BoxRight | GridObject::Empty,
        ) => {
            // do recursive
            can_push_block(*grid.at(next_position), next_position, grid, direction)
                && can_push_block(
                    *grid.at(sibling_next_position),
                    sibling_next_position,
                    grid,
                    direction,
                )
        }
        _ => false,
    }
}

fn count_block_type(grid: &Grid<GridObject>, block_type: GridObject) -> usize {
    grid.positions()
        .filter(|pos| *grid.at(*pos) == block_type)
        .count()
}

fn push_block(
    block_type: GridObject,
    block_position: (usize, usize),
    grid: &mut Grid<GridObject>,
    direction: Cardinal,
) {
    debug!(
        "pushing block {:?} at {:?} in direction {:?} ",
        block_type, block_position, direction
    );
    if block_type != GridObject::BoxLeft && block_type != GridObject::BoxRight {
        return;
    }
    // we know it is safe to push
    let block_sibling_position = if block_type == GridObject::BoxLeft {
        (block_position.0 + 1, block_position.1)
    } else {
        (block_position.0 - 1, block_position.1)
    };
    let next_position = grid
        .get_neighbor_position(block_position, direction)
        .unwrap();
    let sibling_next_position = grid
        .get_neighbor_position(block_sibling_position, direction)
        .unwrap();
    let sibling_block_type = if block_type == GridObject::BoxLeft {
        GridObject::BoxRight
    } else {
        GridObject::BoxLeft
    };
    match (grid.at(next_position), grid.at(sibling_next_position)) {
        (GridObject::Empty, GridObject::Empty) => {
            *grid.mut_at(next_position) = block_type;
            *grid.mut_at(sibling_next_position) = sibling_block_type;
            *grid.mut_at(block_position) = GridObject::Empty;
            *grid.mut_at(block_sibling_position) = GridObject::Empty;
        }
        (
            GridObject::BoxLeft | GridObject::BoxRight | GridObject::Empty,
            GridObject::BoxLeft | GridObject::BoxRight | GridObject::Empty,
        ) => {
            // do recursive
            push_block(
                *grid.at(sibling_next_position),
                sibling_next_position,
                grid,
                direction,
            );
            push_block(*grid.at(next_position), next_position, grid, direction);
            debug!(
                "Setting block_position {:?} and sibling_block_position {:?} to empty",
                block_position, block_sibling_position
            );
            *grid.mut_at(block_position) = GridObject::Empty;
            *grid.mut_at(block_sibling_position) = GridObject::Empty;
            debug!(
                "Setting next_position {:?} to {:?} and sibling_next_position {:?} to {:}",
                next_position, block_type, sibling_next_position, sibling_block_type
            );
            *grid.mut_at(next_position) = block_type;
            *grid.mut_at(sibling_next_position) = sibling_block_type;
        }
        _ => {}
    }
}

struct Solution {}
impl Solver<'_, Answer> for Solution {
    fn solve_part_one(&self, lines: &[&str]) -> Answer {
        let (map, instructions) = lines.split(|s| s.is_empty()).collect_tuple().unwrap();
        let mut grid = Grid::from_lines(map, &GridObject::from_char);

        let instructions = instructions
            .iter()
            .flat_map(|s| s.chars().map(Cardinal::from_char))
            .collect_vec();

        let mut lanternfish_position = grid
            .positions()
            .find(|pos| matches!(grid.at(*pos), GridObject::Lanternfish))
            .unwrap();
        // process instructions
        debug!("Initial state: {}", grid);

        let start_block_left_count = count_block_type(&grid, GridObject::BoxLeft);
        let start_block_right_count = count_block_type(&grid, GridObject::BoxRight);
        let start_wall_count = count_block_type(&grid, GridObject::Wall);

        debug!("start_block_left_count: {}", start_block_left_count);
        debug!("start_block_right_count: {}", start_block_right_count);
        debug!("start_wall_count: {}", start_wall_count);

        for instruction in instructions {
            let neighbors = grid.get_neighbors_along_cardinal(lanternfish_position, instruction);
            // first, check if the lanternfish can move -- either the space is empty, or we can push a line of boxes into an empty space
            match grid.at(neighbors[0]) {
                GridObject::Empty => {
                    *grid.mut_at(lanternfish_position) = GridObject::Empty;
                    *grid.mut_at(neighbors[0]) = GridObject::Lanternfish;
                    lanternfish_position = neighbors[0];
                }
                GridObject::Wall => {
                    // Can't move
                }
                GridObject::Box => {
                    // Push a line of boxes
                    // check if there is an empty space before a wall
                    let mut empty_position = None;
                    for pos in neighbors.iter().skip(1) {
                        match grid.at(*pos) {
                            GridObject::Wall => break,
                            GridObject::Empty => {
                                empty_position = Some(*pos);
                                break;
                            }
                            _ => (),
                        }
                    }
                    if let Some(empty) = empty_position {
                        *grid.mut_at(lanternfish_position) = GridObject::Empty;
                        *grid.mut_at(neighbors[0]) = GridObject::Lanternfish;
                        lanternfish_position = neighbors[0];
                        for pos in neighbors.iter().skip(1) {
                            *grid.mut_at(*pos) = GridObject::Box;
                            if *pos == empty {
                                break;
                            }
                        }
                    }
                }
                _ => {
                    panic!("Can't have neighbor be a lanternfish!");
                }
            }
            debug!(
                "After instruction {:?} with counts ({}, {}, {}): {}",
                instruction,
                count_block_type(&grid, GridObject::BoxLeft),
                count_block_type(&grid, GridObject::BoxRight),
                count_block_type(&grid, GridObject::Wall),
                grid
            );
            if start_block_left_count != count_block_type(&grid, GridObject::BoxLeft)
                || start_block_right_count != count_block_type(&grid, GridObject::BoxRight)
                || start_wall_count != count_block_type(&grid, GridObject::Wall)
            {
                panic!("Block counts changed! start_block_left_count: {:?}, start_block_right_count: {:?}, startwall_count: {:?}, block_left_count: {:?}, block_right_count: {:?}, wall_count: {:?}, grid: {}", grid, start_block_left_count, start_block_right_count, start_wall_count, count_block_type(&grid, GridObject::BoxLeft), count_block_type(&grid, GridObject::BoxRight), count_block_type(&grid, GridObject::Wall));
            }
        }

        grid.positions()
            .filter_map(|pos| match grid.at(pos) {
                GridObject::Box => Some(pos.0 + 100 * pos.1),
                _ => None,
            })
            .sum()
    }

    fn solve_part_two(&self, lines: &[&str]) -> Answer {
        let (map, instructions) = lines.split(|s| s.is_empty()).collect_tuple().unwrap();
        let map = map
            .iter()
            .map(|s| {
                s.chars()
                    .map(|c| match c {
                        '#' => "##",
                        '.' => "..",
                        '@' => "@.",
                        'O' => "[]",
                        _ => panic!("Unknown character "),
                    })
                    .collect::<String>()
            })
            .collect_vec();
        let map: Vec<&str> = map.iter().map(std::ops::Deref::deref).collect();

        let mut grid = Grid::from_lines(&map, &GridObject::from_char);

        let instructions = instructions
            .iter()
            .flat_map(|s| s.chars().map(Cardinal::from_char))
            .collect_vec();

        let mut lanternfish_position = grid
            .positions()
            .find(|pos| matches!(grid.at(*pos), GridObject::Lanternfish))
            .unwrap();
        // process instructions
        debug!("Initial state: {}", grid);

        let start_block_left_count = count_block_type(&grid, GridObject::BoxLeft);
        let start_block_right_count = count_block_type(&grid, GridObject::BoxRight);
        let start_wall_count = count_block_type(&grid, GridObject::Wall);
        debug!("start_block_left_count: {}", start_block_left_count);
        debug!("start_block_right_count: {}", start_block_right_count);
        debug!("start_wall_count: {}", start_wall_count);
        for instruction in instructions {
            let neighbors = grid.get_neighbors_along_cardinal(lanternfish_position, instruction);
            // first, check if the lanternfish can move -- either the space is empty, or we can push a line of boxes into an empty space
            let first_neighbor = *grid.at(neighbors[0]);
            match first_neighbor {
                GridObject::Empty => {
                    *grid.mut_at(lanternfish_position) = GridObject::Empty;
                    *grid.mut_at(neighbors[0]) = GridObject::Lanternfish;
                    lanternfish_position = neighbors[0];
                }
                GridObject::Wall => {
                    // Can't move
                }
                GridObject::Box => {}
                GridObject::BoxLeft | GridObject::BoxRight => {
                    match instruction {
                        Cardinal::East | Cardinal::West => {
                            // Easy case, just look for an empty spot in neighbors like before
                            let mut empty_position = None;
                            for pos in neighbors.iter().skip(1) {
                                match grid.at(*pos) {
                                    GridObject::Wall => break,
                                    GridObject::Empty => {
                                        empty_position = Some(*pos);
                                        break;
                                    }
                                    _ => (),
                                }
                            }
                            if let Some(empty) = empty_position {
                                *grid.mut_at(lanternfish_position) = GridObject::Empty;
                                *grid.mut_at(neighbors[0]) = GridObject::Lanternfish;
                                lanternfish_position = neighbors[0];
                                let mut to_copy = first_neighbor;
                                for pos in neighbors.iter().skip(1) {
                                    let temp = *grid.at(*pos);
                                    *grid.mut_at(*pos) = to_copy;
                                    to_copy = temp;
                                    if *pos == empty {
                                        break;
                                    }
                                }
                            }
                        }
                        Cardinal::North | Cardinal::South => {
                            if can_push_block(first_neighbor, neighbors[0], &grid, instruction) {
                                push_block(first_neighbor, neighbors[0], &mut grid, instruction);
                                *grid.mut_at(lanternfish_position) = GridObject::Empty;
                                *grid.mut_at(neighbors[0]) = GridObject::Lanternfish;
                                lanternfish_position = neighbors[0];
                            }
                        }
                    }
                }
                GridObject::Lanternfish => {
                    panic!("Can't have neighbor be a lanternfish!");
                }
            }
            debug!(
                "After instruction {:?} with counts ({}, {}, {}): {}",
                instruction,
                count_block_type(&grid, GridObject::BoxLeft),
                count_block_type(&grid, GridObject::BoxRight),
                count_block_type(&grid, GridObject::Wall),
                grid
            );
            if start_block_left_count != count_block_type(&grid, GridObject::BoxLeft)
                || start_block_right_count != count_block_type(&grid, GridObject::BoxRight)
                || start_wall_count != count_block_type(&grid, GridObject::Wall)
            {
                panic!("Block counts changed! start_block_left_count: {:?}, start_block_right_count: {:?}, startwall_count: {:?}, block_left_count: {:?}, block_right_count: {:?}, wall_count: {:?}, grid: {}", grid, start_block_left_count, start_block_right_count, start_wall_count, count_block_type(&grid, GridObject::BoxLeft), count_block_type(&grid, GridObject::BoxRight), count_block_type(&grid, GridObject::Wall));
            }
        }

        grid.positions()
            .filter_map(|pos| match grid.at(pos) {
                GridObject::BoxLeft => Some(pos.0 + 100 * pos.1),
                _ => None,
            })
            .sum()
    }
}

fn main() {
    let sample = include_str!("../../samples/15.txt");
    let sample_1 = include_str!("../../samples/15_1.txt");
    let sample_2 = include_str!("../../samples/15_2.txt");
    let sample_3 = include_str!("../../samples/15_3.txt");
    let input = include_str!("../../inputs/15.txt");
    let part_one_problems = [
        aoc::Input::new_sample(sample, PART_ONE_SAMPLE_ANSWER),
        aoc::Input::new_sample(sample_1, 10092),
        aoc::Input::new_final(input),
    ];

    let part_two_problems = [
        aoc::Input::new_sample(sample, 1751),
        aoc::Input::new_sample(sample_1, PART_TWO_SAMPLE_ANSWER),
        aoc::Input::new_sample(sample_2, 618),
        aoc::Input::new_sample(sample_3, 513),
        aoc::Input::new_final(input),
        // 1543780 too high
    ];

    Solution {}.run(&part_one_problems, &part_two_problems);
}
