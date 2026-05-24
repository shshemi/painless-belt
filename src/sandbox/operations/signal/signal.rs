use super::SignalFilter;
use crate::sandbox::Operation;

#[derive(Debug)]
pub struct Signal {
    filter: SignalFilter,
}

impl Signal {
    pub fn self_target() -> Self {
        Self {
            filter: SignalFilter::SelfTarget,
        }
    }

    pub fn others() -> Self {
        Self {
            filter: SignalFilter::Others,
        }
    }
}

impl Operation for Signal {
    fn render(&self) -> String {
        format!("signal {}", self.filter.to_sbdl())
    }
}
