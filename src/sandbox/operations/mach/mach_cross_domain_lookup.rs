use crate::sandbox::ToSbdl;

#[derive(Debug)]
pub struct MachCrossDomainLookup;

impl ToSbdl for MachCrossDomainLookup {
    fn to_sbdl(&self) -> String {
        "mach-cross-domain-lookup".to_string()
    }
}
