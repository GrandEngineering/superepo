use std::path::PathBuf;
mod config;
use clap::{Parser, Subcommand};
use config::{Config, ConfigTomlBin};
use std::process::Command;

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
        bin: Option<String>,
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

fn execute_command(cmd: &str) -> std::process::Output {
    if cfg!(windows) {
        Command::new("cmd")
            .args(["/C", cmd])
            .output()
            .expect("Failed to execute command")
    } else {
        Command::new("sh")
            .args(["-c", cmd])
            .output()
            .expect("Failed to execute command")
    }
}

fn spawn_command(cmd: &str) -> std::process::Child {
    if cfg!(windows) {
        Command::new("cmd")
            .args(["/C", cmd])
            .spawn()
            .expect("Failed to execute command")
    } else {
        Command::new("sh")
            .args(["-c", cmd])
            .spawn()
            .expect("Failed to execute command")
    }
}

fn main() {
    let cli = Cli::parse();
    let config: Config = Config::new();
    match &cli.command {
        Some(Commands::Run { bin, release }) => {
            let run_rel: &Option<String> = &config.config_toml.monorepo.opt_run;
            let run: &Option<String> = &config.config_toml.monorepo.run;

            if let Some(bin_name) = bin {
                if let Some(bins) = &config.config_toml.monorepo.bins {
                    if let Some(binary) = bins.iter().find(|b| b.name == *bin_name) {
                        let cmd = if *release && binary.opt_run.is_some() {
                            binary.opt_run.as_ref().unwrap()
                        } else {
                            &binary.run
                        };

                        println!("Running command for {}: {}", bin_name, cmd);
                        let output = execute_command(cmd);

                        if !output.status.success() {
                            eprintln!(
                                "Command failed with error: {}",
                                String::from_utf8_lossy(&output.stderr)
                            );
                        }
                    } else {
                        println!("Binary '{}' not found in configuration", bin_name);
                    }
                } else {
                    println!("No binaries configured in .superepo.toml");
                }
            } else {
                let cmd = if *release && run_rel.is_some() {
                    run_rel.as_ref().unwrap()
                } else if let Some(run_cmd) = run {
                    run_cmd
                } else {
                    println!("No run command configured in .superepo.toml");
                    return;
                };

                println!("Running default command: {}", cmd);
                let output = execute_command(cmd);

                if !output.status.success() {
                    eprintln!(
                        "Command failed with error: {}",
                        String::from_utf8_lossy(&output.stderr)
                    );
                }
            }
        }
        Some(Commands::Build { bin, release }) => {
            if let Some(bin_name) = bin {
                if let Some(bins) = &config.config_toml.monorepo.bins {
                    if let Some(binary) = bins.iter().find(|b| b.name == *bin_name) {
                        let cmd = if *release {
                            binary.opt_build.as_ref().unwrap_or(&binary.build)
                        } else {
                            &binary.build
                        };

                        println!(
                            "Building binary '{}' in {} mode",
                            bin_name,
                            if *release { "release" } else { "debug" }
                        );
                        let output = spawn_command(cmd)
                            .wait()
                            .expect("Failed to wait for command");

                        if !output.success() {
                            eprintln!("Build failed for binary '{}'", bin_name);
                            std::process::exit(1);
                        }
                        return;
                    }
                }

                if let Some(libs) = &config.config_toml.monorepo.libs {
                    if let Some(lib) = libs.iter().find(|l| l.name == *bin_name) {
                        let cmd = if *release {
                            lib.opt_build.as_ref().unwrap_or(&lib.build)
                        } else {
                            &lib.build
                        };

                        println!(
                            "Building library '{}' in {} mode",
                            bin_name,
                            if *release { "release" } else { "debug" }
                        );
                        let output = spawn_command(cmd)
                            .wait()
                            .expect("Failed to wait for command");

                        if !output.success() {
                            eprintln!("Build failed for library '{}'", bin_name);
                            std::process::exit(1);
                        }
                        return;
                    }
                }

                println!("'{}' not found in configuration", bin_name);
            } else {
                let cmd = if *release {
                    config
                        .config_toml
                        .monorepo
                        .opt_build
                        .as_ref()
                        .unwrap_or(&config.config_toml.monorepo.build)
                } else {
                    &config.config_toml.monorepo.build
                };

                println!(
                    "Building entire project in {} mode",
                    if *release { "release" } else { "debug" }
                );
                let output = spawn_command(cmd)
                    .wait()
                    .expect("Failed to wait for command");

                if !output.success() {
                    eprintln!("Build failed");
                    std::process::exit(1);
                }
            }
        }
        _ => {
            println!("No command provided");
        }
    }
}
