use crate::sandbox::Operation;

#[derive(Debug)]
pub struct MachHostSpecialPortSet;

impl Operation for MachHostSpecialPortSet {
    fn render(&self) -> String {
        "mach-host-special-port-set".to_string()
    }
}
