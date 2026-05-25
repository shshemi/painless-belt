use crate::sandbox::ToSbdl;

#[derive(Debug)]
pub struct ConnectToInternet;

impl ToSbdl for ConnectToInternet {
    fn to_sbdl(&self) -> String {
        "(allow network-outbound)".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn connect_to_internet_renders() {
        assert_eq!(ConnectToInternet.to_sbdl(), "(allow network-outbound)");
    }
}
