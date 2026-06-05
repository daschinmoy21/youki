use anyhow::{Context, Ok, Result};
use serde::{Deserialize, Serialize};
use std::{path::PathBuf, process::Command};

use crate::config::AppConfig;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildResult {
    pub flake: String,
    pub target: String,
    pub installable: String,
    pub out_path: PathBuf,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClosureInfo {
    pub out_path: PathBuf,
    pub paths: Vec<PathBuf>,
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
    let out_path = PathBuf::from(
        stdout
            .lines()
            .find(|line| !line.trim().is_empty())
            .map(|line| line.trim())
            .context("nix build --print-out-paths returned no output path")?,
    );

    Ok(BuildResult {
        flake: cfg.build.flake.clone(),
        target: cfg.build.target.clone(),
        installable,
        out_path,
    })
}

pub fn closure(build: &BuildResult) -> Result<ClosureInfo> {
    let output = Command::new("nix")
        .arg("path-info")
        .arg("--recursive")
        .arg(&build.out_path)
        .output()
        .with_context(|| format!("failed to query closure for {}", build.out_path.display()))?;

    if !output.status.success() {
        anyhow::bail!(
            "nix path-info --recursive failed for {}:\n{}",
            build.out_path.display(),
            String::from_utf8_lossy(&output.stderr)
        );
    }

    let stdout = String::from_utf8(output.stdout).context("closure output was not valid UTF-8")?;
    let paths = stdout
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| PathBuf::from(line.trim()))
        .collect::<Vec<_>>();

    Ok(ClosureInfo {
        out_path: build.out_path.clone(),
        paths,
    })
}
