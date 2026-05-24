use crate::sandbox::ToSbdl;

#[derive(Debug)]
pub struct MachPerUserLookup;

impl ToSbdl for MachPerUserLookup {
    fn to_sbdl(&self) -> String {
        "mach-per-user-lookup".to_string()
    }
}
