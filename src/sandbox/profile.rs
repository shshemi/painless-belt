use crate::{
    AppResult,
    dir::profile_path,
    sandbox::{ToSbdl, template::render},
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

    pub fn with(mut self, rules: &impl ToSbdl) -> Self {
        self.inner.push('\n');
        self.inner.push_str(rules.to_sbdl());
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
