pub mod operations;
pub mod policy;
pub mod profile;
pub mod template;

pub trait ToSbdl {
    fn to_sbdl(&self) -> String;
}

impl<T> ToSbdl for T
where
    for<'a> &'a T: IntoIterator,
    for<'a> <&'a T as IntoIterator>::Item: ToSbdl,
{
    fn to_sbdl(&self) -> String {
        self.into_iter()
            .map(|item| item.to_sbdl())
            .collect::<Vec<_>>()
            .join("\n")
    }
}
