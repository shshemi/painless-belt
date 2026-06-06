use std::{fs, path::PathBuf};

use crate::{AppResult, dir};

const PROFILES_URL: &str =
    "https://raw.githubusercontent.com/shshemi/painless-belt/master/profiles";

pub fn pull_profile(name: &str, dst: &str) -> AppResult<PathBuf> {
    let url = format!("{PROFILES_URL}/{name}.pb");
    let body = ureq::get(&url).call()?.into_body().read_to_string()?;
    let dest = dir::profile_path(dst)?;
    fs::write(&dest, body)?;
    Ok(dest)
}
