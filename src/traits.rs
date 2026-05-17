use core::fmt::Debug;

pub trait ToSbpl: Debug {
    fn to_sbpl(&self) -> String;
}
