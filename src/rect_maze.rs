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
    Up,
    Down,
    Left,
    Right,
}

#[derive(Copy, Clone, Debug)]
pub struct Cell {
    pub up: bool,
    pub down: bool,
    pub left: bool,
    pub right: bool,
    pub in_solution: bool,
}

impl Cell {
    pub fn new() -> Cell {
        Cell {
            up: true,
            down: true,
            left: true,
            right: true,
            in_solution: false,
        }
    }
    pub fn reset(&mut self) {
        self.up = true;
        self.down = true;
        self.left = true;
        self.right = true;
        self.in_solution = false;
    }
    pub fn add_to_solution(&mut self) {
        self.in_solution = true;
    }
}

pub struct RectMaze {
    pub width: usize,
    pub height: usize,
    pub cells: Vec<Cell>,
}

impl RectMaze {
    pub fn new(width: usize, height: usize) -> RectMaze {
        RectMaze {
            width,
            height,
            cells: vec![Cell::new(); width * height],
        }
    }
    pub fn open_start_and_end(&mut self) {
        self.set(0, 0, &Direction::Up, false);
        self.set(self.width - 1, self.height - 1, &Direction::Down, false);
    }
    pub fn get(&self, x: usize, y: usize) -> Cell {
        self.cells[y * self.width + x]
    }
    pub fn add_cell_to_solution(&mut self, x: usize, y: usize) {
        self.cells[y * self.width + x].add_to_solution();
    }
    pub fn set(&mut self, x: usize, y: usize, dir: &Direction, val: bool) {
        match dir {
            Direction::Up => self.cells[y * self.width + x].up = val,
            Direction::Down => self.cells[y * self.width + x].down = val,
            Direction::Left => self.cells[y * self.width + x].left = val,
            Direction::Right => self.cells[y * self.width + x].right = val,
        }
    }
    pub fn reset(&mut self) {
        for cell in self.cells.iter_mut() {
            cell.reset();
        }
    }

    pub fn open_all(&mut self) {
        for cell in self.cells.iter_mut() {
            cell.up = false;
            cell.down = false;
            cell.left = false;
            cell.right = false;
        }
    }

    pub fn is_wall_at_dir(&self, x: usize, y: usize, dir: &Direction) -> bool {
        match dir {
            Direction::Up => y == 0,
            Direction::Down => y == self.height - 1,
            Direction::Left => x == 0,
            Direction::Right => x == self.width - 1,
        }
    }

    pub fn is_open_at_dir(&self, x: usize, y: usize, dir: &Direction) -> bool {
        match dir {
            Direction::Up => !self.get(x, y).up,
            Direction::Down => !self.get(x, y).down,
            Direction::Left => !self.get(x, y).left,
            Direction::Right => !self.get(x, y).right,
        }
    }

    pub fn open_at_dir(&mut self, x: usize, y: usize, dir: &Direction) {
        match dir {
            Direction::Up => {
                self.set(x, y, dir, false);
                self.set(x, y - 1, &Direction::Down, false)
            }
            Direction::Down => {
                self.set(x, y, dir, false);
                self.set(x, y + 1, &Direction::Up, false)
            }
            Direction::Left => {
                self.set(x, y, dir, false);
                self.set(x - 1, y, &Direction::Right, false)
            }
            Direction::Right => {
                self.set(x, y, dir, false);
                self.set(x + 1, y, &Direction::Left, false)
            }
        }
    }

