use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Generate a rectangular maze
    Rect {
        /// Width of the maze
        #[clap(long, short, default_value_t = 16)]
        x: usize,
        /// Height of the maze
        #[clap(long, short, default_value_t = 16)]
        y: usize,
        /// Bias towards horizontal or vertical walls (0.0 - 1.0), 0.5 is equal, 0.0 is vertical, 1.0 is horizontal
        #[clap(long, default_value_t = 0.5)]
        bias: f64,
        /// Bias towards longer solutions (0.0 - 1.0)
        #[clap(long, short, default_value_t = 0.0)]
        length_bias: f64,
        /// Print the maze to stdout
        #[clap(long, short, action, default_value_t = false)]
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
    },
    /// Generate a circular maze
    Circ {
        /// Rings of the maze
        #[clap(long, short, default_value_t = 4)]
        rings: usize,
        /// Bars/spokes of the maze
        #[clap(long, short, default_value_t = 8)]
        bar: usize,
        /// Base number of spokes of the innermost ring; outer rings subdivide
        /// (double) every `frequency` rings so cells keep a roughly constant
        /// width. Set to 0 to disable subdivision.
        #[clap(long, short, default_value_t = 2)]
        frequency: usize,
        #[clap(long, short, default_value_t = 0.5)]
        /// Size of the inner radius
        inner_radius: f64,
        /// Bias towards angular or radial walls (0.0 - 1.0), 0.5 is equal, 0.0 is angular, 1.0 is
        /// radial
        #[clap(long, default_value_t = 0.5)]
        bias: f64,
        /// Bias towards longer solutions (0.0 - 1.0)
        #[clap(long, short, default_value_t = 0.0)]
        length_bias: f64,
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
    },
}
