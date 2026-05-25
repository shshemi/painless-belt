use std::{fs, path::PathBuf};

use anyhow::anyhow;

use crate::AppResult;

pub fn home_dir() -> AppResult<PathBuf> {
    let home = dirs::home_dir().ok_or(anyhow!("Home directory not found"))?;
    let dir = home.join(".painless-belt");
    mkdir(dir)
}

pub fn profiles_dir() -> AppResult<PathBuf> {
    let home = home_dir()?;
    let dir = home.join("profiles");
    mkdir(dir)
}

pub fn profile(name: &str) -> AppResult<PathBuf> {
    let profiles = profiles_dir()?;
    let profile = profiles.join(name);
    if profile.is_file() {
        Ok(profile)
    } else {
        Err(anyhow!("Invalid profile name"))
    }
}

fn mkdir(dir: PathBuf) -> AppResult<PathBuf> {
    if !dir.is_dir() {
        if dir.exists() {
            fs::remove_file(&dir)?;
        }
        fs::create_dir_all(&dir)?;
    }
    Ok(dir)
}
