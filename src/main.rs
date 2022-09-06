use maze_gen::{generate, solve, Maze};
pub fn main() {
    const WIDTH: usize = 20;
    const HEIGHT: usize = 15;
    let mut maze = Maze::new(WIDTH, WIDTH);
    generate(&mut maze);
    maze.print(Some("out.txt"), true).unwrap();
    solve(&mut maze, (0, 0), (WIDTH - 1, HEIGHT - 1));
}
