use std::io;

pub fn read_from_stdin(msg: &mut String, error_msg: &str) {
    io::stdin().read_line(msg).expect(error_msg);
}

pub fn get_values_by_delim(s: String, delim: &str) -> Vec<String> {
    s.split(delim)
        .filter_map(|val| {
            if val.is_empty() {
                return None;
            }
            Some(val.to_owned())
        })
        .collect::<Vec<String>>()
}

#[cfg(test)]
mod tests {
    use super::get_values_by_delim;

    #[test]
    fn test_split_names_handles_leading_comma() {
        assert_eq!(
            get_values_by_delim(String::from(",Anthony,Caroline"), ","),
            vec!["Anthony", "Caroline"]
        );
    }
    #[test]
    fn test_split_names_handles_trailing_comma() {
        assert_eq!(
            get_values_by_delim(String::from("Anthony,Caroline,"), ","),
            vec!["Anthony", "Caroline"]
        );
    }

    #[test]
    fn test_split_names_handles_leading_and_trailing_comma() {
        assert_eq!(
            get_values_by_delim(String::from(",Anthony,Caroline,"), ","),
            vec!["Anthony", "Caroline"]
        );
    }
}
