use crate::structs::*;
pub const KNOWN_TOKEN: &str = " (KNOWN)";

impl AnnotatedWirelessNetwork {
    pub fn get_display_string(&self) -> String {
        if self.known {
            self.essid.clone() + KNOWN_TOKEN
        } else {
            self.essid.clone()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_token_for_known_network() {
        let essid = "DOOK".to_string();
        let nw = AnnotatedWirelessNetwork {
            essid: essid.clone(),
            known: true,
            ..Default::default()
        };
        let token = nw.get_display_string();
        assert_eq![token, essid + KNOWN_TOKEN];
    }

    #[test]
    fn test_get_token_for_unknown_network() {
        let essid = "DOOK".to_string();
        let nw = AnnotatedWirelessNetwork {
            essid: essid.clone(),
            known: false,
            ..Default::default()
        };
        let token = nw.get_display_string();
        assert_eq![token, essid];
    }
}
