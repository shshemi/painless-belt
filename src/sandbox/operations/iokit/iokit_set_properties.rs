use regex::Regex;

use super::IokitFilter;
use crate::sandbox::ToSbdl;

#[derive(Debug)]
pub struct IokitSetProperties {
    filter: IokitFilter,
}

impl IokitSetProperties {
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

impl ToSbdl for IokitSetProperties {
    fn to_sbdl(&self) -> String {
        format!("iokit-set-properties {}", self.filter.to_sbdl())
    }
}
