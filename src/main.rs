use clap::{Parser, Subcommand};
use maze_gen::{
    circ_maze::{self, CircMaze},
    rect_maze,
};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Adds files to myapp
    Rect {
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
        #[clap(long, action, default_value_t = false)]
        solve: bool,
    },
    Circ {
        /// Rings of the maze
        #[clap(long, short, default_value_t = 4)]
        rings: usize,
        /// Spokes of the maze
        #[clap(long, short, default_value_t = 8)]
        spokes: usize,
        /// Split frequency, how often to split a spoke
        #[clap(long, short, default_value_t = 0)]
        freq: usize,
    },
}
pub fn main() {
    let args = Cli::parse();
    match &args.command {
        Some(Commands::Rect {
            x,
            y,
            bias,
            length_bias,
            print,
            wall_thickness,
            transparency,
            output,
            solve,
        }) => {
            let mut maze = rect_maze::generate(*x, *y, *bias, *length_bias);
            maze.open_start_and_end();
            if *solve {
                rect_maze::solve(&mut maze, (0, 0), (*x - 1, *y - 1));
            }
            maze.draw(Some(output.as_str()), *wall_thickness, *transparency)
                .unwrap();
            maze.print(Some(format!("{}.txt", output).as_str()), *print)
                .unwrap();
        }
        Some(Commands::Circ {
            rings,
            spokes,
            freq,
        }) => {
            let mut maze = circ_maze::generate(*rings, *spokes, *freq);
        }
        None => {
            println!("No subcommand was used, try --help");
        }
    }
}
