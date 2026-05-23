use super::{Host, NetworkFilter, Port, Proto};
use crate::sandbox::Operation;
use crate::sandbox::operations::Filter;

#[derive(Debug)]
pub struct Network {
    filter: NetworkFilter,
}

impl Network {
    pub fn local(proto: Proto, host: Host, port: Port) -> Self {
        Self { filter: NetworkFilter::Local { proto, host, port } }
    }

    pub fn remote(proto: Proto, host: Host, port: Port) -> Self {
        Self { filter: NetworkFilter::Remote { proto, host, port } }
    }
}

impl Operation for Network {
    fn render(&self) -> String {
        format!("network* {}", self.filter.render())
    }
}
