use crate::traits::ToSbpl;

#[derive(Debug)]
pub struct ExecuteSystemBinaries;

impl ToSbpl for ExecuteSystemBinaries {
    fn to_sbpl(&self) -> String {
        r#"(allow process-exec)
(allow process-fork)
(allow file-read* (literal "/"))
(allow file-read* (subpath "/usr/bin"))
(allow file-read* (subpath "/bin"))
(allow file-read* (subpath "/usr/lib"))
(allow file-read* (subpath "/usr/share"))
(allow file-read* (subpath "/System"))
(allow file-read* (subpath "/private/etc"))
(allow file-map-executable)
(allow file-read-metadata)
(allow mach-lookup)
(allow sysctl-read)
(allow process-info*)
(allow iokit-open)
(allow signal)"#
            .to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn execute_system_binaries_renders() {
        let expected = "(allow process-exec)\n\
            (allow process-fork)\n\
            (allow file-read* (literal \"/\"))\n\
            (allow file-read* (subpath \"/usr/bin\"))\n\
            (allow file-read* (subpath \"/bin\"))\n\
            (allow file-read* (subpath \"/usr/lib\"))\n\
            (allow file-read* (subpath \"/usr/share\"))\n\
            (allow file-read* (subpath \"/System\"))\n\
            (allow file-read* (subpath \"/private/etc\"))\n\
            (allow file-map-executable)\n\
            (allow file-read-metadata)\n\
            (allow mach-lookup)\n\
            (allow sysctl-read)\n\
            (allow process-info*)\n\
            (allow iokit-open)\n\
            (allow signal)";
        assert_eq!(ExecuteSystemBinaries.to_sbpl(), expected);
    }
}
