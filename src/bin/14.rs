use std::collections::HashMap;

use aoc::Solver;
use itertools::Itertools;
use log::debug;
use regex::Regex;

type Answer = usize;

const PART_ONE_SAMPLE_ANSWER: Answer = 12;
const PART_TWO_SAMPLE_ANSWER: Answer = 0;

fn debug_robots(grid_size: (isize, isize), robots: &HashMap<(isize, isize), Vec<(isize, isize)>>) {
    for y in 0..grid_size.1 {
        let line = (0..grid_size.0)
            .map(|x| {
                if let Some(_velocities) = robots.get(&(x, y)) {
                    '*'
                    // if velocities.len() >= 10 {
                    //     '*'
                    // } else {
                    //     (velocities.len() as u8).to_string().chars().next().unwrap()
                    // }
                } else {
                    ' '
                }
            })
            .collect::<String>();
        debug!("{}", line);
    }
}

struct Solution {}
impl Solver<'_, Answer> for Solution {
    fn solve_part_one(&self, lines: &[&str]) -> Answer {
        let size = if lines.len() == 12 {
            (11, 7)
        } else {
            (101, 103)
        };
        debug!("Size {:?}", size);

        let regex = Regex::new(r"p=([0-9]+),([0-9]+) v=(-?[0-9]+),(-?[0-9]+)").unwrap();
        let mut robots: HashMap<(isize, isize), Vec<(isize, isize)>> = lines
            .iter()
            .map(|line| {
                let captures = regex.captures(line).unwrap();
                let nums = captures
                    .iter()
                    .skip(1)
                    .map(|n| n.unwrap().as_str().parse::<isize>().unwrap())
                    .collect_vec();
                let position: (isize, isize) =
                    nums.iter().take(2).copied().collect_tuple().unwrap();
                let velocity: (isize, isize) = nums
                    .iter()
                    .skip(2)
                    .take(2)
                    .copied()
                    .collect_tuple()
                    .unwrap();
                (position, velocity)
            })
            .fold(HashMap::new(), |mut map, (position, velocity)| {
                map.entry(position).or_default().push(velocity);
                map
            });
        debug!("Initial state");
        debug_robots(size, &robots);

        let steps = 100;
        for _i in 0..steps {
            robots = robots
                .iter()
                .fold(HashMap::new(), |mut map, (position, velocities)| {
                    for velocity in velocities {
                        let new_position = (
                            (position.0 + velocity.0).rem_euclid(size.0),
                            (position.1 + velocity.1).rem_euclid(size.1),
                        );
                        map.entry(new_position).or_default().push(*velocity);
                    }
                    map
                });

            debug!("Step {}", _i);
            debug_robots(size, &robots);
        }
        debug!("Final state");
        debug!(
            "Robots: {:?}",
            robots.iter().map(|(p, v)| (p, v.len())).collect_vec()
        );
        let mid_x = size.0 / 2;
        let mid_y = size.1 / 2;
        let quadrants = [
            (0..mid_x, 0..mid_y),                       // top left
            (0..mid_x, (mid_y + 1)..size.1),            // bottom left
            ((mid_x + 1)..size.0, 0..mid_y),            // top right
            ((mid_x + 1)..size.0, (mid_y + 1)..size.1), // bottom right
        ];
        quadrants
            .iter()
            .map(|(xs, ys)| {
                let total: usize = robots
                    .iter()
                    .filter_map(|(p, vs)| {
                        if xs.contains(&p.0) && ys.contains(&p.1) {
                            Some(vs.len())
                        } else {
                            None
                        }
                    })
                    .sum();
                debug!("Quadrant {:?} has {} robots", (xs, ys), total);
                total
            })
            .product()
    }

    fn solve_part_two(&self, lines: &[&str]) -> Answer {
        let size = if lines.len() == 12 {
            return 0;
        } else {
            (101, 103)
        };
        debug!("Size {:?}", size);

        let regex = Regex::new(r"p=([0-9]+),([0-9]+) v=(-?[0-9]+),(-?[0-9]+)").unwrap();
        let mut robots: HashMap<(isize, isize), Vec<(isize, isize)>> = lines
            .iter()
            .map(|line| {
                let captures = regex.captures(line).unwrap();
                let nums = captures
                    .iter()
                    .skip(1)
                    .map(|n| n.unwrap().as_str().parse::<isize>().unwrap())
                    .collect_vec();
                let position: (isize, isize) =
                    nums.iter().take(2).copied().collect_tuple().unwrap();
                let velocity: (isize, isize) = nums
                    .iter()
                    .skip(2)
                    .take(2)
                    .copied()
                    .collect_tuple()
                    .unwrap();
                (position, velocity)
            })
            .fold(HashMap::new(), |mut map, (position, velocity)| {
                map.entry(position).or_default().push(velocity);
                map
            });
        debug!("Initial state");
        debug_robots(size, &robots);

        let steps = 10000;
        for _i in 0..steps {
            robots = robots
                .iter()
                .fold(HashMap::new(), |mut map, (position, velocities)| {
                    for velocity in velocities {
                        let new_position = (
                            (position.0 + velocity.0).rem_euclid(size.0),
                            (position.1 + velocity.1).rem_euclid(size.1),
                        );
                        map.entry(new_position).or_default().push(*velocity);
                    }
                    map
                });

            debug!("Seconds elapsed {}", _i + 1);
            debug_robots(size, &robots);
        }
        debug!("Final state");
        debug!(
            "Robots: {:?}",
            robots.iter().map(|(p, v)| (p, v.len())).collect_vec()
        );
        0
    }
}

fn main() {
    let sample = include_str!("../../samples/14.txt");
    let input = include_str!("../../inputs/14.txt");
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
