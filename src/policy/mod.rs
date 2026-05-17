pub mod connect_to_internet;
pub mod execute_system_binaries;
pub mod read_dir;
pub mod read_write_dir;
pub mod write_dir;

pub(crate) fn escape(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for c in s.chars() {
        match c {
            '\\' => {
                out.push('\\');
                out.push('\\');
            }
            '"' => {
                out.push('\\');
                out.push('"');
            }
            _ => out.push(c),
        }
    }
    out
}
