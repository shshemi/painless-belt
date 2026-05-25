#[allow(clippy::module_inception)]
pub mod signal;

#[derive(Debug)]
enum SignalFilter {
    SelfTarget,
    Others,
}

impl SignalFilter {
    fn to_sbdl(&self) -> String {
        match self {
            SignalFilter::SelfTarget => "(target self)".to_string(),
            SignalFilter::Others => "(target others)".to_string(),
        }
    }
}
