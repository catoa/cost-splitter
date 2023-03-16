#[cfg(test)]
mod tests {
    use splitter::utils::get_values_by_delim;

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
