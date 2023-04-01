use rand::{
    distributions::{Distribution, WeightedIndex},
    Rng,
};
use std::{
    collections::HashSet,
    fs::File,
    io::{Result, Write},
};
use svg::node::element::path::Data;
use svg::node::element::{Path, Rectangle};
use svg::Document;

#[derive(Copy, Clone)]
pub enum Direction {
    Out,
    In,
    Left,
    Right,
}

#[derive(Copy, Clone, Debug)]
pub struct Cell {
    pub outward: bool,
    pub inward: bool,
    pub left: bool,
    pub right: bool,
    pub in_solution: bool,
}

impl Cell {
    pub fn new() -> Cell {
        Cell {
            outward: true,
            inward: true,
            left: true,
            right: true,
            in_solution: false,
        }
    }
    pub fn reset(&mut self) {
        self.outward = true;
        self.inward = true;
        self.left = true;
        self.right = true;
        self.in_solution = false;
    }
    pub fn add_to_solution(&mut self) {
        self.in_solution = true;
    }
}

pub struct CircMaze {
    pub rings: usize,
    pub spokes: usize,
    pub cells: Vec<Cell>,
    pub split_frequency: usize,
}

impl CircMaze {
    pub fn new(rings: usize, spokes: usize, split_frequency: usize) -> CircMaze {
        let mut cells = Vec::new();
        for _ in 0..rings {
            for _ in 0..spokes {
                cells.push(Cell::new());
            }
        }
        CircMaze {
            rings,
            spokes,
            cells,
            split_frequency,
        }
    }
    pub fn reset(&mut self) {
        for cell in self.cells.iter_mut() {
            cell.reset();
        }
    }
    pub fn get(&self, ring: usize, spoke: usize) -> Cell {
        self.cells[ring * self.spokes + spoke]
    }
    pub fn set(&mut self, ring: usize, spoke: usize, direction: &Direction, value: bool) {
        match direction {
            Direction::Out => self.cells[ring * self.spokes + spoke].outward = value,
            Direction::In => self.cells[ring * self.spokes + spoke].inward = value,
            Direction::Left => self.cells[ring * self.spokes + spoke].left = value,
            Direction::Right => self.cells[ring * self.spokes + spoke].right = value,
        }
    }
    pub fn add_cell_to_solution(&mut self, ring: usize, spoke: usize) {
        self.cells[ring * self.spokes + spoke].add_to_solution();
    }
    pub fn is_wall_at_dir(&self, r: usize, dir: &Direction) -> bool {
        match dir {
            Direction::In => r == self.rings - 1,
            Direction::Out => r == 0,
            _ => false,
        }
    }
    pub fn open_at_dir(&mut self, r: usize, s: usize, dir: &Direction) {
        match dir {
            Direction::In => {
                self.set(r, s, dir, false);
                self.set(r - 1, s, &Direction::Out, false)
            }
            Direction::Out => {
                self.set(r, s, dir, false);
                self.set(r + 1, s, &Direction::In, false)
            }
            Direction::Left => {
                self.set(r, s, dir, false);
                self.set(r, s - 1, &Direction::Right, false)
            }
            Direction::Right => {
                self.set(r, s, dir, false);
                self.set(r, s + 1, &Direction::Left, false)
            }
        }
    }
}

pub fn generate(rings: usize, spokes: usize, split_frequency: usize) -> CircMaze {
    let mut maze = CircMaze::new(rings, spokes, split_frequency);
    let mut stack: Vec<(usize, usize)> = Vec::new();
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    let mut r: usize = 0;
    let mut s: usize = 0;
    let mut dir: Direction;
    let dirs: Vec<Direction> = vec![
        Direction::In,
        Direction::Out,
        Direction::Left,
        Direction::Right,
    ];
    let mut rng = rand::thread_rng();
    visited.insert((r, s));
    stack.push((r, s));
    loop {
        (r, s) = stack.pop().unwrap();
        let mut unvisited_neighbors: Vec<Direction> = Vec::new();
        for dir in dirs.iter() {
            if !maze.is_wall_at_dir(r, dir) {
                let (nr, ns) = match dir {
                    Direction::In => (r - 1, s),
                    Direction::Out => (r + 1, s),
                    Direction::Left => {
                        if s > 0 {
                            (r, s - 1)
                        } else {
                            (r, maze.spokes - 1)
                        }
                    }
                    Direction::Right => {
                        if s < maze.spokes - 1 {
                            (r, s + 1)
                        } else {
                            (r, 0)
                        }
                    }
                };
                if !visited.contains(&(nr, ns)) {
                    unvisited_neighbors.push(*dir);
                }
            }
        }
        if unvisited_neighbors.len() > 0 {
            stack.push((r, s));
            let dir_index = rng.gen_range(0..unvisited_neighbors.len());
            dir = unvisited_neighbors[dir_index];
            // let dist = WeightedIndex::new(unvisited_neighbors.iter().map(|item| item.1)).unwrap();
            // dir = unvisited_neighbors[dist.sample(&mut rng)].0;
            maze.open_at_dir(r, s, &dir);
            let (nr, ns) = match dir {
                Direction::In => (r - 1, s),
                Direction::Out => (r + 1, s),
                Direction::Left => {
                    if s > 0 {
                        (r, s - 1)
                    } else {
                        (r, maze.spokes - 1)
                    }
                }
                Direction::Right => {
                    if s < maze.spokes - 1 {
                        (r, s + 1)
                    } else {
                        (r, 0)
                    }
                }
            };
            visited.insert((nr, ns));
            stack.push((nr, ns));
        }
        if stack.len() == 0 {
            break;
        }
    }
    maze
}