    pub fn print(&self, path: Option<&str>, do_print: bool) -> Result<()> {
        let mut maze_str = String::new();
        let mut horizontal_wall_str: String = String::new();
        horizontal_wall_str += "┌   ┬";
        horizontal_wall_str += &"---┬".repeat(self.width - 2);
        horizontal_wall_str += "---┐";
        maze_str += &(horizontal_wall_str + "\n");
        for y in 0..self.height - 1 {
            let mut horizontal_wall_str: String = String::new();
            horizontal_wall_str += "├";
            let mut vertical_wall_str: String = String::new();
            vertical_wall_str += "│";
            for x in 0..self.width - 1 {
                if self.is_open_at_dir(x, y, &Direction::Down) {
                    horizontal_wall_str += "   ┼";
                } else {
                    horizontal_wall_str += "---┼";
                }
                if self.is_open_at_dir(x, y, &Direction::Right) {
                    if self.get(x, y).in_solution == true {
                        vertical_wall_str += " x  ";
                    } else {
                        vertical_wall_str += "    ";
                    }
                } else {
                    if self.get(x, y).in_solution == true {
                        vertical_wall_str += " x │";
                    } else {
                        vertical_wall_str += "   │";
                    }
                }
            }
            if self.get(self.width - 1, y).in_solution == true {
                vertical_wall_str += " x │";
            } else {
                vertical_wall_str += "   │";
            }
            maze_str += &(vertical_wall_str + "\n");
            if self.is_open_at_dir(self.width - 1, y, &Direction::Down) {
                horizontal_wall_str += "   ┤";
            } else {
                horizontal_wall_str += "---┤";
            }
            maze_str += &(horizontal_wall_str + "\n");
        }
        let mut vertical_wall_str: String = String::new();
        vertical_wall_str += "│";
        for x in 0..self.width - 1 {
            if self.is_open_at_dir(x, self.height - 1, &Direction::Right) {
                if self.get(x, self.height - 1).in_solution == true {
                    vertical_wall_str += " x  ";
                } else {
                    vertical_wall_str += "    ";
                }
            } else {
                if self.get(x, self.height - 1).in_solution == true {
                    vertical_wall_str += " x │";
                } else {
                    vertical_wall_str += "   │";
                }
            }
        }
        if self.get(self.width - 1, self.height - 1).in_solution == true {
            vertical_wall_str += " x │";
        } else {
            vertical_wall_str += "   │";
        }
        let mut horizontal_wall_str: String = String::new();
        horizontal_wall_str += "└";
        horizontal_wall_str += &"---┴".repeat(self.width - 1);
        horizontal_wall_str += "   ┘";
        maze_str += &(vertical_wall_str + "\n");
        maze_str += &(horizontal_wall_str + "\n");
        if do_print {
            print!("{}", maze_str);
        }
        if !path.is_none() {
            let mut file = File::create(path.unwrap())?;
            file.write_all(maze_str.as_bytes())?;
            return Ok(());
        }
        Ok(())
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
        if solution_marks.len() > 0 {
            for rect in solution_marks {
                document = document.add(rect);
            }
            for path in wall_path_copy {
                document = document.add(path);
            }
            svg::save(format!("sol_{}.svg", path.unwrap_or("maze")), &document).unwrap();
            if ok_pdf {
                let svg =
                    std::fs::read_to_string(format!("sol_{}.svg", path.unwrap_or("maze"))).unwrap();
                let pdf = svg2pdf::convert_str(&svg, svg2pdf::Options::default()).unwrap();
                std::fs::write(format!("sol_{}.pdf", path.unwrap_or("maze")), pdf).unwrap();
            }
        }
        Ok(())
    }
}

