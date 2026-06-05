use std::{
    fs,
    path::{Path, PathBuf},
};

use anyhow::{Context, Ok, Result};
use oci_spec::runtime::{
    LinuxBuilder, MountBuilder, ProcessBuilder, RootBuilder, SpecBuilder,
};
use serde::Serialize;

use crate::{config::AppConfig, nix::{BuildResult, ClosureInfo}};

#[derive(Debug, Clone, Serialize)]
pub struct GeneratedBundle {
    pub bundle_dir: PathBuf,
    pub rootfs_dir: PathBuf,
    pub config_path: PathBuf,
}

fn resolve_command(cfg: &AppConfig, build: &BuildResult) -> Vec<String> {
    let out = build.out_path.display().to_string();
    cfg.container
        .command
        .iter()
        .map(|arg| arg.replace("{out}", &out))
        .collect()
}

fn create_workdir(rootfs_dir: &Path, workdir: &str) -> Result<()> {
    let relative = workdir.trim_start_matches('/');
    if relative.is_empty() {
        return Ok(());
    }
    fs::create_dir_all(rootfs_dir.join(relative))
        .with_context(|| format!("Failed to create workdir {workdir} in rootfs"))
}

pub fn generate_bundle(
    cfg: &AppConfig,
    build: &BuildResult,
    closure: &ClosureInfo,
    out_dir: &Path,
) -> Result<GeneratedBundle> {
    let bundle_dir = out_dir.to_path_buf();
    let rootfs_dir = bundle_dir.join("rootfs");
    let config_path = bundle_dir.join("config.json");

    fs::create_dir_all(&rootfs_dir)
        .with_context(|| format!("failed to create {}", rootfs_dir.display()))?;
    fs::create_dir_all(rootfs_dir.join("tmp"))
        .with_context(|| format!("failed to create tmp dir in {}", rootfs_dir.display()))?;
    create_workdir(&rootfs_dir, &cfg.container.workdir)?;

    let process = ProcessBuilder::default()
        .args(resolve_command(cfg, build))
        .cwd(cfg.container.workdir.clone())
        .env(cfg.container.env.clone())
        .terminal(false)
        .build()
        .context("failed to build OCI process")?;

    let root = RootBuilder::default()
        .path(PathBuf::from("rootfs"))
        .readonly(true)
        .build()
        .context("failed to build OCI root")?;

    let store_mount = MountBuilder::default()
        .destination(PathBuf::from("/nix/store"))
        .source(PathBuf::from("/nix/store"))
        .typ("bind")
        .options(vec!["rbind".to_string(), "ro".to_string()])
        .build()
        .context("failed to build /nix/store mount")?;

    let tmp_mount = MountBuilder::default()
        .destination(PathBuf::from("/tmp"))
        .source(PathBuf::from("tmpfs"))
        .typ("tmpfs")
        .options(vec![
            "nosuid".to_string(),
            "nodev".to_string(),
            "mode=1777".to_string(),
        ])
        .build()
        .context("failed to build /tmp mount")?;

    let linux = LinuxBuilder::default()
        .build()
        .context("failed to build OCI linux section")?;

    let spec = SpecBuilder::default()
        .version("1.0.2")
        .process(process)
        .root(root)
        .mounts(vec![store_mount, tmp_mount])
        .linux(linux)
        .hostname(cfg.name.clone())
        .annotations(std::collections::HashMap::from([
            ("youki-nix.flake".to_string(), cfg.build.flake.clone()),
            ("youki-nix.target".to_string(), cfg.build.target.clone()),
            (
                "youki-nix.out-path".to_string(),
                build.out_path.display().to_string(),
            ),
            (
                "youki-nix.closure-size".to_string(),
                closure.paths.len().to_string(),
            ),
        ]))
        .build()
        .context("failed to build OCI spec")?;

    spec.save(&config_path)
        .with_context(|| format!("failed to save {}", config_path.display()))?;

    Ok(GeneratedBundle {
        bundle_dir,
        rootfs_dir,
        config_path,
    })
}
