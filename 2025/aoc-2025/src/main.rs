use std::fs;

use clap::{Parser, Subcommand};

use anyhow::Result;

use crate::gen_06::generate;

mod commons;
mod day_01;
mod day_02;
mod day_03;
mod day_04;
mod day_05;
mod day_06;
mod day_07;
mod day_09;
mod gen_06;

/// A simple CLI application
#[derive(Parser)]
#[command(name = "Aoc 2025")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Exec {
        #[arg(required = true, help = "The path to the file")]
        file_path: String,
    },
    Generate {
        #[arg(required = true, help = "How many rows")]
        n_rows: usize,
        #[arg(required = true, help = "How many cols")]
        n_cols: usize,
        #[arg(required = true, help = "Max number of digits within col")]
        max_col_size: usize,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Exec { file_path } => {
            println!("{}", day_06::p2(&fs::read_to_string(file_path)?))
        }
        Commands::Generate {
            n_rows,
            n_cols,
            max_col_size,
        } => {
            println!(
                "{}",
                generate(n_rows as i32, n_cols as i32, max_col_size as i32)
            );
        }
    }
    Ok(())
}
