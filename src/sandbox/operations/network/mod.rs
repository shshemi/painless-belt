use super::Filter;

pub mod network;
pub mod network_bind;
pub mod network_inbound;
pub mod network_outbound;

#[derive(Debug)]
pub enum Proto {
    Ip,
    Tcp,
    Udp,
}

impl Proto {
    fn render(&self) -> &'static str {
        match self {
            Proto::Ip => "ip",
            Proto::Tcp => "tcp",
            Proto::Udp => "udp",
        }
    }
}

/// macOS sandbox only accepts `*` or `localhost` for the host portion of a
/// network filter — arbitrary IPs and hostnames are rejected at profile init.
#[derive(Debug)]
pub enum Host {
    Any,
    Localhost,
}

impl Host {
    fn render(&self) -> &'static str {
        match self {
            Host::Any => "*",
            Host::Localhost => "localhost",
        }
    }
}

#[derive(Debug)]
pub enum Port {
    Any,
    Number(u16),
}

impl Port {
    fn render(&self) -> String {
        match self {
            Port::Any => "*".to_string(),
            Port::Number(n) => n.to_string(),
        }
    }
}

#[derive(Debug)]
pub(super) enum NetworkFilter {
    Local {
        proto: Proto,
        host: Host,
        port: Port,
    },
    Remote {
        proto: Proto,
        host: Host,
        port: Port,
    },
}

impl Filter for NetworkFilter {
    fn render(&self) -> String {
        match self {
            NetworkFilter::Local { proto, host, port } => format!(
                "(local {} \"{}:{}\")",
                proto.render(),
                host.render(),
                port.render(),
            ),
            NetworkFilter::Remote { proto, host, port } => format!(
                "(remote {} \"{}:{}\")",
                proto.render(),
                host.render(),
                port.render(),
            ),
        }
    }
}
