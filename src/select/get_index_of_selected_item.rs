use crate::rerr;
use crate::errors::*;
use super::additional_options_for_manual_selection::SelectionOption;

use std::str::FromStr;

pub(super) fn get_index_of_selected_item(line: &str) -> Result<usize, RuwiError> {
    let line = line.trim();

    if line == "." {
       return throw_refresh_error_and_print();
    }

    if line == "" {
       return Ok(0);
    }

    if let Ok(selection_option) = SelectionOption::from_str(line) {
        match selection_option {
            SelectionOption::Refresh => throw_refresh_error_and_print(),
        }
    } else {
        line.split(") ")
            .next()
            .ok_or_else(|| get_line_parse_err(line))?
            .parse::<usize>()
            .or_else(|_| Err(get_line_parse_err(line)))
    }
}

fn get_line_parse_err(line: &str) -> RuwiError {
    rerr!(
        RuwiErrorKind::FailedToParseSelectedLine,
        format!("Failed to parse line {}", line)
    )
}

fn get_refresh_requested_err() -> RuwiError {
    rerr!(RuwiErrorKind::RefreshRequested, "Refresh requested.")
}

fn throw_refresh_error_and_print() -> Result<usize, RuwiError> {
    eprintln!("[NOTE]: Refresh requested, running synchronous scan.");
    Err(get_refresh_requested_err())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_indices() -> Result<(), RuwiError> {
        let test_cases: Vec<(&str, Result<usize, RuwiError>)> = vec![
            ("1) jfdlskajfdlksa", Ok(1)),
            ("0) jfdlskajfdlksa", Ok(0)),
            ("22) jfdlskajfdlksa", Ok(22)),
            ("69) 54) jfdlskajfdlksa", Ok(69)),
            ("4000) jfdlskajfdlksa", Ok(4000)),
            ("4000000000) jfdlskajfdlksa", Ok(4_000_000_000)),
            ("-12) negawifi", Err(get_line_parse_err("-12) negawifi"))),
            ("jf jfjf", Err(get_line_parse_err("jf jfjf"))),
            ("!@&*(#@!", Err(get_line_parse_err("!@&*(#@!"))),
            ("refresh", Err(get_refresh_requested_err())),
            (".", Err(get_refresh_requested_err())),
            (" ", Ok(0)),
            ("\n", Ok(0)),
            ("\t", Ok(0)),
            ("\n\n\t \t", Ok(0)),
            ("\n\n\t1\t", Ok(1)),
            ("\n\t12\t", Ok(12)),
        ];

        for (line, res) in test_cases {
            dbg!(&line, &res);
            match get_index_of_selected_item(line) {
                Ok(val) => assert_eq![res?, val],
                Err(err) => assert_eq![res.err().unwrap().kind, err.kind],
            }
        }
        Ok(())
    }

}
