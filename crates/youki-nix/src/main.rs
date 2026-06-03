mod cli;
mod config;
mod error;
use anyhow::Result;
use clap::Parser;
use cli::{Cli, Commands};

use crate::config::AppConfig;

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Build { flake } => {
            println!("build requested for {flake}");
        }
        Commands::Bundle { flake, out } => {
            println!("bundle requested for {flake} -> {}", out.display());
        }
        Commands::Run { flake } => {
            println!("run requested for {flake}");
        }
        Commands::Deploy { host } => {
            println!("deploy requested for {host}");
        }
        Commands::Rollback {
            app,
            host,
            generation,
        } => {
            println!("rollback requested: app={app}, host={host}, generation={generation:?}");
        }
        Commands::InspectConfig { manifest } => {
            let cfg = AppConfig::load(&manifest)?;
            println!("{}", serde_json::to_string_pretty(&cfg)?);
        }
    }

    Ok(())
}
