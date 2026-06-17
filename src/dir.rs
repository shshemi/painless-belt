use std::{fs, path::PathBuf};

use anyhow::anyhow;

use crate::AppResult;

pub fn home_dir() -> AppResult<PathBuf> {
    let home = dirs::home_dir()
        .ok_or_else(|| anyhow!("Could not determine your home directory (is $HOME set?)"))?;
    let dir = home.join(".painless-belt");
    mkdir(dir)
}

pub fn config_path() -> AppResult<PathBuf> {
    Ok(home_dir()?.join("config"))
}

pub fn profile_dir() -> AppResult<PathBuf> {
    let home = home_dir()?;
    let dir = home.join("profiles");
    mkdir(dir)
}

pub fn profile_path(name: &str) -> AppResult<PathBuf> {
    let mut path = profile_dir()?.join(name);
    path.as_mut_os_string().push(".pb");
    Ok(path)
}

pub fn remove_profile(name: &str) -> AppResult<PathBuf> {
    let path = profile_path(name)?;
    if !path.is_file() {
        return Err(anyhow!("Profile '{name}' not found at {}", path.display()));
    }
    fs::remove_file(&path)?;
    Ok(path)
}

pub fn list_profiles() -> AppResult<Vec<String>> {
    let mut names: Vec<String> = fs::read_dir(profile_dir()?)?
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .filter(|path| path.extension().is_some_and(|ext| ext == "pb"))
        .filter_map(|path| {
            path.file_stem()
                .map(|stem| stem.to_string_lossy().into_owned())
        })
        .collect();
    for builtin in ["default", "empty"] {
        if !names.iter().any(|name| name == builtin) {
            names.push(builtin.to_owned());
        }
    }
    names.sort();
    Ok(names)
}

pub fn copy_profile(name: &str, dst: &str) -> AppResult<()> {
    let src = profile_path(name)?;
    let dst = profile_path(dst)?;
    fs::copy(&src, &dst)?;
    Ok(())
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
