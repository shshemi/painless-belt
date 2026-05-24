use crate::sandbox::ToSbdl;

#[derive(Debug)]
pub struct MachPrivTaskPort;

impl ToSbdl for MachPrivTaskPort {
    fn to_sbdl(&self) -> String {
        "mach-priv-task-port".to_string()
    }
}
