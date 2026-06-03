use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(name = "youki-nix")]
#[command(about = "[Experimental] Nix-aware build and deployment layer for youki")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    Build {
        #[arg(default_value = ".")]
        flake: String,
    },
    Bundle {
        #[arg(default_value = ".")]
        flake: String,
        #[arg(long, default_value = "./dist/bundle")]
        out: PathBuf,
    },
    Run {
        #[arg(default_value = ".")]
        flake: String,
    },
    Deploy {
        host: String,
    },
    Rollback {
        app: String,
        host: String,
        #[arg(long)]
        generation: Option<u64>,
    },
    InspectConfig {
        #[arg(long, default_value = "youki-nix.toml")]
        manifest: PathBuf,
    },
}
