use crate::sandbox::ToSbdl;

#[derive(Debug)]
pub struct MachTaskName;

impl ToSbdl for MachTaskName {
    fn to_sbdl(&self) -> String {
        "mach-task-name".to_string()
    }
}
