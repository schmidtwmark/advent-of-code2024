use clap::Parser;
use itertools::Itertools;
use log::{debug, error, info};
use simple_logger::SimpleLogger;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt;
use std::fmt::Debug;
use std::fmt::Display;
use std::str::FromStr;
use std::thread;
use std::time::Duration;
use std::time::Instant;

#[derive(Clone, PartialEq, Eq, Copy)]
pub enum Part {
    One,
    Two,
    All,
}

impl FromStr for Part {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "1" => Ok(Part::One),
            "2" => Ok(Part::Two),
            "all" => Ok(Part::All),
            _ => Err(format!("Unknown part {s}")),
        }
    }
}

impl Display for Part {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Part::One => write!(f, "1"),
            Part::Two => write!(f, "2"),
            Part::All => write!(f, "all"),
        }
    }
}

#[derive(Clone, PartialEq, Eq, Copy)]
pub enum Target {
    Sample(usize),
    Samples,
    Final,
    All,
}

impl FromStr for Target {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "final" | "f" => Ok(Target::Final),
            "all" | "a" => Ok(Target::All),
            "samples" | "s" => Ok(Target::Samples),
            _ => {
                if let Ok(num) = s.parse() {
                    Ok(Target::Sample(num))
                } else {
                    Err::<Self, Self::Err>(format!("Unknown target {s}"))
                }
            }
        }
    }
}

impl Display for Target {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Target::Sample(idx) => write!(f, "sample {idx}"),
            Target::Samples => write!(f, "samples"),
            Target::Final => write!(f, "final"),
            Target::All => write!(f, "all"),
        }
    }
}

impl Target {
    fn filter_inputs<'a, D: InputResult>(
        self,
        inputs: &'a [Input<D>],
    ) -> impl Iterator<Item = (usize, &'a Input<'a, D>)> {
        if let Target::Sample(idx) = self {
            if idx >= inputs.len() || inputs[idx].solution.is_none() {
                panic!(
                    "Sample #{} does not exist, inputs: {:?}",
                    idx,
                    inputs
                        .iter()
                        .enumerate()
                        .map(|(idx, input)| (idx, input.solution.clone()))
                        .collect_vec()
                );
            }
        }

        inputs
            .iter()
            .enumerate()
            .filter(move |(idx, input)| match self {
                Target::Sample(sample_idx) => input.solution.is_some() && *idx == sample_idx,
                Target::Samples => input.solution.is_some(),
                Target::Final => input.solution.is_none(),
                Target::All => true,
            })
    }
}

#[derive(Parser, Copy, Clone)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[arg(short, long, default_value_t = Part::All)]
    pub part: Part,
    #[arg(short, long, default_value_t = Target::All)]
    pub target: Target,
}

pub trait InputResult: Display + Send + Sync + Eq + PartialEq + Debug + Clone {}
impl<T> InputResult for T where T: Display + Send + Sync + Eq + PartialEq + Debug + Clone {}

