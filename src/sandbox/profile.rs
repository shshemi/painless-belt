use crate::{
    AppResult,
    dir::profile,
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
        Ok(Self {
            inner: render(&std::fs::read_to_string(profile(name)?)?)?,
        })
    }

    pub fn with(mut self, rules: &impl ToSbdl) -> Self {
        self.inner.push('\n');
        self.inner.push_str(rules.to_sbdl());
        self
    }

    pub fn init(&self) -> AppResult<()> {
        crate::ffi::sandbox_init(&self.inner, 0)
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
            inner: render(include_str!("../../default.pb")).expect("Invalid default profile"),
        }
    }
}
