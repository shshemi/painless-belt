pub mod file;
pub mod iokit;
pub mod ipc;
pub mod mach;
pub mod network;
pub mod signal;
pub mod sysctl;

pub trait Filter {
    fn render(&self) -> String;
}
