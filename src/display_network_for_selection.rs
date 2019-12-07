use crate::structs::*;
pub static KNOWN_TOKEN: &str = "K";
pub static OPEN_TOKEN: &str = "O";

impl AnnotatedWirelessNetwork {
    pub fn get_display_string(&self) -> String {
        // TODO: change unit tests
        let tags = self.get_tags_string();
        let strength = self.get_strenth_string();
        format!("{}{}{}", strength, self.essid, tags)
    }

    pub(crate) fn get_tags_string(&self) -> String {
        let open = !self.is_encrypted;
        let known = self.known;
        let open_tag = if open { OPEN_TOKEN } else { "" };
        let known_tag = if known { KNOWN_TOKEN } else { "" };

        if open || known {
            format!(" [{}{}]", open_tag, known_tag)
        } else {
            "".to_string()
        }
    }

    pub(crate) fn get_strenth_string(&self) -> String {
        match self.signal_strength {
            Some(st) => format!("[{}] ", st + 90),
            None => "".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn verify_display_strength_and_tags(
        is_known: bool,
        is_open: bool,
        signal_strength: Option<i32>,
    ) {
        let essid = "YEEEEEEE".to_string();
        let nw = AnnotatedWirelessNetwork {
            essid: essid.clone(),
            known: is_known,
            is_encrypted: !is_open,
            signal_strength,
            ..Default::default()
        };
        let token = nw.get_display_string();
        let tags_string = nw.get_tags_string();
        let strength_string = nw.get_strenth_string();

        assert![token.starts_with(&strength_string)];
        assert![token.contains(&essid)];
        assert![token.ends_with(&tags_string)];

        assert_eq![is_known, tags_string.contains(&KNOWN_TOKEN)];
        assert_eq![is_open, tags_string.contains(&OPEN_TOKEN)];

        if let Some(st) = signal_strength {
            assert![strength_string.contains(&format!("{}", st + 90))];
        } else {
            assert![strength_string.is_empty()];
        }
    }

    #[test]
    fn test_display() {
        verify_display_strength_and_tags(true, true, None);
        verify_display_strength_and_tags(false, true, None);
        verify_display_strength_and_tags(true, false, Some(-32));
        verify_display_strength_and_tags(false, false, Some(-90));
    }
}