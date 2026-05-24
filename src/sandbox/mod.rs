pub mod operations;
pub mod profile;
pub mod template;

pub trait Operation {
    fn render(&self) -> String;
}
