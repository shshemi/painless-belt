use crate::sandbox::Operation;

#[derive(Debug)]
pub struct MachTaskName;

impl Operation for MachTaskName {
    fn render(&self) -> String {
        "mach-task-name".to_string()
    }
}
