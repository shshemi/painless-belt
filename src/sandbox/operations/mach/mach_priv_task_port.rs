use crate::sandbox::Operation;

#[derive(Debug)]
pub struct MachPrivTaskPort;

impl Operation for MachPrivTaskPort {
    fn render(&self) -> String {
        "mach-priv-task-port".to_string()
    }
}
