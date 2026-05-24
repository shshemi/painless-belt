use crate::sandbox::ToSbdl;

#[derive(Debug)]
pub struct MachPrivHostPort;

impl ToSbdl for MachPrivHostPort {
    fn to_sbdl(&self) -> String {
        "mach-priv-host-port".to_string()
    }
}
