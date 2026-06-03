use anyhow::{Context, Ok, Result};
use serde::{Deserialize, Serialize};
use std::process::Command;

use crate::config::AppConfig;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildResult {
    pub flake: String,
    pub target: String,
    pub installable: String,
    pub out_path: String,
}

pub fn build(cfg: &AppConfig) -> Result<BuildResult> {
    let installable = format!("{}#{}", cfg.build.flake, cfg.build.target);
    let output = Command::new("nix")
        .arg("build")
        .arg(&installable)
        .arg("--print-out-paths")
        .output()
        .with_context(|| format!("failed to execute nix build for {installable}"))?;

    if !output.status.success() {
        anyhow::bail!(
            "nix build failed for {} \n{}",
            installable,
            String::from_utf8_lossy(&output.stderr)
        );
    }
    let stdout =
        String::from_utf8(output.stdout).context("nix build output was not valid UTF-8")?;
    let out_path = stdout
        .lines()
        .find(|line| !line.trim().is_empty())
        .map(|line| line.trim().to_string())
        .context("nix build --print-out-paths returned no output path")?;

    Ok(BuildResult {
        flake: cfg.build.flake.clone(),
        target: cfg.build.target.clone(),
        installable,
        out_path,
    })
}
