use super::{Host, NetworkFilter, Port, Proto};
use crate::sandbox::ToSbdl;

#[derive(Debug)]
pub struct NetworkBind {
    filter: NetworkFilter,
}

impl NetworkBind {
    pub fn local(proto: Proto, host: Host, port: Port) -> Self {
        Self {
            filter: NetworkFilter::Local { proto, host, port },
        }
    }

    pub fn remote(proto: Proto, host: Host, port: Port) -> Self {
        Self {
            filter: NetworkFilter::Remote { proto, host, port },
        }
    }
}

impl ToSbdl for NetworkBind {
    fn to_sbdl(&self) -> String {
        format!("network-bind {}", self.filter.to_sbdl())
    }
}
