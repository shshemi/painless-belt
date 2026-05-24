use crate::sandbox::ToSbdl;

#[derive(Debug)]
pub struct MachHostSpecialPortSet;

impl ToSbdl for MachHostSpecialPortSet {
    fn to_sbdl(&self) -> String {
        "mach-host-special-port-set".to_string()
    }
}
