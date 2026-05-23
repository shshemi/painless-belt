use crate::sandbox::Operation;

#[derive(Debug)]
pub struct MachPerUserLookup;

impl Operation for MachPerUserLookup {
    fn render(&self) -> String {
        "mach-per-user-lookup".to_string()
    }
}
