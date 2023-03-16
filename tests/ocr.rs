#[cfg(test)]
mod tests {
    use splitter::charges::Charge;
    use splitter::ocr::get_charges_from_text;

    #[test]
    fn read_test_receipt() {
        let data = String::from(
            "Lorem 6.50 \n
            Ipsum 7.50 \n
            Dolor Sit 48.00",
        );
        let actual = get_charges_from_text(data);
        let expected = [
            Charge {
                name: "Lorem".to_string(),
                cost: 6.5,
                is_assigned: false,
            },
            Charge {
                name: "Ipsum".to_string(),
                cost: 7.5,
                is_assigned: false,
            },
            Charge {
                name: "Dolor Sit".to_string(),
                cost: 48.00,
                is_assigned: false,
            },
        ];
        assert_eq!(actual, expected);
    }
}
