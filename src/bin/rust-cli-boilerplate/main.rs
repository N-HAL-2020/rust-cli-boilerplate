use anyhow::Result;
use clap::Parser;
use std::process;

use rust_cli_boilerplate::error::NotImplementedError;

mod config;
use crate::config::Args;

fn run() -> Result<()> {
    let args = Args::parse();
    println!("{:?}", args);

    Err(NotImplementedError::new("run"))?
}

fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {}", e);
        process::exit(1);
    }
}
