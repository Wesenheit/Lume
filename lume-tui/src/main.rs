mod entry;
mod utils;

use std::io;

use clap::Parser;
use entry::{Cli, Commands};
use lume_core::combined::CombinedPattern;
use lume_core::core::{Matrix, MatrixConfig, Renderable};
use lume_core::cpu::Cpu;
use lume_core::random::CM5;
use utils::{Pallete, draw_cli};

fn main() -> io::Result<()> {
    let cli = Cli::parse();
    let (mut matrix, mut pattern): (Matrix, Box<dyn Renderable>) = match &cli.command {
        Commands::Random { size, step } => {
            let pattern = Box::new(CM5 { sliding: *step });
            let config = MatrixConfig {
                size: *size,
                reduce: cli.reduce_u8,
            };
            let matrix = Matrix::random(config);
            (matrix, pattern)
        }
        Commands::Cpu {
            simple,
            reduce,
            step,
        } => {
            let pattern = Box::new(Cpu::new(*simple, *reduce, *step));
            let size = pattern.count();
            let config = MatrixConfig {
                size: size,
                reduce: cli.reduce_u8,
            };
            let matrix = Matrix::zero(config);
            (matrix, pattern)
        }
        Commands::Combined { path } => {
            let pattern = Box::new(CombinedPattern::from_yaml(path));
            let size = pattern.get_size();
            let config = MatrixConfig {
                size: size,
                reduce: cli.reduce_u8,
            };
            let matrix = Matrix::random(config);
            (matrix, pattern)
        }
    };
    let ms = cli.ms;
    let theme = match cli.light {
        true => Pallete::Light,
        false => Pallete::Dark,
    };
    draw_cli(&mut matrix, pattern.as_mut(), ms, theme)
}
