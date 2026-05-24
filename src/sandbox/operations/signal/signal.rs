use super::SignalFilter;
use crate::sandbox::ToSbdl;

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

impl ToSbdl for Signal {
    fn to_sbdl(&self) -> String {
        format!("signal {}", self.filter.to_sbdl())
    }
}
