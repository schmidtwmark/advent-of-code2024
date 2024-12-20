use std::collections::{HashMap, HashSet, VecDeque};

use aoc::{Grid, Solver};
use itertools::Itertools;
use log::debug;

type Answer = usize;

const PART_ONE_SAMPLE_ANSWER: Answer = 0;
const PART_TWO_SAMPLE_ANSWER: Answer = 0;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
enum GridObject {
    #[default]
    Path,
    Wall,
    Start,
    End,
}

impl GridObject {
    fn from_char(c: char) -> GridObject {
        match c {
            '.' => GridObject::Path,
            '#' => GridObject::Wall,
            'S' => GridObject::Start,
            'E' => GridObject::End,
            _ => panic!("Unknown character "),
        }
    }
}

fn shortest_paths_to(
    grid: &Grid<GridObject>,
    target: (usize, usize),
) -> HashMap<(usize, usize), usize> {
    let mut queue = VecDeque::new();
    let mut map = HashMap::new();
    queue.push_back((target, 0));
    map.insert(target, 0);
    while let Some((pos, length)) = queue.pop_front() {
        let neighbors = grid.cardinal_neighbor_positions(pos);
        for neighbor in neighbors {
            if !map.contains_key(&neighbor)
                && (*grid.at(neighbor) == GridObject::Path
                    || *grid.at(neighbor) == GridObject::End
                    || *grid.at(neighbor) == GridObject::Start)
            {
                queue.push_back((neighbor, length + 1));
                map.insert(neighbor, length + 1);
            }
        }
    }
    map
}

fn manhattan_within_d(
    (x, y): (usize, usize),
    d: usize,
    grid: &Grid<GridObject>,
) -> Vec<(usize, usize)> {
    let mut points = Vec::new();

    for x_prime in (x.saturating_sub(d))..=(x + d).min(grid.width - 1) {
        let x_delta = x_prime.abs_diff(x);
        let max_distance_y = d.abs_diff(x_delta);
        for y_prime in
            (y.saturating_sub(max_distance_y))..=(y + max_distance_y).min(grid.height - 1)
        {
            match grid.get((x_prime, y_prime)) {
                Some(GridObject::Path | GridObject::End) => points.push((x_prime, y_prime)),
                _ => continue,
            }
        }
    }

    points
}

type Cheat = ((usize, usize), (usize, usize));

fn cheat_savings(lines: &[&str], cheat_length: usize) -> Answer {
    let grid = Grid::from_lines(lines, &GridObject::from_char);
    let start = grid
        .positions()
        .find(|pos| matches!(grid.at(*pos), GridObject::Start))
        .unwrap();
    let end = grid
        .positions()
        .find(|pos| matches!(grid.at(*pos), GridObject::End))
        .unwrap();

    let shortest_path_map = shortest_paths_to(&grid, end);
    let base_shortest = *shortest_path_map.get(&start).unwrap();
    debug!("base_shortest: {}", base_shortest);

    let mut cheat_savings: HashMap<usize, Vec<Cheat>> = HashMap::new();
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();
    queue.push_back((start, 0, None));
    visited.insert((start, None));
    while let Some((pos, length, cheated)) = queue.pop_front() {
        if length >= base_shortest {
            continue;
        }
        if pos == end {
            if let Some((c_start, c_end)) = cheated {
                let savings = base_shortest - length;
                cheat_savings
                    .entry(savings)
                    .or_default()
                    .push((c_start, c_end));
            }
        }

        if cheated.is_none() {
            // Find all spots within 20 steps of this position.
            let distance =
                |p1: (usize, usize), p2: (usize, usize)| p1.0.abs_diff(p2.0) + p1.1.abs_diff(p2.1);

            let positions = manhattan_within_d(pos, cheat_length, &grid);
            for cheat_position in positions {
                let cheat = Some((pos, cheat_position));
                if !visited.contains(&(cheat_position, cheat)) {
                    let cheat_shortest = shortest_path_map.get(&cheat_position).unwrap();
                    let total_length = length + distance(pos, cheat_position) + cheat_shortest;
                    if total_length <= base_shortest {
                        let savings = base_shortest - total_length;
                        cheat_savings
                            .entry(savings)
                            .or_default()
                            .push((pos, cheat_position));
                    }
                }
            }
        }

        let neighbors = grid.cardinal_neighbor_positions(pos);
        for neighbor in neighbors {
            if !visited.contains(&(neighbor, cheated))
                && (*grid.at(neighbor) == GridObject::Path || *grid.at(neighbor) == GridObject::End)
            {
                queue.push_back((neighbor, length + 1, cheated));
                visited.insert((neighbor, cheated));
            }
        }
    }

    let cheat_savings = cheat_savings
        .into_iter()
        .sorted_by_key(|(x, _)| *x)
        .collect_vec();
    for (x, positions) in &cheat_savings {
        debug!("{}: {} positions {:?}", x, positions.len(), positions);
    }

    cheat_savings
        .iter()
        .fold(0, |mut acc, (savings, positions)| {
            if *savings >= 100 {
                acc += positions.len()
            }
            acc
        })
}

struct Solution {}
impl Solver<'_, Answer> for Solution {
    fn solve_part_one(&self, lines: &[&str]) -> Answer {
        cheat_savings(lines, 2)
    }

    fn solve_part_two(&self, lines: &[&str]) -> Answer {
        cheat_savings(lines, 20)
    }
}

fn main() {
    let sample = include_str!("../../samples/20.txt");
    let input = include_str!("../../inputs/20.txt");
    let part_one_problems = [
        aoc::Input::new_sample(sample, PART_ONE_SAMPLE_ANSWER),
        aoc::Input::new_final(input),
    ];

    let part_two_problems = [
        aoc::Input::new_sample(sample, PART_TWO_SAMPLE_ANSWER),
        aoc::Input::new_final(input),
    ];

    Solution {}.run(&part_one_problems, &part_two_problems);
}
