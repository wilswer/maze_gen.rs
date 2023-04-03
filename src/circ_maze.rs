use rand::{
    distributions::{Distribution, WeightedIndex},
    Rng,
};
use std::{collections::HashSet, io::Result};
use svg::node::element::path::Data;
use svg::node::element::Path;
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
    pub fn set(&mut self, direction: Direction, value: bool) {
        match direction {
            Direction::Out => self.outward = value,
            Direction::In => self.inward = value,
            Direction::Left => self.left = value,
            Direction::Right => self.right = value,
        }
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
    pub fn is_open_at_dir(&self, r: usize, s: usize, dir: &Direction) -> bool {
        match dir {
            Direction::In => !self.get(r, s).inward,
            Direction::Out => !self.get(r, s).outward,
            Direction::Left => !self.get(r, s).left,
            Direction::Right => !self.get(r, s).right,
        }
    }
    pub fn open_at_dir(&mut self, r: usize, s: usize, dir: &Direction) {
        match dir {
            Direction::In => {
                self.set(r, s, dir, false);
                self.set(r + 1, s, &Direction::Out, false)
            }
            Direction::Out => {
                self.set(r, s, dir, false);
                self.set(r - 1, s, &Direction::In, false)
            }
            Direction::Left => {
                self.set(r, s, dir, false);
                {
                    if s > 0 {
                        self.set(r, s - 1, &Direction::Right, false)
                    } else {
                        self.set(r, self.spokes - 1, &Direction::Right, false)
                    }
                }
            }
            Direction::Right => {
                self.set(r, s, dir, false);
                {
                    if s < self.spokes - 1 {
                        self.set(r, s + 1, &Direction::Left, false)
                    } else {
                        self.set(r, 0, &Direction::Left, false)
                    }
                }
            }
        }
    }
    pub fn draw(&self, path: Option<&str>, line_thickness: f64, transparency: f64) -> Result<()> {
        let inner_radius = 3.0;
        let margin = 5;
        let translate = self.rings as f64 + (margin as f64) + inner_radius;
        let mut arc_paths = Vec::new();
        // let mut line_paths = Vec::new();
        for r in 0..self.rings {
            for s in 0..self.spokes {
                let r_float = r as f64;
                let radius = self.rings as f64 + inner_radius;
                let theta = 2.0 * std::f64::consts::PI * (s as f64) / (self.spokes as f64);
                let x = (radius - r_float) * theta.cos();
                let y = (radius - r_float) * theta.sin();
                let mut data = Data::new().move_to((x + translate, y + translate));
                // Arcs
                let end_theta = theta + 2.0 * std::f64::consts::PI / (self.spokes as f64);
                let end_x = (radius - r_float) * end_theta.cos();
                let end_y = (radius - r_float) * end_theta.sin();
                if !self.is_open_at_dir(r, s, &Direction::Out) {
                    data = data.elliptical_arc_to((
                        radius,
                        radius,
                        0.0,
                        0,
                        1,
                        end_x + translate,
                        end_y + translate,
                    ));
                } else {
                    data = data.move_to((end_x + translate, end_y + translate));
                }
                if !self.is_open_at_dir(r, s, &Direction::Left) {
                    let inner_x = (radius - r_float - 1.0) * end_theta.cos();
                    let inner_y = (radius - r_float - 1.0) * end_theta.sin();
                    data = data.line_to((inner_x + translate, inner_y + translate));
                }
                let path = Path::new()
                    .set("fill", "none")
                    .set("stroke", "black")
                    .set("stroke-width", line_thickness)
                    .set("d", data);
                arc_paths.push(path);
            }
        }

        let mut document = Document::new().set("viewBox", (0, 0, 2.0 * translate, 2.0 * translate));
        for arc in arc_paths {
            document = document.add(arc);
        }
        svg::save(format!("{}.svg", path.unwrap_or("maze.svg")), &document).unwrap();
        Ok(())
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
                    Direction::In => (r + 1, s),
                    Direction::Out => (r - 1, s),
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
            //println!("r: {}, s: {}, dir: {:?}", r, s, dir);
            maze.open_at_dir(r, s, &dir);
            let (nr, ns) = match dir {
                Direction::In => (r + 1, s),
                Direction::Out => (r - 1, s),
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
