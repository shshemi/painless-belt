use super::Filter;

pub mod signal;

#[derive(Debug)]
enum SignalFilter {
    SelfTarget,
    Others,
}

impl Filter for SignalFilter {
    fn render(&self) -> String {
        match self {
            SignalFilter::SelfTarget => "(target self)".to_string(),
            SignalFilter::Others => "(target others)".to_string(),
        }
    }
}
