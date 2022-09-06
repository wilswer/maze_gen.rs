use maze_gen::{generate, Maze};
pub fn main() {
    let mut maze = Maze::new(20, 15);
    generate(&mut maze);
    maze.print(Some("out.txt"), true).unwrap();
}