pub fn generate(width: usize, height: usize, bias: f64, length_bias: f64) -> RectMaze {
    let mut maze = RectMaze::new(width, height);
    let mut stack: Vec<(usize, usize)> = Vec::new();
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    let mut x: usize = 0;
    let mut y: usize = 0;
    let row_weight: f64 = 1.0 - bias;
    let col_weight: f64 = bias;
    let mut dir: Direction;
    let dirs: Vec<Direction> = vec![
        Direction::Up,
        Direction::Down,
        Direction::Left,
        Direction::Right,
    ];
    let mut rng = rand::thread_rng();
    visited.insert((x, y));
    stack.push((x, y));
    loop {
        (x, y) = stack.pop().unwrap();
        let mut unvisited_neighbors: Vec<(Direction, f64)> = Vec::new();
        for dir in dirs.iter() {
            if !maze.is_wall_at_dir(x, y, dir) {
                let (nx, ny, weight) = match dir {
                    Direction::Up => (x, y - 1, col_weight + length_bias),
                    Direction::Down => (x, y + 1, col_weight),
                    Direction::Left => (x - 1, y, row_weight + length_bias),
                    Direction::Right => (x + 1, y, row_weight),
                };
                if !visited.contains(&(nx, ny)) {
                    unvisited_neighbors.push((*dir, weight));
                }
            }
        }
        if unvisited_neighbors.len() > 0 {
            stack.push((x, y));
            // dir_index = rng.gen_range(0..unvisited_neighbors.len());
            // dir = unvisited_neighbors[dir_index];
            let dist = WeightedIndex::new(unvisited_neighbors.iter().map(|item| item.1)).unwrap();
            dir = unvisited_neighbors[dist.sample(&mut rng)].0;
            maze.open_at_dir(x, y, &dir);
            let (nx, ny) = match dir {
                Direction::Up => (x, y - 1),
                Direction::Down => (x, y + 1),
                Direction::Left => (x - 1, y),
                Direction::Right => (x + 1, y),
            };
            visited.insert((nx, ny));
            stack.push((nx, ny));
        }
        if stack.len() == 0 {
            break;
        }
    }
    maze
}

pub fn solve(maze: &mut RectMaze, start: (usize, usize), stop: (usize, usize)) {
    let mut stack: Vec<(usize, usize)> = Vec::new();
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    let mut visited_list: Vec<(usize, usize)> = Vec::new();
    let mut x: usize = start.0;
    let mut y: usize = start.1;
    let mut dir: Direction;
    let mut dir_index: usize;
    let dirs: Vec<Direction> = vec![
        Direction::Up,
        Direction::Down,
        Direction::Left,
        Direction::Right,
    ];
    let mut rng = rand::thread_rng();
    visited.insert((x, y));
    visited_list.push((x, y));
    stack.push((x, y));
    loop {
        (x, y) = stack.pop().unwrap();
        let mut unvisited_neighbors: Vec<Direction> = Vec::new();
        for dir in dirs.iter() {
            if !maze.is_wall_at_dir(x, y, dir) && maze.is_open_at_dir(x, y, dir) {
                let (nx, ny) = match dir {
                    Direction::Up => (x, y - 1),
                    Direction::Down => (x, y + 1),
                    Direction::Left => (x - 1, y),
                    Direction::Right => (x + 1, y),
                };
                if !visited.contains(&(nx, ny)) {
                    unvisited_neighbors.push(*dir);
                }
            }
        }
        if unvisited_neighbors.len() > 0 {
            stack.push((x, y));
            dir_index = rng.gen_range(0..unvisited_neighbors.len());
            dir = unvisited_neighbors[dir_index];
            let (nx, ny) = match dir {
                Direction::Up => (x, y - 1),
                Direction::Down => (x, y + 1),
                Direction::Left => (x - 1, y),
                Direction::Right => (x + 1, y),
            };
            visited.insert((nx, ny));
            visited_list.push((nx, ny));
            stack.push((nx, ny));
        }
        if visited.contains(&stop) {
            break;
        }
    }
    let mut solution_list: Vec<(usize, usize)> = Vec::new();
    let mut current: (usize, usize) = visited_list.pop().unwrap();
    solution_list.push(current);
    maze.add_cell_to_solution(current.0, current.1);
    for v in visited_list.iter().rev() {
        current = *v;
        let mut neighbor_set: HashSet<(usize, usize)> =
            HashSet::from([(current.0 + 1, current.1), (current.0, current.1 + 1)]);
        if current.0 > 0 {
            neighbor_set.insert((current.0 - 1, current.1));
        }
        if current.1 > 0 {
            neighbor_set.insert((current.0, current.1 - 1));
        }
        if neighbor_set.contains(solution_list.last().unwrap()) {
            solution_list.push(current);
            maze.add_cell_to_solution(current.0, current.1);
        }
    }
}
