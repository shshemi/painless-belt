use regex::Regex;

use crate::misc::ext::str_ext::StrExt;

pub mod iokit_open;
pub mod iokit_set_properties;

#[derive(Debug)]
enum IokitFilter {
    UserClientClass(String),
    UserClientClassRegex(Regex),
    Property(String),
    PropertyRegex(Regex),
}

impl IokitFilter {
    fn to_sbdl(&self) -> String {
        match self {
            IokitFilter::UserClientClass(s) => {
                format!("(iokit-user-client-class \"{}\")", s.escape())
            }
            IokitFilter::UserClientClassRegex(r) => format!(
                "(iokit-user-client-class-regex #\"{}\")",
                r.as_str().replace('"', "\\\"")
            ),
            IokitFilter::Property(s) => format!("(iokit-property \"{}\")", s.escape()),
            IokitFilter::PropertyRegex(r) => format!(
                "(iokit-property-regex #\"{}\")",
                r.as_str().replace('"', "\\\"")
            ),
        }
    }
}
