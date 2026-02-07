mod utils;
mod entry;

use std::io;

use clap::Parser;
use entry::{Cli,Commands};
use lume_core::random::CM5;
use lume_core::cpu::Cpu;
use lume_core::core::{Matrix,Renderable};
use utils::{draw_cli,Pallete};

fn main() -> io::Result<()> {
    let cli = Cli::parse();
    let (mut matrix,mut pattern):(Matrix,Box<dyn Renderable>) = match &cli.command {
        Commands::Random { size } => {
            let pattern = Box::new(CM5);
            let matrix = Matrix::random(*size);
            (matrix,pattern)
        }
        Commands::Cpu {simple,reduce}=> {
            let pattern = Box::new(Cpu::new(*simple,*reduce));
            let size = pattern.count();
            let matrix = Matrix::zero(size);
            (matrix,pattern)
        }
    };
    let ms = cli.ms;
    let theme = match cli.light {
        true => Pallete::Light,
        false => Pallete::Dark,
    };
    draw_cli(&mut matrix,pattern.as_mut(),ms,theme)
}
