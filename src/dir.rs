use std::{fs, path::PathBuf};

use anyhow::anyhow;

use crate::AppResult;

pub fn home_dir() -> AppResult<PathBuf> {
    let home = dirs::home_dir().ok_or(anyhow!("Home directory not found"))?;
    let dir = home.join(".painless-belt");
    mkdir(dir)
}

pub fn config() -> AppResult<PathBuf> {
    Ok(home_dir()?.join("config"))
}

pub fn profiles_dir() -> AppResult<PathBuf> {
    let home = home_dir()?;
    let dir = home.join("profiles");
    mkdir(dir)
}

pub fn profile_path(name: &str) -> AppResult<PathBuf> {
    let mut path = profiles_dir()?.join(name);
    path.as_mut_os_string().push(".pb");
    Ok(path)
}

pub fn profile(name: &str) -> AppResult<PathBuf> {
    let path = profile_path(name)?;
    if path.is_file() {
        Ok(path)
    } else {
        Err(anyhow!("Invalid profile name"))
    }
}

pub fn remove_profile(name: &str) -> AppResult<PathBuf> {
    let path = profile_path(name)?;
    if !path.is_file() {
        return Err(anyhow!("Profile not found: {name}"));
    }
    fs::remove_file(&path)?;
    Ok(path)
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
