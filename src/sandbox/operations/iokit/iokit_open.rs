use regex::Regex;

use super::IokitFilter;
use crate::sandbox::Operation;

#[derive(Debug)]
pub struct IokitOpen {
    filter: IokitFilter,
}

impl IokitOpen {
    pub fn user_client_class(class: impl Into<String>) -> Self {
        Self {
            filter: IokitFilter::UserClientClass(class.into()),
        }
    }

    pub fn user_client_class_regex(regex: Regex) -> Self {
        Self {
            filter: IokitFilter::UserClientClassRegex(regex),
        }
    }

    pub fn property(prop: impl Into<String>) -> Self {
        Self {
            filter: IokitFilter::Property(prop.into()),
        }
    }

    pub fn property_regex(regex: Regex) -> Self {
        Self {
            filter: IokitFilter::PropertyRegex(regex),
        }
    }
}

impl Operation for IokitOpen {
    fn render(&self) -> String {
        format!("iokit-open {}", self.filter.to_sbdl())
    }
}
