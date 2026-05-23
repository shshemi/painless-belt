use crate::sandbox::Operation;

#[derive(Debug)]
pub struct MachBootstrap;

impl Operation for MachBootstrap {
    fn render(&self) -> String {
        "mach-bootstrap".to_string()
    }
}
