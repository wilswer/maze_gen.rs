use rand::distributions::{Distribution, WeightedIndex};
use std::f64::consts::PI;
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
    pub fn open_start_and_end(&mut self) {
        self.set(0, 0, &Direction::Out, false);
        self.set(self.rings - 1, self.spokes / 2, &Direction::In, false);
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
            Direction::Right => {
                self.set(r, s, dir, false);
                {
                    if s > 0 {
                        self.set(r, s - 1, &Direction::Left, false)
                    } else {
                        self.set(r, self.spokes - 1, &Direction::Left, false)
                    }
                }
            }
            Direction::Left => {
                self.set(r, s, dir, false);
                {
                    if s < self.spokes - 1 {
                        self.set(r, s + 1, &Direction::Right, false)
                    } else {
                        self.set(r, 0, &Direction::Right, false)
                    }
                }
            }
        }
    }
    pub fn draw(
        &self,
        path: Option<&str>,
        line_thickness: f64,
        transparency: f64,
        inner_radius: f64,
    ) -> Result<()> {
        let margin = 0.5;
        let translate = self.rings as f64 + margin + inner_radius;
        let mut arc_paths = Vec::new();
        let mut solution_marks = Vec::new();
        // let mut line_paths = Vec::new();
        for r in 0..self.rings {
            for s in 0..self.spokes {
                let r_float = r as f64;
                let radius = self.rings as f64 + inner_radius;
                let theta = 2.0 * PI * (s as f64) / (self.spokes as f64)
                    - PI / 2.0
                    - PI / (self.spokes as f64);
                let x = (radius - r_float) * theta.cos();
                let y = (radius - r_float) * theta.sin();
                let mut data = Data::new().move_to((x + translate, y + translate));
                // Arcs
                let end_theta = theta + 2.0 * PI / (self.spokes as f64);
                let end_x = (radius - r_float) * end_theta.cos();
                let end_y = (radius - r_float) * end_theta.sin();
                if !self.is_open_at_dir(r, s, &Direction::Out) {
                    data = data.elliptical_arc_to((
                        radius - r_float,
                        radius - r_float,
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
                if !self.is_open_at_dir(r, s, &Direction::In) && (r == self.rings - 1) {
                    let inner_x = inner_radius * theta.cos();
                    let inner_y = inner_radius * theta.sin();
                    data = data.move_to((inner_x + translate, inner_y + translate));
                    let inner_end_x = inner_radius * end_theta.cos();
                    let inner_end_y = inner_radius * end_theta.sin();
                    data = data.elliptical_arc_to((
                        inner_radius,
                        inner_radius,
                        0.0,
                        0,
                        1,
                        inner_end_x + translate,
                        inner_end_y + translate,
                    ));
                }
                let path = Path::new()
                    .set("fill", "none")
                    .set("stroke", "black")
                    .set("stroke-width", line_thickness)
                    .set("d", data);
                arc_paths.push(path);

                // Solution marks
                if self.get(r, s).in_solution {
                    let inner_x = (radius - r_float - 1.0) * end_theta.cos();
                    let inner_y = (radius - r_float - 1.0) * end_theta.sin();
                    let inner_x2 = (radius - r_float - 1.0) * theta.cos();
                    let inner_y2 = (radius - r_float - 1.0) * theta.sin();
                    let mut data = Data::new().move_to((x + translate, y + translate));
                    data = data.elliptical_arc_to((
                        radius - r_float,
                        radius - r_float,
                        0.0,
                        0,
                        1,
                        end_x + translate,
                        end_y + translate,
                    ));
                    data = data.line_to((inner_x + translate, inner_y + translate));
                    data = data.elliptical_arc_to((
                        radius - r_float - 1.0,
                        radius - r_float - 1.0,
                        0.0,
                        0,
                        0,
                        inner_x2 + translate,
                        inner_y2 + translate,
                    ));
                    data = data.close();
                    let solution_mark = Path::new()
                        .set("fill", "red")
                        .set("stroke", "none")
                        .set("opacity", transparency)
                        .set("d", data);
                    solution_marks.push(solution_mark);
                }
            }
        }

        let mut document = Document::new().set("viewBox", (0, 0, 2.0 * translate, 2.0 * translate));
        for arc in arc_paths {
            document = document.add(arc);
        }
        for mark in solution_marks {
            document = document.add(mark);
        }
        svg::save(format!("{}.svg", path.unwrap_or("maze.svg")), &document).unwrap();
        Ok(())
    }
}

pub fn generate(
    rings: usize,
    spokes: usize,
    split_frequency: usize,
    bias: f64,
    length_bias: f64,
) -> CircMaze {
    let mut maze = CircMaze::new(rings, spokes, split_frequency);
    let mut stack: Vec<(usize, usize)> = Vec::new();
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    let mut r: usize = 0;
    let mut s: usize = 0;
    let angle_weight: f64 = 1.0 - bias;
    let radial_weight: f64 = bias;
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
        let mut unvisited_neighbors: Vec<(Direction, f64)> = Vec::new();
        for dir in dirs.iter() {
            if !maze.is_wall_at_dir(r, dir) {
                let (nr, ns, weight) = match dir {
                    Direction::In => (r + 1, s, radial_weight),
                    Direction::Out => (r - 1, s, radial_weight + length_bias),
                    Direction::Right => {
                        if s > 0 {
                            (r, s - 1, angle_weight + length_bias)
                        } else {
                            (r, maze.spokes - 1, angle_weight + length_bias)
                        }
                    }
                    Direction::Left => {
                        if s < maze.spokes - 1 {
                            (r, s + 1, angle_weight + length_bias)
                        } else {
                            (r, 0, angle_weight + length_bias)
                        }
                    }
                };
                if !visited.contains(&(nr, ns)) {
                    unvisited_neighbors.push((*dir, weight));
                }
            }
        }
        if unvisited_neighbors.len() > 0 {
            stack.push((r, s));
            // let dir_index = rng.gen_range(0..unvisited_neighbors.len());
            // dir = unvisited_neighbors[dir_index];
            let dist = WeightedIndex::new(unvisited_neighbors.iter().map(|item| item.1)).unwrap();
            dir = unvisited_neighbors[dist.sample(&mut rng)].0;
            //println!("r: {}, s: {}, dir: {:?}", r, s, dir);
            maze.open_at_dir(r, s, &dir);
            let (nr, ns) = match dir {
                Direction::In => (r + 1, s),
                Direction::Out => (r - 1, s),
                Direction::Right => {
                    if s > 0 {
                        (r, s - 1)
                    } else {
                        (r, maze.spokes - 1)
                    }
                }
                Direction::Left => {
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
