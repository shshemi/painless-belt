use crate::traits::ToSbpl;

#[derive(Debug)]
pub struct ExecuteSystemBinaries;

impl ToSbpl for ExecuteSystemBinaries {
    fn to_sbpl(&self) -> String {
        r#"(allow process-exec (subpath "/usr/bin"))
(allow process-exec (subpath "/bin"))
(allow file-read* (subpath "/usr/bin"))
(allow file-read* (subpath "/bin"))
(allow file-read* (subpath "/usr/lib"))
(allow file-read* (subpath "/System/Library"))"#
            .to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn execute_system_binaries_renders() {
        let expected = "(allow process-exec (subpath \"/usr/bin\"))\n\
            (allow process-exec (subpath \"/bin\"))\n\
            (allow file-read* (subpath \"/usr/bin\"))\n\
            (allow file-read* (subpath \"/bin\"))\n\
            (allow file-read* (subpath \"/usr/lib\"))\n\
            (allow file-read* (subpath \"/System/Library\"))";
        assert_eq!(ExecuteSystemBinaries.to_sbpl(), expected);
    }
}
