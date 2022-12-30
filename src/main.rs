use clap::Parser;
use maze_gen::{generate, solve, Maze};

#[derive(Parser)]
struct Cli {
    #[clap(long)]
    width: usize,
    #[clap(long)]
    height: usize,
    #[clap(long, short, action)]
    print: bool,
    #[clap(long, short, action)]
    solve: bool,
}
pub fn main() {
    let args = Cli::parse();
    let mut maze = Maze::new(args.width, args.height);
    generate(&mut maze);
    if args.solve {
        solve(&mut maze, (0, 0), (args.width - 1, args.height - 1));
    }
    maze.print(Some("out.txt"), args.print).unwrap();
}
