use std::collections::HashSet;

use aoc::{Graph, Solver};
use itertools::Itertools;
use log::debug;

type Answer = String;

struct Solution {}
impl Solver<'_, Answer> for Solution {
    fn solve_part_one(&self, lines: &[&str]) -> Answer {
        let graph = lines.iter().fold(Graph::new(), |mut graph, line| {
            let (a, b) = line.split_once('-').unwrap();
            graph.add_edge(a, b, ());
            graph.add_edge(b, a, ());
            graph
        });

        debug!("Graph: {:?}", graph);

        let mut components = HashSet::new();
        for vertex in graph.all_vertices() {
            let edges = graph.edges.get(vertex).unwrap();
            edges
                .iter()
                .cartesian_product(edges.iter())
                .for_each(|(a, b)| {
                    if a != b {
                        let a_edges = graph.edges.get(a.0).unwrap();
                        let b_edges = graph.edges.get(b.0).unwrap();
                        if a_edges.contains_key(b.0)
                            && a_edges.contains_key(vertex)
                            && b_edges.contains_key(a.0)
                            && b_edges.contains_key(vertex)
                        {
                            let mut component = [vertex, a.0, b.0];
                            component.sort();
                            components.insert(component);
                        }
                    }
                })
        }

        debug!("{} Components: {:?}", components.len(), components);

        components
            .iter()
            .filter(|c| c.iter().any(|s| s.starts_with("t")))
            .count()
            .to_string()
    }

    fn solve_part_two(&self, lines: &[&str]) -> Answer {
        let graph = lines.iter().fold(Graph::new(), |mut graph, line| {
            let (a, b) = line.split_once('-').unwrap();
            graph.add_edge(a, b, ());
            graph.add_edge(b, a, ());
            graph
        });

        debug!("Graph: {:?}", graph);

        let mut current_components =
            graph
                .all_vertices()
                .fold(HashSet::new(), |mut components, vertex| {
                    components.insert(vec![vertex]);
                    components
                });
        let mut components = HashSet::new();
        loop {
            debug!("Current Components: {:?}", current_components);
            components.clear();
            for component in &current_components {
                let edges = component.iter().fold(HashSet::new(), |mut edges, vertex| {
                    edges.extend(
                        graph
                            .edges
                            .get(*vertex)
                            .unwrap()
                            .iter()
                            .filter(|v| !component.contains(&v.0)),
                    );
                    edges
                });
                let mut new_component = component.clone();
                // all the edges adjacent to this component
                // Of these, add the ones that are adjacent to EVERY vertex in this component
                edges.iter().for_each(|n| {
                    let neighbor_edges = graph.edges.get(n.0).unwrap();
                    if new_component
                        .iter()
                        .all(|v| neighbor_edges.contains_key(*v))
                    {
                        new_component.push(n.0);
                    }
                });
                new_component.sort();

                if new_component.len() > component.len() {
                    components.insert(new_component);
                }
            }
            if components.is_empty() {
                break;
            } else {
                current_components = components.clone();
            }
        }

        debug!(
            "{} Components: {:?}",
            current_components.len(),
            current_components
        );

        current_components
            .iter()
            .max_by_key(|c| c.len())
            .unwrap()
            .iter()
            .join(",")
    }
}

fn main() {
    let sample = include_str!("../../samples/23.txt");
    let input = include_str!("../../inputs/23.txt");
    let part_one_problems = [
        aoc::Input::new_sample(sample, "7".to_string()),
        aoc::Input::new_final(input), // 2419 too high
    ];

    let part_two_problems = [
        aoc::Input::new_sample(sample, "co,de,ka,ta".to_string()),
        aoc::Input::new_final(input),
    ];

    Solution {}.run(&part_one_problems, &part_two_problems);
}
