use std::{
    path::{Path, PathBuf},
    process::{Command, ExitStatus},
    time::{SystemTime, UNIX_EPOCH},
};

use anyhow::{Context, Result};

use crate::{
    bundle::GeneratedBundle,
    config::AppConfig,
};

fn container_id(app_name: &str) -> String {
    let ts = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    format!("{}{}", app_name, ts)
}

fn find_youki_bin() -> PathBuf {
    let exe = std::env::current_exe().unwrap_or_default();
    let crate_dir = exe.parent().unwrap_or(Path::new("."));
    let sibling = crate_dir.join("youki");
    if sibling.exists() {
        return sibling;
    }
    PathBuf::from("./youki")
}

pub fn run_bundle(cfg: &AppConfig, bundle: &GeneratedBundle) -> Result<ExitStatus> {
    let id = container_id(&cfg.name);
    let youki = find_youki_bin();
    let status = Command::new(&youki)
        .arg("run")
        .arg("-b")
        .arg(&bundle.bundle_dir)
        .arg(&id)
        .status()
        .with_context(|| {
            format!(
                "Failed to execute youki at {} for bundle {}",
                youki.display(),
                bundle.bundle_dir.display()
            )
        })?;
    Ok(status)
}
