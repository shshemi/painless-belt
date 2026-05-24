use crate::sandbox::ToSbdl;

#[derive(Debug)]
pub struct MachIssueExtension;

impl ToSbdl for MachIssueExtension {
    fn to_sbdl(&self) -> String {
        "mach-issue-extension".to_string()
    }
}