pub trait Solver<'a, D>: Sync
where
    D: InputResult + Sync + 'a,
{
    fn solve_part_one(&self, lines: &[&str]) -> D;
    fn solve_part_two(&self, lines: &[&str]) -> D;

    fn run_single(&self, solver: &(dyn Fn(&[&str]) -> D + Sync), lines: &[&str]) -> (D, Duration) {
        let start = Instant::now();
        let result = solver(lines);
        let elapsed = start.elapsed();
        (result, elapsed)
    }

    fn run_part_one(&self, lines: &[&str]) -> (D, Duration) {
        self.run_single(&|lines| self.solve_part_one(lines), lines)
    }

    fn run_all_for_solver<'b, const PART: u8>(
        &'a self,
        solver: &'b (dyn Fn(&[&str]) -> D + Sync),
        inputs: impl Iterator<Item = (usize, &'a Input<'a, D>)>,
    ) {
        thread::scope(|s| {
            for (idx, input) in inputs {
                s.spawn(move || {
                    let (result, elapsed) =
                        self.run_single(solver, get_lines(input.data).as_slice());
                    if let Some(solution) = &input.solution {
                        if solution == &result {
                            info!(
                                "Part {PART} sample #{idx} passed: {} ({:?})",
                                result, elapsed
                            );
                        } else {
                            error!(
                                "Part {PART} sample #{idx} failed : {} (expected {}, {:?})",
                                result, solution, elapsed
                            );
                        }
                    } else {
                        info!("Part {PART} final: {} ({:?})", result, elapsed);
                    }
                });
            }
        })
    }

    fn run_all_part_one(&'a self, inputs: impl Iterator<Item = (usize, &'a Input<'a, D>)>) {
        self.run_all_for_solver::<1>(&|lines| self.solve_part_one(lines), inputs);
    }

    fn run_part_two(&self, lines: &[&str]) -> (D, Duration) {
        self.run_single(&|lines| self.solve_part_two(lines), lines)
    }
    fn run_all_part_two(&'a self, inputs: impl Iterator<Item = (usize, &'a Input<'a, D>)>) {
        self.run_all_for_solver::<2>(&|lines| self.solve_part_two(lines), inputs);
    }

    fn run(&'a self, part_one_inputs: &'a [Input<D>], part_two_inputs: &'a [Input<D>]) {
        let args = Cli::parse();

        SimpleLogger::new().env().init().unwrap();
        thread::scope(|s| {
            if args.part == Part::One || args.part == Part::All {
                let part_one_inputs = args.target.filter_inputs(part_one_inputs);
                s.spawn(|| {
                    self.run_all_part_one(part_one_inputs);
                });
            }
            if args.part == Part::Two || args.part == Part::All {
                let part_two_inputs = args.target.filter_inputs(part_two_inputs);
                s.spawn(|| {
                    self.run_all_part_two(part_two_inputs);
                });
            }
        })
    }
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct Input<'a, D>
where
    D: InputResult,
{
    data: &'a str,
    solution: Option<D>,
}

impl<'a, D> Input<'a, D>
where
    D: InputResult,
{
    pub fn new_sample(sample: &'a str, solution: D) -> Self {
        Self {
            data: sample,
            solution: Some(solution),
        }
    }

    pub fn new_final(input: &'a str) -> Self {
        Self {
            data: input,
            solution: None,
        }
    }
}

