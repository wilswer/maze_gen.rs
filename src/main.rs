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
    /// Bias towards horizontal or vertical walls (0.0 - 1.0), 0.5 is equal, 0.0 is vertical, 1.0 is horizontal
    #[clap(long, short, default_value_t = 0.5)]
    bias: f64,
    /// bias towards horizontal or vertical walls (0.0 - 1.0)
    #[clap(short, long, default_value_t = 0.0)]
    length_bias: f64,
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
    generate(&mut maze, args.bias, args.length_bias);
    if args.solve {
        solve(&mut maze, (0, 0), (args.columns - 1, args.rows - 1));
    }
    maze.print(Some("out.txt"), !args.silent).unwrap();
}
