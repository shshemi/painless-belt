// pub mod operations;
// pub mod policy;
pub mod profile;
pub mod rule_set;
pub mod template;

pub trait ToSbpl {
    fn to_sbpl(&self) -> &str;
}
