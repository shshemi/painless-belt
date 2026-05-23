mod action;
pub mod operations;
pub mod profile;
mod rule;

pub trait Operation {
    fn render(&self) -> String;
}
