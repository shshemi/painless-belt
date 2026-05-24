pub mod operations;
pub mod profile;
pub mod template;

pub trait ToSbdl {
    fn to_sbdl(&self) -> String;
}
