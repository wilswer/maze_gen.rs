use clap::Parser;
use maze_gen::{generate, solve, Maze};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Width of the maze
    #[clap(long, short, default_value_t = 16)]
    x: usize,
    /// Height of the maze
    #[clap(long, short, default_value_t = 16)]
    y: usize,
    /// Bias towards horizontal or vertical walls (0.0 - 1.0), 0.5 is equal, 0.0 is vertical, 1.0 is horizontal
    #[clap(long, short, default_value_t = 0.5)]
    bias: f64,
    /// bias towards horizontal or vertical walls (0.0 - 1.0)
    #[clap(short, long, default_value_t = 0.0)]
    length_bias: f64,
    /// Print the maze to stdout
    #[clap(long, action, default_value_t = false)]
    print: bool,
    /// Thickness of the walls in SVG
    #[clap(long, short, action, default_value_t = 0.1)]
    wall_thickness: f64,
    /// Solution path transparency in SVG
    #[clap(long, short, action, default_value_t = 0.2)]
    transparency: f64,
    /// Output file, without extension
    #[clap(long, short, default_value = "maze")]
    output: String,
    /// Solve the maze
    #[clap(long, short, action, default_value_t = false)]
    solve: bool,
}
pub fn main() {
    let args = Cli::parse();
    let mut maze = Maze::new(args.x, args.y);
    generate(&mut maze, args.bias, args.length_bias);
    maze.open_start_and_end();
    if args.solve {
        solve(&mut maze, (0, 0), (args.x - 1, args.y - 1));
    }
    maze.draw(
        Some(args.output.as_str()),
        args.wall_thickness,
        args.transparency,
    )
    .unwrap();
    maze.print(Some(format!("{}.txt", args.output).as_str()), args.print)
        .unwrap();
}
