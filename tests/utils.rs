#[cfg(test)]
mod tests {
    use splitter::io::MockInputReader;
    use splitter::utils::{get_values_by_delim, read_input};

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

    #[test]
    fn test_split_names_handles_whitespace() {
        assert_eq!(
            get_values_by_delim(String::from(" Anthony , Caroline "), ","),
            vec!["Anthony", "Caroline"]
        );
    }

    #[test]
    fn test_split_names_empty_string() {
        assert_eq!(
            get_values_by_delim(String::from(""), ","),
            Vec::<String>::new()
        );
    }

    #[test]
    fn test_split_names_single_value() {
        assert_eq!(
            get_values_by_delim(String::from("Anthony"), ","),
            vec!["Anthony"]
        );
    }

    #[test]
    fn test_split_names_different_delimiter() {
        assert_eq!(
            get_values_by_delim(String::from("Anthony|Caroline|Bob"), "|"),
            vec!["Anthony", "Caroline", "Bob"]
        );
    }

    #[test]
    fn test_read_input_with_mock() {
        let mut reader = MockInputReader::new(vec!["test input".to_string()]);
        let mut buffer = String::new();
        
        let result = read_input(&mut reader, &mut buffer);
        assert!(result.is_ok());
        assert!(buffer.contains("test input"));
    }

    #[test]
    fn test_read_input_multiple_lines() {
        let mut reader = MockInputReader::new(vec![
            "first line".to_string(),
            "second line".to_string(),
        ]);
        let mut buffer = String::new();
        
        let result1 = read_input(&mut reader, &mut buffer);
        assert!(result1.is_ok());
        assert!(buffer.contains("first line"));
        
        buffer.clear();
        let result2 = read_input(&mut reader, &mut buffer);
        assert!(result2.is_ok());
        assert!(buffer.contains("second line"));
    }
}
