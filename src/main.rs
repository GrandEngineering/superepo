use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Optional name to operate on

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Runs the project
    Run {
        /// lists test values
        #[arg(value_name = "bin")]
        bin: String,
    },
    /// Builds the project
    Build {
        /// lists test values
        #[arg(value_name = "bin")]
        bin: Option<String>,
    },
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Run { bin }) => {
            println!("Running binary: {bin}");
        }
        Some(Commands::Build { bin }) => {
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
}
