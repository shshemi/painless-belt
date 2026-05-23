use std::fmt::Debug;

use crate::sandbox::Operation;

use super::action::Action;

pub struct Rule {
    action: Action,
    operation: Box<dyn Operation>,
}

impl Rule {
    pub fn allow<Op: Operation + 'static>(operation: Op) -> Self {
        Self {
            action: Action::Allow,
            operation: Box::new(operation),
        }
    }

    pub fn deny<Op: Operation + 'static>(operation: Op) -> Self {
        Self {
            action: Action::Deny,
            operation: Box::new(operation),
        }
    }

    pub fn render(&self) -> String {
        format!("({} {})", self.action.render(), self.operation.render())
    }
}

impl Debug for Rule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Rule")
            .field("action", &self.action)
            .field("operation", &"<dyn Operation>")
            .finish()
    }
}
