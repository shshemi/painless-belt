use crate::sandbox::Operation;

#[derive(Debug)]
pub struct MachDerivePort;

impl Operation for MachDerivePort {
    fn render(&self) -> String {
        "mach-derive-port".to_string()
    }
}
