use clap::Parser;
use maze_gen::{generate, solve, Maze};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Width of the maze
    #[clap(long, short, default_value_t = 16)]
    columns: usize,
    /// Height of the maze
    #[clap(long, short, default_value_t = 16)]
    rows: usize,
    /// Don't print the maze to stdout
    #[clap(long, action, default_value_t = false)]
    silent: bool,
    /// Solve the maze
    #[clap(long, short, action, default_value_t = false)]
    solve: bool,
}
pub fn main() {
    let args = Cli::parse();
    let mut maze = Maze::new(args.columns, args.rows);
    generate(&mut maze);
    if args.solve {
        solve(&mut maze, (0, 0), (args.columns - 1, args.rows - 1));
    }
    maze.print(Some("out.txt"), !args.silent).unwrap();
}
