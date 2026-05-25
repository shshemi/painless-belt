use crate::sandbox::ToSbdl;

#[derive(Debug)]
pub struct ExecuteSystemBinaries;

impl ToSbdl for ExecuteSystemBinaries {
    fn to_sbdl(&self) -> String {
        r#"(allow process-exec)
(allow process-fork)
(allow file-read* (subpath "/"))
(allow file-write* (subpath "/dev"))
(allow file-ioctl)
(allow file-map-executable)
(allow file-read-metadata)
(allow mach-lookup)
(allow sysctl-read)
(allow process-info*)
(allow iokit-open)
(allow ipc-posix-shm)
(allow system-socket)
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
            (allow file-read* (subpath \"/\"))\n\
            (allow file-write* (subpath \"/dev\"))\n\
            (allow file-ioctl)\n\
            (allow file-map-executable)\n\
            (allow file-read-metadata)\n\
            (allow mach-lookup)\n\
            (allow sysctl-read)\n\
            (allow process-info*)\n\
            (allow iokit-open)\n\
            (allow ipc-posix-shm)\n\
            (allow system-socket)\n\
            (allow signal)";
        assert_eq!(ExecuteSystemBinaries.to_sbdl(), expected);
    }
}
