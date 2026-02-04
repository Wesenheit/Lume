use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "Lume")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Random {
        #[arg(short, long)]
        size: usize,
    },
    Cpu, 
}

