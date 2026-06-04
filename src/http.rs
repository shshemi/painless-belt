use std::{fs, path::PathBuf};

use crate::{AppResult, dir};

const PROFILES_URL: &str =
    "https://raw.githubusercontent.com/shshemi/painless-belt/master/profiles";

fn fetch_profile_body(name: &str) -> AppResult<String> {
    let url = format!("{PROFILES_URL}/{name}.pb");
    Ok(ureq::get(&url).call()?.into_body().read_to_string()?)
}

pub fn pull_profile(name: &str, dst: &str) -> AppResult<PathBuf> {
    let body = fetch_profile_body(name)?;
    let dest = dir::profile_path(dst)?;
    fs::write(&dest, body)?;
    Ok(dest)
}
