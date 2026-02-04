mod utils;
mod entry;

use std::io;

use clap::Parser;
use entry::{Cli,Commands};
use lume_core::random::CM5;
use lume_core::core::Matrix;
use utils::draw_cli;

fn main() -> io::Result<()> {
    let cli = Cli::parse();
    match &cli.command {
        Commands::Random { size } => {
            let pattern = CM5;
            let mut matrix = Matrix::random(*size);
            return draw_cli(&mut matrix,pattern);
        }
    }
}
