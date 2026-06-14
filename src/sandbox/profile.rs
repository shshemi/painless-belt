use crate::{
    AppResult,
    dir::profile_path,
    sandbox::{ToSbpl, template::render},
};

#[derive(Debug)]
pub struct Profile {
    inner: String,
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
        let profile = profile_path(name)
            .and_then(|p| Ok(std::fs::read_to_string(p)?))
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

    pub fn push_rules(&mut self, rules: &str) {
        if !rules.is_empty() {
            self.inner.push('\n');
            self.inner.push_str(rules);
        }
    }

    pub fn init(&self) -> AppResult<()> {
        crate::ffi::sandbox_init(&self.inner, 0)
    }

    pub fn empty() -> Self {
        Self {
            inner: render(include_str!("../../profiles/empty.pb")).expect("Invalid profile"),
        }
    }

    /// Merge another profile's rules into this one, so the result allows
    /// whatever either profile allows. This profile's header is kept and
    /// `other`'s rules are appended, dropping `other`'s `(version ...)` and
    /// `(... default)` lines: re-declaring the catch-all default below
    /// existing rules would void them under SBPL's last-match-wins
    /// evaluation. `other`'s rules land last, so they win on conflicts.
    pub fn merge(mut self, other: &Self) -> Self {
        for line in other.inner.lines() {
            let trimmed = line.trim();
            if trimmed.is_empty()
                || trimmed.starts_with("(version")
                || trimmed == "(deny default)"
                || trimmed == "(allow default)"
            {
                continue;
            }
            self.inner.push('\n');
            self.inner.push_str(line);
        }
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
    fn merge_keeps_base_header_and_appends_other_rules() {
        let base = profile("(version 1)\n(deny default)\n(allow file-read* (subpath \"/a\"))");
        let other = profile("(version 1)\n(deny default)\n(allow file-read* (subpath \"/b\"))");
        assert_eq!(
            base.merge(&other).as_ref(),
            "(version 1)\n(deny default)\n\
             (allow file-read* (subpath \"/a\"))\n\
             (allow file-read* (subpath \"/b\"))"
        );
    }

    #[test]
    fn merge_drops_only_the_others_version_and_default() {
        let merged = profile("(version 1)\n(deny default)\n(allow signal)")
            .merge(&profile("(version 1)\n(allow default)\n(allow process-fork)"));
        let sbpl = merged.as_ref();
        // Only the base's header survives; the other's is stripped.
        assert_eq!(sbpl.matches("(version 1)").count(), 1);
        assert_eq!(sbpl.matches("default)").count(), 1);
        assert!(sbpl.contains("(allow signal)"));
        assert!(sbpl.contains("(allow process-fork)"));
    }

    #[test]
    fn merge_with_empty_profile_is_unchanged() {
        let base = profile("(version 1)\n(deny default)\n(allow signal)");
        let empty = profile("(version 1)\n(deny default)");
        assert_eq!(
            base.merge(&empty).as_ref(),
            "(version 1)\n(deny default)\n(allow signal)"
        );
    }
}
