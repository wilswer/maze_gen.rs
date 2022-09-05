use std::{
    collections::HashSet,
    fs::File,
    io::{Result, Write},
};

use rand::Rng;
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
}

impl Cell {
    pub fn new() -> Cell {
        Cell {
            up: true,
            down: true,
            left: true,
            right: true,
        }
    }
    pub fn reset(&mut self) {
        self.up = true;
        self.down = true;
        self.left = true;
        self.right = true;
    }
    pub fn open(&mut self, dir: Direction) {
        match dir {
            Direction::Up => self.up = false,
            Direction::Down => self.down = false,
            Direction::Left => self.left = false,
            Direction::Right => self.right = false,
        }
    }
    pub fn is_open_at_dir(&mut self, dir: Direction) -> bool {
        match dir {
            Direction::Up => !self.up,
            Direction::Down => !self.down,
            Direction::Left => !self.left,
            Direction::Right => !self.right,
        }
    }
    pub fn has_wall_at_dir(&mut self, dir: Direction) -> bool {
        match dir {
            Direction::Up => self.up,
            Direction::Down => self.down,
            Direction::Left => self.left,
            Direction::Right => self.right,
        }
    }
}

pub struct Maze {
    pub width: usize,
    pub height: usize,
    pub cells: Vec<Cell>,
}

impl Maze {
    pub fn new(width: usize, height: usize) -> Maze {
        Maze {
            width,
            height,
            cells: vec![Cell::new(); width * height],
        }
    }

    pub fn get(&self, x: usize, y: usize) -> Cell {
        self.cells[y * self.width + x]
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

    // pub fn set(&mut self, x: usize, y: usize, value: bool) {
    //     self.cells[y * self.width + x].visited = value;
    // }

    // pub fn reset(&self) {
    //     for cell in &self.cells {
    //         cell.reset();
    //     }
    // }

    // pub fn open_all(&mut self) {
    //     for cell in &self.cells {
    //         cell.up = false;
    //         cell.down = false;
    //     }
    // }

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
            Direction::Up => self.set(x, y, dir, false),
            Direction::Down => self.set(x, y, dir, false),
            Direction::Left => self.set(x, y, dir, false),
            Direction::Right => self.set(x, y, dir, false),
        }
        match dir {
            Direction::Up => self.set(x, y - 1, &Direction::Down, false),
            Direction::Down => self.set(x, y + 1, &Direction::Up, false),
            Direction::Left => self.set(x - 1, y, &Direction::Right, false),
            Direction::Right => self.set(x + 1, y, &Direction::Left, false),
        }
    }

    pub fn print(&self, path: Option<&str>) -> Result<()> {
        let mut maze_str = String::new();
        let mut horizontal_wall_str: String = String::new();
        horizontal_wall_str += "┌   ┬";
        horizontal_wall_str += &"---┬".repeat(self.width - 2);
        horizontal_wall_str += "---┐";
        println!("{}", horizontal_wall_str);
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
                    vertical_wall_str += "    ";
                } else {
                    vertical_wall_str += "   │";
                }
            }
            vertical_wall_str += "   │";
            println!("{}", vertical_wall_str);
            maze_str += &(vertical_wall_str + "\n");
            if self.is_open_at_dir(self.width - 1, y, &Direction::Down) {
                horizontal_wall_str += "   ┤";
            } else {
                horizontal_wall_str += "---┤";
            }
            println!("{}", horizontal_wall_str);
            maze_str += &(horizontal_wall_str + "\n");
        }
        let mut vertical_wall_str: String = String::new();
        vertical_wall_str += "│";
        for x in 0..self.width - 1 {
            if self.is_open_at_dir(x, self.height - 1, &Direction::Right) {
                vertical_wall_str += "    ";
            } else {
                vertical_wall_str += "   │";
            }
        }
        vertical_wall_str += "   │";
        let mut horizontal_wall_str: String = String::new();
        horizontal_wall_str += "└";
        horizontal_wall_str += &"---┴".repeat(self.width - 1);
        horizontal_wall_str += "   ┘";
        println!("{}", vertical_wall_str);
        println!("{}", horizontal_wall_str);
        maze_str += &(vertical_wall_str + "\n");
        maze_str += &(horizontal_wall_str + "\n");
        if !path.is_none() {
            let mut file = File::create(path.unwrap())?;
            file.write_all(maze_str.as_bytes())?;
            return Ok(());
        }
        Ok(())
    }
}

pub fn generate(maze: &mut Maze) {
    maze.reset();
    let mut stack: Vec<(usize, usize)> = Vec::new();
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    let mut x: usize = 0;
    let mut y: usize = 0;
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
    stack.push((x, y));
    loop {
        (x, y) = stack.pop().unwrap();
        let mut unvisited_neighbors: Vec<Direction> = Vec::new();
        for dir in dirs.iter() {
            if !maze.is_wall_at_dir(x, y, dir) {
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
}
