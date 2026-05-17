use crate::traits::ToSbpl;

#[derive(Debug)]
pub struct Sandbox {
    def: Action,
    plc: Vec<Box<dyn ToSbpl>>,
}

impl Sandbox {
    pub fn allow_by_default() -> Self {
        Self {
            def: Action::Allow,
            plc: Default::default(),
        }
    }

    pub fn deny_by_default() -> Self {
        Self {
            def: Action::Deny,
            plc: Default::default(),
        }
    }

    pub fn with_policy(mut self, policy: impl ToSbpl + 'static) -> Self {
        self.plc.push(Box::new(policy));
        self
    }

    pub fn init(&self) -> Result<(), crate::ffi::Error> {
        crate::ffi::sandbox_init(&self.to_sbpl(), 0)
    }
}

impl ToSbpl for Sandbox {
    fn to_sbpl(&self) -> String {
        let mut out = format!("(version 1)\n({} default)", self.def.to_sbpl());
        for r in &self.plc {
            out.push('\n');
            out.push_str(&r.to_sbpl());
        }
        out
    }
}

#[derive(Debug)]
enum Action {
    Allow,
    Deny,
}

impl ToSbpl for Action {
    fn to_sbpl(&self) -> String {
        match self {
            Action::Allow => "allow".into(),
            Action::Deny => "deny".into(),
        }
    }
}
