// src/main.rs
/*
 * Main executable for DisasterrecoverySystems
 */

use clap::Parser;
use disasterrecoverysystems::{Result, run};

#[derive(Parser)]
#[command(version, about = DisasterrecoverySystems - A Rust implementation)]
struct Cli {
    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    run(args.verbose)
}
