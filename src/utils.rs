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
    pub fn open(&mut self, dir: Direction) {
        match dir {
            Direction::Up => self.up = false,
            Direction::Down => self.down = false,
            Direction::Left => self.left = false,
            Direction::Right => self.right = false,
        }
    }
    pub fn is_open_at_dir(&self, dir: Direction) -> bool {
        match dir {
            Direction::Up => !self.up,
            Direction::Down => !self.down,
            Direction::Left => !self.left,
            Direction::Right => !self.right,
        }
    }
    pub fn has_wall_at_dir(&self, dir: Direction) -> bool {
        !self.is_open_at_dir(dir)
    }
    pub fn add_to_solution(&mut self) {
        self.in_solution = true;
    }
}
