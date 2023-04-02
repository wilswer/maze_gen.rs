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
        let cell_size = 10;
        let margin = 5;
        let mut wall_paths = Vec::new();
        let mut solution_marks = Vec::new();
        for x in 0..self.width {
            for y in 0..self.height {
                let mut data =
                    Data::new().move_to((x * cell_size + margin, y * cell_size + margin));
                if self.is_open_at_dir(x, y, &Direction::Up) {
                    data = data.move_by(((cell_size as f64) + line_thickness / 2.0, 0));
                } else {
                    data = data.line_by(((cell_size as f64) + line_thickness / 2.0, 0));
                }
                if self.is_open_at_dir(x, y, &Direction::Right) {
                    data = data.move_by((0, (cell_size as f64) + line_thickness / 2.0));
                } else {
                    data = data.line_by((0, (cell_size as f64) + line_thickness / 2.0));
                }
                if y == self.height - 1 && !self.is_open_at_dir(x, y, &Direction::Down) {
                    data = data.line_by((-((cell_size as f64) + line_thickness / 2.0), 0));
                } else {
                    data = data.move_by((-((cell_size as f64) + line_thickness / 2.0), 0));
                }
                if x == 0 && !self.is_open_at_dir(x, y, &Direction::Left) {
                    data = data.line_by((0, -((cell_size as f64) + line_thickness / 2.0)));
                }
                let path = Path::new()
                    .set("fill", "none")
                    .set("stroke", "black")
                    .set("stroke-width", line_thickness)
                    .set("d", data);
                wall_paths.push(path);
                if self.get(x, y).in_solution {
                    let solution_rect = Rectangle::new()
                        .set("x", x * cell_size + margin)
                        .set("y", y * cell_size + margin)
                        .set("width", cell_size)
                        .set("height", cell_size)
                        .set("fill", "red")
                        .set("fill-opacity", transparency);
                    solution_marks.push(solution_rect);
                }
            }
        }

        let mut document = Document::new().set(
            "viewBox",
            (
                0,
                0,
                cell_size * self.width + 2 * margin,
                cell_size * self.height + 2 * margin,
            ),
        );
        let wall_path_copy = wall_paths.clone();
        for path in wall_paths {
            document = document.add(path);
        }
        svg::save(format!("{}.svg", path.unwrap_or("maze.svg")), &document).unwrap();
        let svg = std::fs::read_to_string(format!("{}.svg", path.unwrap_or("maze"))).unwrap();
        let pdf = svg2pdf::convert_str(&svg, svg2pdf::Options::default());
        let ok_pdf = match pdf {
            Ok(pdf) => {
                std::fs::write(format!("{}.pdf", path.unwrap_or("maze")), pdf).unwrap();
                true
            }
            Err(e) => {
                println!("Error: {}, could not produce PDF", e);
                false
            }
        };

        let mut document = Document::new().set(
            "viewBox",
            (
                0,
                0,
                cell_size * self.width + 2 * margin,
                cell_size * self.height + 2 * margin,
            ),
        );
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
