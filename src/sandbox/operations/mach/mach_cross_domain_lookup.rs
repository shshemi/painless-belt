use crate::sandbox::Operation;

#[derive(Debug)]
pub struct MachCrossDomainLookup;

impl Operation for MachCrossDomainLookup {
    fn render(&self) -> String {
        "mach-cross-domain-lookup".to_string()
    }
}
