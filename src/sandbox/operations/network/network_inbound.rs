use super::{Host, NetworkFilter, Port, Proto};
use crate::sandbox::Operation;

#[derive(Debug)]
pub struct NetworkInbound {
    filter: NetworkFilter,
}

impl NetworkInbound {
    pub fn local(proto: Proto, host: Host, port: Port) -> Self {
        Self { filter: NetworkFilter::Local { proto, host, port } }
    }

    pub fn remote(proto: Proto, host: Host, port: Port) -> Self {
        Self { filter: NetworkFilter::Remote { proto, host, port } }
    }
}

impl Operation for NetworkInbound {
    fn render(&self) -> String {
        format!("network-inbound {}", self.filter.to_sbdl())
    }
}
