use anyhow::anyhow;

use crate::{
    AppResult,
    fs::profile_path,
    sandbox::{ToSbpl, template::render},
};

#[derive(Debug)]
pub struct Profile {
    inner: String,
}

fn profile_to_string(name: &str) -> AppResult<String> {
    let path = profile_path(name)?;
    std::fs::read_to_string(&path).map_err(|e| {
        if e.kind() == std::io::ErrorKind::NotFound {
            anyhow!(
                "Profile '{name}' not found at {}. Run `pb ls` to list profiles or `pb pull {name}` to fetch it.",
                path.display()
            )
        } else {
            anyhow!("Failed to read profile '{name}' at {}: {e}", path.display())
        }
    })
}

impl Profile {
    pub fn allow_by_default() -> Self {
        Self {
            inner: "(version 1)\n(allow default)".to_owned(),
        }
    }

    pub fn deny_by_default() -> Self {
        Self {
            inner: "(version 1)\n(deny default)".to_owned(),
        }
    }

    pub fn load(name: &str) -> AppResult<Self> {
        let profile = profile_to_string(name)
            .and_then(|s| render(&s))
            .map(|profile| Self { inner: profile });
        match name {
            "default" => Ok(profile.unwrap_or_default()),
            "empty" => Ok(profile.unwrap_or(Self::empty())),
            _ => profile,
        }
    }

    pub fn with(mut self, rules: &impl ToSbpl) -> Self {
        self.inner.push('\n');
        self.inner.push_str(rules.to_sbpl());
        self
    }

    pub fn init(&self) -> AppResult<()> {
        crate::ffi::sandbox_init(&self.inner, 0)
    }

    pub fn empty() -> Self {
        Self {
            inner: render(include_str!("../../profiles/empty.pb")).expect("Invalid profile"),
        }
    }

    pub fn merge(mut self, other: &Self) -> Self {
        self.inner.push('\n');
        self.inner.push_str(&other.inner);
        self
    }
}

impl AsRef<str> for Profile {
    fn as_ref(&self) -> &str {
        &self.inner
    }
}

impl Default for Profile {
    fn default() -> Self {
        Self {
            inner: render(include_str!("../../profiles/default.pb"))
                .expect("Invalid default profile"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn profile(inner: &str) -> Profile {
        Profile {
            inner: inner.to_owned(),
        }
    }

    #[test]
    fn merge_with_empty_profile_is_unchanged() {
        let base = profile("(version 1)\n(deny default)\n(allow signal)");
        let empty = profile("(version 1)\n(deny default)");
        assert_eq!(
            base.merge(&empty).as_ref(),
            "(version 1)\n(deny default)\n(allow signal)\n(version 1)\n(deny default)"
        );
    }
}
