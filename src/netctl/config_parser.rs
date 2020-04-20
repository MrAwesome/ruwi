use super::structs::*;

use strum::AsStaticRef;
use strum_macros::AsStaticStr;
use unescape::unescape;

#[derive(Debug, Clone, PartialEq, Eq, AsStaticStr)]
enum NetctlFieldKey {
    #[strum(serialize = "ESSID=")]
    Essid,
    #[strum(serialize = "Interface=")]
    Interface,
    #[strum(serialize = "Connection=")]
    ConnectionType,
    #[strum(serialize = "Key=")]
    EncryptionKey,
}

impl<'a> NetctlRawConfig<'a> {
    fn get_field(&self, field: NetctlFieldKey) -> Option<String> {
        let token = field.as_static();
        self.contents.as_ref().lines().find_map(|line| {
            if line.starts_with(token) {
                // TODO: better matching of quote types, since a password/essid could end with an escaped quote
                let value = line
                    .trim_start_matches(token)
                    .trim_start_matches('\'')
                    .trim_start_matches('"')
                    .trim_end_matches('\'')
                    .trim_end_matches('"')
                    .to_string();
                Some(value)
            } else {
                None
            }
        })
    }

    // NOTE: ESSID is unescaped here. This can happen further up the stack if necessary.
    pub(super) fn get_essid(&self) -> Option<String> {
        self.get_field(NetctlFieldKey::Essid)
            .map(|raw_essid_entry| unescape(&raw_essid_entry).unwrap_or(raw_essid_entry))
    }

    pub(super) fn get_interface(&self) -> Option<String> {
        self.get_field(NetctlFieldKey::Interface)
    }

    pub(super) fn get_connection_type(&self) -> Option<String> {
        self.get_field(NetctlFieldKey::ConnectionType)
    }

    pub(super) fn get_encryption_key(&self) -> Option<String> {
        self.get_field(NetctlFieldKey::EncryptionKey)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use strum::AsStaticRef;

    fn get_config<'a>(contents: &str) -> NetctlRawConfig<'a> {
        NetctlRawConfig::builder()
            .contents(contents)
            .identifier("lawl")
            .location("/tmp/lawl")
            .build()
    }

    fn get_ethernet_config<'a>() -> NetctlRawConfig<'a> {
        get_config(super::super::tests::ETHERNET_SAMPLE)
    }

    fn get_wireless_open_config<'a>() -> NetctlRawConfig<'a> {
        get_config(super::super::tests::WIRELESS_OPEN_SAMPLE)
    }

    fn get_wireless_encrypted_config<'a>() -> NetctlRawConfig<'a> {
        get_config(super::super::tests::WIRELESS_ENCRYPTED_SAMPLE)
    }

    #[test]
    fn test_parse_ethernet() {
        let config = get_ethernet_config();
        assert_eq!["enp0s25", config.get_interface().unwrap()];
        assert_eq![
            NetctlConnectionType::Wired.as_static(),
            config.get_connection_type().unwrap()
        ];
        assert![config.get_essid().is_none()];
        assert![config.get_encryption_key().is_none()];
    }

    #[test]
    fn test_parse_wireless_open() {
        let config = get_wireless_open_config();
        assert_eq!["wlp3s0", config.get_interface().unwrap()];
        assert_eq![
            NetctlConnectionType::Wifi.as_static(),
            config.get_connection_type().unwrap()
        ];
        assert_eq!["Chateau de Chine Hotel", config.get_essid().unwrap()];
        assert![config.get_encryption_key().is_none()];
    }

    #[test]
    fn test_parse_wireless_encrypted() {
        let config = get_wireless_encrypted_config();
        assert_eq!["wlp3s0", config.get_interface().unwrap()];
        assert_eq![
            NetctlConnectionType::Wifi.as_static(),
            config.get_connection_type().unwrap()
        ];
        assert_eq!["Lobby", config.get_essid().unwrap()];
        assert_eq!["KS201819", config.get_encryption_key().unwrap()];
    }
}
