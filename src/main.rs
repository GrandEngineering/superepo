use std::path::PathBuf;
mod config;
use clap::{Parser, Subcommand};
use config::Config;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Runs the project
    Run {
        /// The binary to run
        #[arg(value_name = "bin")]
        bin: String,
        #[arg(short, long)]
        release: bool,
    },
    /// Builds the project
    Build {
        /// The binary or library to build
        #[arg(value_name = "bin")]
        bin: Option<String>,
        #[arg(short, long)]
        release: bool,
    },
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Run { bin, release }) => {
            println!("Running binary: {bin}");
        }
        Some(Commands::Build { bin, release }) => {
            if let Some(bin) = bin {
                println!("Building binary: {bin}");
            } else {
                println!("Building some binary");
            }
        }
        _ => {
            println!("No command provided");
        }
    }
    let config: Config = Config::new();
    println!("{:#?}", config)
}
