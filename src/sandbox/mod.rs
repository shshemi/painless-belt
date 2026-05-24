pub mod operations;
pub mod profile;

pub trait Operation {
    fn render(&self) -> String;
}
