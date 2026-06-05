mod bundle;
mod cli;
mod config;
mod error;
mod nix;
use std::path::Path;

use anyhow::Result;
use clap::Parser;
use cli::{Cli, Commands};

use crate::{config::AppConfig, nix::build};

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Build { flake: _flake } => {
            let cfg = AppConfig::load(std::path::Path::new("youki-nix-test.toml"))?;
            let result = build(&cfg)?;
            println!("{}", serde_json::to_string_pretty(&result)?);
        }
        Commands::Bundle { flake, out } => {
            println!("bundle requested for {flake} -> {}", out.display());
            let cfg = AppConfig::load(Path::new("youki-nix-test.toml"))?;
            let build = nix::build(&cfg)?;
            let closure = nix::closure(&build)?;
            let generated = bundle::generate_bundle(&cfg, &build, &closure, &out)?;
            println!("{}", serde_json::to_string_pretty(&generated)?);
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
