#[derive(Debug)]
pub enum Action {
    Allow,
    Deny,
}

impl Action {
    pub fn render(&self) -> String {
        match self {
            Action::Allow => "allow",
            Action::Deny => "deny",
        }
        .into()
    }
}
