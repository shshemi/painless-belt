use std::{fs, path::PathBuf};

use anyhow::anyhow;

use crate::{AppResult, dir};

const PROFILES_URL: &str =
    "https://raw.githubusercontent.com/shshemi/painless-belt/master/profiles";

pub fn pull_profile(name: &str, dst: &str) -> AppResult<PathBuf> {
    let url = format!("{PROFILES_URL}/{name}.pb");
    let body = ureq::get(&url)
        .call()
        .map_err(|e| match e {
            ureq::Error::StatusCode(404) => anyhow!(
                "No upstream profile named '{name}' (looked at {url}). Run `pb ls` to see local profiles."
            ),
            ureq::Error::StatusCode(code) => {
                anyhow!("Failed to fetch profile '{name}' from {url}: upstream returned HTTP {code}")
            }
            ureq::Error::HostNotFound | ureq::Error::Io(_) | ureq::Error::ConnectionFailed => {
                anyhow!("Failed to fetch profile '{name}' from {url}: could not reach the server (no network?)")
            }
            other => anyhow!("Failed to fetch profile '{name}' from {url}: {other}"),
        })?
        .into_body()
        .read_to_string()
        .map_err(|e| anyhow!("Failed to read the response for profile '{name}' from {url}: {e}"))?;
    let dest = dir::profile_path(dst)?;
    fs::write(&dest, body)
        .map_err(|e| anyhow!("Failed to save profile '{dst}' to {}: {e}", dest.display()))?;
    Ok(dest)
}
