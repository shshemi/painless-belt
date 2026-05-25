use crate::{
    AppResult,
    dir::{profile, profiles_dir},
    sandbox::{
        ToSbdl,
        template::{self, render},
    },
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

    pub fn allow(mut self, op: impl ToSbdl + 'static) -> Self {
        self.inner.push_str(&format!("\n(allow {})", op.to_sbdl()));
        self
    }

    pub fn deny(mut self, op: impl ToSbdl + 'static) -> Self {
        self.inner.push_str(&format!("\n(deny {})", op.to_sbdl()));
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
        todo!()
    }
}
