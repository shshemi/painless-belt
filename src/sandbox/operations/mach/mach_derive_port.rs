use crate::sandbox::ToSbdl;

#[derive(Debug)]
pub struct MachDerivePort;

impl ToSbdl for MachDerivePort {
    fn to_sbdl(&self) -> String {
        "mach-derive-port".to_string()
    }
}
