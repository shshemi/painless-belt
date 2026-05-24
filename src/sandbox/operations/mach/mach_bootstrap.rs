use crate::sandbox::ToSbdl;

#[derive(Debug)]
pub struct MachBootstrap;

impl ToSbdl for MachBootstrap {
    fn to_sbdl(&self) -> String {
        "mach-bootstrap".to_string()
    }
}
