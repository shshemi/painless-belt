use crate::sandbox::Operation;

#[derive(Debug)]
pub struct MachIssueExtension;

impl Operation for MachIssueExtension {
    fn render(&self) -> String {
        "mach-issue-extension".to_string()
    }
}
