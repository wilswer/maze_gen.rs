use clap::Parser;

use maze_gen::{circ_maze, cli::Cli, cli::Commands, rect_maze};

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
            maze.print(Some(format!("{output}.txt").as_str()), *print)
                .unwrap();
        }
        Some(Commands::Circ {
            rings,
            bar,
            frequency,
            inner_radius,
            bias,
            length_bias,
            wall_thickness,
            transparency,
            output,
            solve,
        }) => {
            let mut maze = circ_maze::generate(*rings, *bar, *frequency, *bias, *length_bias);
            // let mut maze = circ_maze::CircMaze::new(*rings, *bar, *freq);
            maze.open_start_and_end();
            if *solve {
                circ_maze::solve(&mut maze, (0, 0), (*rings - 1, *bar / 2));
            }
            maze.draw(
                Some(output.as_str()),
                *wall_thickness / 10.0,
                *transparency,
                *inner_radius,
            )
            .unwrap();
        }
        None => {
            println!("No subcommand was used, try --help");
        }
    }
}
