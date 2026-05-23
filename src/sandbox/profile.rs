use crate::sandbox::Operation;

use super::{action::Action, rule::Rule};

#[derive(Debug)]
pub struct Profile {
    by_default: Action,
    rules: Vec<Rule>,
}

impl Profile {
    pub fn allow_by_default() -> Self {
        Self {
            by_default: Action::Allow,
            rules: Vec::new(),
        }
    }

    pub fn deny_by_default() -> Self {
        Self {
            by_default: Action::Deny,
            rules: Vec::new(),
        }
    }

    pub fn allow(mut self, op: impl Operation + 'static) -> Self {
        self.rules.push(Rule::allow(op));
        self
    }

    pub fn deny(mut self, op: impl Operation + 'static) -> Self {
        self.rules.push(Rule::deny(op));
        self
    }

    pub fn render(&self) -> String {
        let mut out = format!("(version 1)\n({} default)", self.by_default.render());
        for rule in &self.rules {
            out.push('\n');
            out.push_str(&rule.render());
        }
        out
    }

    pub fn init(&self) -> Result<(), crate::ffi::Error> {
        crate::ffi::sandbox_init(&self.render(), 0)
    }
}