fn get_lines(file: &str) -> Vec<&str> {
    file.lines().collect_vec()
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct Grid<T> {
    pub state: Vec<T>,
    pub width: usize,
    pub height: usize,
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub enum Cardinal {
    North,
    South,
    East,
    West,
}

impl Cardinal {
    pub fn opposite(&self) -> Cardinal {
        match self {
            Cardinal::North => Cardinal::South,
            Cardinal::South => Cardinal::North,
            Cardinal::East => Cardinal::West,
            Cardinal::West => Cardinal::East,
        }
    }
    pub fn clockwise(&self) -> Cardinal {
        match self {
            Cardinal::North => Cardinal::East,
            Cardinal::South => Cardinal::West,
            Cardinal::East => Cardinal::South,
            Cardinal::West => Cardinal::North,
        }
    }

    pub fn counter_clockwise(&self) -> Cardinal {
        self.clockwise().clockwise().clockwise()
    }

    pub fn all() -> Vec<Cardinal> {
        vec![
            Cardinal::North,
            Cardinal::South,
            Cardinal::East,
            Cardinal::West,
        ]
    }

    pub fn from_char(c: char) -> Cardinal {
        match c {
            'N' | '^' => Cardinal::North,
            'S' | 'v' => Cardinal::South,
            'E' | '>' => Cardinal::East,
            'W' | '<' => Cardinal::West,
            _ => panic!("Unknown cardinal"),
        }
    }
}

impl<T: Default + Clone> Grid<T> {
    pub fn new_empty(width: usize, height: usize) -> Grid<T> {
        Grid {
            state: vec![T::default(); width * height],
            width,
            height,
        }
    }

    pub fn new(state: Vec<T>, width: usize, height: usize) -> Grid<T> {
        Grid {
            state,
            width,
            height,
        }
    }

    pub fn from_2d(initial: Vec<Vec<T>>) -> Grid<T> {
        let height = initial.len();
        let width = initial[0].len();
        Grid {
            state: initial.into_iter().flatten().collect_vec(),
            width,
            height,
        }
    }

    pub fn pos_to_index(&self, pos: (usize, usize)) -> usize {
        let (x, y) = pos;
        y * self.width + x
    }
    pub fn index_to_pos(&self, idx: usize) -> (usize, usize) {
        let x = idx % self.width;
        let y = idx / self.width;
        (x, y)
    }

    pub fn at(&self, pos: (usize, usize)) -> &T {
        &self.state[self.pos_to_index(pos)]
    }
    pub fn at_isize(&self, pos: (isize, isize)) -> &T {
        self.get_isize(pos).unwrap()
    }

    pub fn get_neighbor_position(
        &self,
        pos: (usize, usize),
        cardinal: Cardinal,
    ) -> Option<(usize, usize)> {
        match cardinal {
            Cardinal::North => {
                if pos.1 >= 1 {
                    Some((pos.0, pos.1 - 1))
                } else {
                    None
                }
            }
            Cardinal::South => {
                if pos.1 < (self.height - 1) {
                    Some((pos.0, pos.1 + 1))
                } else {
                    None
                }
            }
            Cardinal::West => {
                if pos.0 >= 1 {
                    Some((pos.0 - 1, pos.1))
                } else {
                    None
                }
            }
            Cardinal::East => {
                if pos.0 < (self.width - 1) {
                    Some((pos.0 + 1, pos.1))
                } else {
                    None
                }
            }
        }
    }

    pub fn get_neighbors_along_cardinal(
        &self,
        pos: (usize, usize),
        cardinal: Cardinal,
    ) -> Vec<(usize, usize)> {
        let mut neighbors = Vec::new();
        let mut neighbor = self.get_neighbor_position(pos, cardinal);
        while let Some(n) = neighbor {
            neighbors.push(n);
            neighbor = self.get_neighbor_position(n, cardinal);
        }
        neighbors
    }

    pub fn get_neighbor_at(&self, pos: (usize, usize), cardinal: Cardinal) -> Option<&T> {
        if let Some(neighbor) = match cardinal {
            Cardinal::North => {
                if pos.1 >= 1 {
                    Some((pos.0, pos.1 - 1))
                } else {
                    None
                }
            }
            Cardinal::South => Some((pos.0, pos.1 + 1)),
            Cardinal::East => {
                if pos.0 >= 1 {
                    Some((pos.0 - 1, pos.1))
                } else {
                    None
                }
            }
            Cardinal::West => Some((pos.0 - 1, pos.1)),
        } {
            self.get(neighbor)
        } else {
            None
        }
    }

    // Allow for negatives, which simplifies movement logic
    pub fn get_isize(&self, pos: (isize, isize)) -> Option<&T> {
        if pos.0 >= 0 && pos.1 >= 0 {
            let (x, y) = (pos.0 as usize, pos.1 as usize);
            if x < self.width && y < self.height {
                Some(&self.state[self.pos_to_index((x, y))])
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn get(&self, pos: (usize, usize)) -> Option<&T> {
        if pos.0 < self.width && pos.1 < self.height {
            Some(&self.state[self.pos_to_index(pos)])
        } else {
            None
        }
    }

    pub fn get_subgrid(&self, pos: (usize, usize), width: usize, height: usize) -> Grid<T> {
        let mut subgrid = Grid::new_empty(width, height);
        for y in 0..height {
            for x in 0..width {
                let pos = (pos.0 + x, pos.1 + y);
                if let Some(value) = self.get(pos) {
                    subgrid.state[y * width + x] = value.clone();
                }
            }
        }
        subgrid
    }

    pub fn mut_at(&mut self, pos: (usize, usize)) -> &mut T {
        let index = self.pos_to_index(pos);
        &mut self.state[index]
    }
    pub fn cardinal_neighbor_positions(&self, pos: (usize, usize)) -> Vec<(usize, usize)> {
        let (x, y) = (pos.0 as i64, pos.1 as i64);
        let neighbors = [(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)];
        neighbors
            .iter()
            .filter(|(x, y)| {
                (0..self.width as i64).contains(x) && (0..self.height as i64).contains(y)
            })
            .map(|(x, y)| (*x as usize, *y as usize))
            .collect_vec()
    }

    pub fn cardinal_neighbors(&self, pos: (usize, usize)) -> impl Iterator<Item = &T> {
        let (x, y) = pos;
        let delta = -1..=1;
        delta
            .clone()
            .cartesian_product(delta)
            .filter_map(move |(dx, dy): (i64, i64)| {
                if (dx == 0 && dy == 0) || (dx * dy).abs() == 1 {
                    None
                } else {
                    let new_x = x as i64 + dx;
                    let new_y = y as i64 + dy;
                    if new_x >= 0
                        && new_x < self.width as i64
                        && new_y >= 0
                        && new_y < self.height as i64
                    {
                        Some(self.at((new_x as usize, new_y as usize)))
                    } else {
                        None
                    }
                }
            })
    }

    pub fn neighbors(&self, pos: (usize, usize)) -> impl Iterator<Item = &T> {
        let (x, y) = pos;
        let delta = -1..=1;
        delta
            .clone()
            .cartesian_product(delta)
            .filter_map(move |(dx, dy)| {
                if dx == 0 && dy == 0 {
                    None
                } else {
                    let new_x = x as i64 + dx;
                    let new_y = y as i64 + dy;
                    if new_x >= 0
                        && new_x < self.width as i64
                        && new_y >= 0
                        && new_y < self.height as i64
                    {
                        Some(self.at((new_x as usize, new_y as usize)))
                    } else {
                        None
                    }
                }
            })
    }
    pub fn neighbor_positions(&self, pos: (usize, usize)) -> Vec<(usize, usize)> {
        let (x, y) = pos;
        let delta = -1..=1;
        delta
            .clone()
            .cartesian_product(delta)
            .filter_map(move |(dx, dy)| {
                if dx == 0 && dy == 0 {
                    None
                } else {
                    let new_x = x as i64 + dx;
                    let new_y = y as i64 + dy;
                    if new_x >= 0
                        && new_x < self.width as i64
                        && new_y >= 0
                        && new_y < self.height as i64
                    {
                        Some((new_x as usize, new_y as usize))
                    } else {
                        None
                    }
                }
            })
            .collect()
    }

    pub fn horizontal_neighbors(
        &self,
        pos: (usize, usize),
    ) -> (
        impl Iterator<Item = &T> + Clone,
        impl Iterator<Item = &T> + Clone,
    ) {
        let (x0, y0) = pos;
        let left_half = (0..x0).rev().map(move |x| self.at((x, y0)));
        let right_half = ((x0 + 1)..self.width).map(move |x| self.at((x, y0)));
        (left_half, right_half)
    }

    pub fn vertical_neighbors(
        &self,
        pos: (usize, usize),
    ) -> (
        impl Iterator<Item = &T> + Clone,
        impl Iterator<Item = &T> + Clone,
    ) {
        let (x0, y0) = pos;
        let top_half = (0..y0).rev().map(move |y| self.at((x0, y)));
        let bottom_half = ((y0 + 1)..self.height).map(move |y| self.at((x0, y)));
        (top_half, bottom_half)
    }

    pub fn neighbors_along_directions(
        &self,
        pos: (usize, usize),
    ) -> Vec<impl Iterator<Item = (usize, usize)>> {
        let (x, y) = pos;
        let (width, height) = (self.width, self.height);
        let delta = -1..=1;
        delta
            .clone()
            .cartesian_product(delta)
            .filter_map(move |(dx, dy)| {
                if dx == 0 && dy == 0 {
                    None
                } else {
                    let nums = 1..std::cmp::max(width, height);

                    // Have to make an in scope copy to appease borrow checker
                    let width = width;
                    let height = height;

                    Some(
                        nums.filter_map(move |d| {
                            let new_x = x as i64 + dx * d as i64;
                            let new_y = y as i64 + dy * d as i64;

                            if new_x >= 0 && new_y >= 0 {
                                Some((new_x as usize, new_y as usize))
                            } else {
                                None
                            }
                        })
                        .take_while(move |(new_x, new_y)| *new_x < width && *new_y < height),
                    )
                }
            })
            .collect_vec()
    }

    pub fn to_2d(&self) -> Vec<Vec<&T>> {
        self.state
            .chunks(self.width)
            .map(|chunk| chunk.iter().collect_vec())
            .collect_vec()
    }

    pub fn positions(&self) -> impl Iterator<Item = (usize, usize)> {
        (0..self.height)
            .cartesian_product(0..self.width)
            .map(|(y, x)| (x, y))
    }

    pub fn row(&self, y: usize) -> impl Iterator<Item = &T> {
        self.state[y * self.width..(y + 1) * self.width].iter()
    }

    pub fn col(&self, x: usize) -> impl Iterator<Item = &T> {
        self.state
            .iter()
            .skip(x)
            .step_by(self.width)
            .take(self.height)
    }

    pub fn from_lines(lines: &[&str], transformer: &dyn Fn(char) -> T) -> Grid<T> {
        let height = lines.len();
        let width = lines[0].len();
        let state = lines
            .iter()
            .flat_map(|line| line.chars().map(transformer))
            .collect_vec();
        Grid {
            state,
            width,
            height,
        }
    }
}

impl<T: Display> Display for Grid<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f)?;
        for chunk in self.state.chunks(self.width) {
            for t in chunk {
                write!(f, "{t}")?;
            }
            writeln!(f)?;
        }
        writeln!(f)
    }
}

#[derive(Default)]
pub struct Graph<V, E> {
    pub edges: HashMap<V, HashMap<V, E>>,
}

impl<V, E> Graph<V, E>
where
    V: Eq + std::hash::Hash + Clone + Debug,
    E: Eq + std::hash::Hash + Clone + Debug,
{
    pub fn new() -> Graph<V, E> {
        Graph {
            edges: HashMap::new(),
        }
    }

    pub fn add_edge(&mut self, from: V, to: V, edge: E) {
        let from_entry = self.edges.entry(from);
        from_entry.or_default().insert(to, edge);
    }

    pub fn get(&self, vertex: &V) -> Option<&HashMap<V, E>> {
        self.edges.get(vertex)
    }

    pub fn debug(&self) {
        for (vertex, edges) in &self.edges {
            debug!("{:?}: {:?}", vertex, edges);
        }
    }

    pub fn debug_connections(&self) {
        for (vertex, edges) in &self.edges {
            debug!(
                "{:?}: {:?}",
                vertex,
                edges.iter().map(|(_, v)| v).collect_vec()
            );
        }
    }

    pub fn all_vertices(&self) -> impl Iterator<Item = &V> {
        self.edges.keys()
    }

    pub fn all_distances<'a>(&'a self, start: &'a V) -> HashMap<&'a V, usize> {
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();
        queue.push_back((start, 0));

        let mut distances = HashMap::new();

        while let Some((vertex, depth)) = queue.pop_front() {
            if visited.contains(vertex) {
                continue;
            }
            visited.insert(vertex);

            distances.insert(vertex, depth);

            if let Some(edges) = self.edges.get(vertex) {
                for neighbor in edges.keys() {
                    queue.push_back((neighbor, depth + 1));
                }
            }
        }
        distances
    }

    pub fn bfs(&self, start: V, end: V) -> Option<usize> {
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();
        queue.push_back((start, 0));

        while let Some((vertex, depth)) = queue.pop_front() {
            if visited.contains(&vertex) {
                continue;
            }
            visited.insert(vertex.clone());

            if vertex == end {
                return Some(depth);
            }

            if let Some(edges) = self.edges.get(&vertex) {
                for neighbor in edges.keys() {
                    queue.push_back((neighbor.clone(), depth + 1));
                }
            }
        }
        info!("Never found end!");
        None
    }
}
