// pub mod operations;
// pub mod policy;
pub mod profile;
pub mod rule_set;
pub mod template;

pub trait ToSbdl {
    fn to_sbdl(&self) -> &str;
}
