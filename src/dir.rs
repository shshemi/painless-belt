use std::{fs, path::PathBuf};

use anyhow::anyhow;

use crate::AppResult;

const PROFILES_URL: &str =
    "https://raw.githubusercontent.com/shshemi/painless-belt/master/profiles";

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

pub fn pull_profile(name: &str) -> AppResult<PathBuf> {
    let url = format!("{PROFILES_URL}/{name}.pb");
    let body = ureq::get(&url).call()?.into_body().read_to_string()?;
    let dest = profile_path(name)?;
    fs::write(&dest, body)?;
    Ok(dest)
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
