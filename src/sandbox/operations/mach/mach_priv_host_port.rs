use crate::sandbox::Operation;

#[derive(Debug)]
pub struct MachPrivHostPort;

impl Operation for MachPrivHostPort {
    fn render(&self) -> String {
        "mach-priv-host-port".to_string()
    }
}
