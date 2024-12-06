use core::panic;
use std::{collections::HashSet, io::Empty, str::FromStr};

use aoc::{Grid, Solver};
use itertools::Itertools;
use log::debug;

#[derive(Debug, Default, Clone, Eq, PartialEq, Hash, Copy)]
enum Direction {
    #[default]
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn turn_right(self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
        }
    }

    fn move_forward(&self, pos: (isize, isize)) -> (isize, isize) {
        match self {
            Direction::Up => (pos.0, pos.1 - 1),
            Direction::Down => (pos.0, pos.1 + 1),
            Direction::Left => (pos.0 - 1, pos.1),
            Direction::Right => (pos.0 + 1, pos.1),
        }
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
enum GridObject {
    Wall,
    #[default]
    Empty,
    Guard(Direction),
}

impl GridObject {
    fn from_char(c: char) -> Self {
        match c {
            '#' => GridObject::Wall,
            '.' => GridObject::Empty,
            '^' => GridObject::Guard(Direction::Up),
            'v' => GridObject::Guard(Direction::Down),
            '<' => GridObject::Guard(Direction::Left),
            '>' => GridObject::Guard(Direction::Right),
            _ => panic!("Unknown character "),
        }
    }
}

struct Solution {}
impl Solver<'_, usize> for Solution {
    fn solve_part_one(&self, lines: &[&str]) -> usize {
        let grid = Grid::from_lines(lines, &GridObject::from_char);
        let guard_position = grid
            .positions()
            .find(|pos| *grid.at(*pos) == GridObject::Guard(Direction::Up))
            .unwrap();
        let mut guard_position = (guard_position.0 as isize, guard_position.1 as isize);
        let mut direction = Direction::Up;
        let mut visited = HashSet::<(isize, isize)>::new();
        visited.insert(guard_position);

        loop {
            let new_position = direction.move_forward(guard_position);
            if let Some(object) = grid.get_isize(new_position) {
                match object {
                    GridObject::Wall => {
                        direction = direction.turn_right();
                    }
                    GridObject::Empty | GridObject::Guard(_) => {
                        visited.insert(new_position);
                        guard_position = new_position;
                    }
                }
            } else {
                break;
            }
        }

        visited.len()
    }

    fn solve_part_two(&self, lines: &[&str]) -> usize {
        let grid = Grid::from_lines(lines, &GridObject::from_char);
        let guard_position = grid
            .positions()
            .find(|pos| *grid.at(*pos) == GridObject::Guard(Direction::Up))
            .unwrap();
        let guard_start = (guard_position.0 as isize, guard_position.1 as isize);
        let start_direction = Direction::Up;

        grid.positions()
            .filter(|pos| {
                debug!("Checking {:?}", pos);
                if grid.at(*pos) != &GridObject::Empty {
                    return false;
                }
                let mut grid = grid.clone();
                *grid.mut_at(*pos) = GridObject::Wall;
                let mut guard_position = guard_start;
                let mut direction = start_direction;
                let mut visited = HashSet::<((isize, isize), Direction)>::new();
                visited.insert((guard_position, direction));

                loop {
                    let new_position = direction.move_forward(guard_position);
                    if let Some(object) = grid.get_isize(new_position) {
                        match object {
                            GridObject::Wall => {
                                direction = direction.turn_right();
                            }
                            GridObject::Empty | GridObject::Guard(_) => {
                                if visited.contains(&(new_position, direction)) {
                                    return true;
                                }
                                visited.insert((new_position, direction));
                                guard_position = new_position;
                            }
                        }
                    } else {
                        return false;
                    }
                }
            })
            .count()
    }
}

fn main() {
    let sample = include_str!("../../samples/6.txt");
    let input = include_str!("../../inputs/6.txt");
    let part_one_problems = [
        aoc::Input::new_sample(sample, 41),
        aoc::Input::new_final(input),
    ];

    let part_two_problems = [
        aoc::Input::new_sample(sample, 6),
        aoc::Input::new_final(input),
    ];

    Solution {}.run(&part_one_problems, &part_two_problems);
}
