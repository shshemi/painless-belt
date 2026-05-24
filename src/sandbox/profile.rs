use crate::sandbox::Operation;

#[derive(Debug)]
pub struct Profile {
    prf: String,
}

impl Profile {
    pub fn allow_by_default() -> Self {
        Self {
            prf: "(version 1)\n(allow default)".to_owned(),
        }
    }

    pub fn deny_by_default() -> Self {
        Self {
            prf: "(version 1)\n(deny default)".to_owned(),
        }
    }

    pub fn allow(mut self, op: impl Operation + 'static) -> Self {
        self.prf.push_str(&format!("\n(allow {})", op.render()));
        self
    }

    pub fn deny(mut self, op: impl Operation + 'static) -> Self {
        self.prf.push_str(&format!("\n(deny {})", op.render()));
        self
    }

    pub fn init(&self) -> Result<(), crate::ffi::Error> {
        crate::ffi::sandbox_init(&self.prf, 0)
    }
}

impl AsRef<str> for Profile {
    fn as_ref(&self) -> &str {
        &self.prf
    }
}
