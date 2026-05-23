use super::SignalFilter;
use crate::sandbox::Operation;
use crate::sandbox::operations::Filter;

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
        format!("signal {}", self.filter.render())
    }
}
