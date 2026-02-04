mod utils;
mod entry;

use std::io;

use clap::Parser;
use entry::{Cli,Commands};
use lume_core::random::CM5;
use lume_core::cpu::Cpu;
use lume_core::core::{Matrix,Renderable};
use utils::draw_cli;

fn main() -> io::Result<()> {
    let cli = Cli::parse();
    let (mut matrix,mut pattern,ms):(Matrix,Box<dyn Renderable>,u64) = match &cli.command {
        Commands::Random { size,ms } => {
            let pattern = Box::new(CM5);
            let matrix = Matrix::random(*size);
            (matrix,pattern,*ms)
        }
        Commands::Cpu {ms} => {
            let pattern = Box::new(Cpu::new());
            let size = pattern.count();
            let matrix = Matrix::zero(size);
            (matrix,pattern,*ms)
        }
    };
    draw_cli(&mut matrix,pattern.as_mut(),ms)
}
