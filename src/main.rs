use maze_gen::{generate, solve, Maze};
pub fn main() {
    const WIDTH: usize = 64;
    const HEIGHT: usize = 64;
    let mut maze = Maze::new(WIDTH, HEIGHT);
    generate(&mut maze);
    solve(&mut maze, (0, 0), (WIDTH - 1, HEIGHT - 1));
    maze.print(Some("out.txt"), false).unwrap();
}
