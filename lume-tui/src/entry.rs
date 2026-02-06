use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "Lume")]
pub struct Cli {
    #[arg(short, long,default_value = "100")]
    pub ms: u64,

    #[arg(short, long)]
    pub light: bool,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Random {
        #[arg(short, long)]
        size: usize,
    },
    Cpu {
        #[arg(short, long)]
        simple: bool,
    }
}
