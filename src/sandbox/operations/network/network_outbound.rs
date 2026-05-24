use super::{Host, NetworkFilter, Port, Proto};
use crate::sandbox::Operation;

#[derive(Debug)]
pub struct NetworkOutbound {
    filter: NetworkFilter,
}

impl NetworkOutbound {
    pub fn local(proto: Proto, host: Host, port: Port) -> Self {
        Self { filter: NetworkFilter::Local { proto, host, port } }
    }

    pub fn remote(proto: Proto, host: Host, port: Port) -> Self {
        Self { filter: NetworkFilter::Remote { proto, host, port } }
    }
}

impl Operation for NetworkOutbound {
    fn render(&self) -> String {
        format!("network-outbound {}", self.filter.to_sbdl())
    }
}
