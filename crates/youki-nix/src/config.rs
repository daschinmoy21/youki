use std::fs;
use std::path::Path;

use anyhow::{Context, Ok, Result};
use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AppConfig {
    pub name: String,
    pub build: BuildConfig,
    pub container: ContainerConfig,
    #[serde(default)]
    pub deploy: DeployConfig,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BuildConfig {
    pub flake: String,
    pub target: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ContainerConfig {
    pub command: Vec<String>,

    #[serde(default = "default_workdir")]
    pub workdir: String,

    #[serde(default)]
    pub env: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct DeployConfig {
    #[serde(default)]
    pub hosts: Vec<String>,
}

fn default_workdir() -> String {
    "/app".to_string()
}

impl AppConfig {
    pub fn load(path: &Path) -> Result<Self> {
        let raw = fs::read_to_string(path)
            .with_context(|| format!("Failed to read manifest {}", path.display()))?;

        let cfg: AppConfig = toml::from_str(&raw)
            .with_context(|| format!("Failed to parse manifest!{}", path.display()))?;
        cfg.validate()?;
        Ok(cfg)
    }

    pub fn validate(&self) -> Result<()> {
        if self.name.trim().is_empty() {
            anyhow::bail!("Config name cannot be empty!!");
        }
        if self.build.flake.trim().is_empty() {
            anyhow::bail!("config field `build.flake` cannot be empty");
        }
        if self.build.target.trim().is_empty() {
            anyhow::bail!("config field `build.target` cannot be empty");
        }
        if self.container.command.is_empty() {
            anyhow::bail!("config field `container.command` cannot be empty");
        }
        Ok(())
    }
}
