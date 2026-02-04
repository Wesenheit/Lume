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
        #[arg(short, long,default_value = "100")]
        ms: u64
    },
    Cpu{
        #[arg(short, long,default_value = "100")]
        ms: u64
    }
}

