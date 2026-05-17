use crate::traits::ToSbpl;

#[derive(Debug)]
pub struct ConnectToInternet;

impl ToSbpl for ConnectToInternet {
    fn to_sbpl(&self) -> String {
        "(allow network-outbound)".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn connect_to_internet_renders() {
        assert_eq!(ConnectToInternet.to_sbpl(), "(allow network-outbound)");
    }
}
