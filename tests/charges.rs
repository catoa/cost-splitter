#[cfg(test)]
mod tests {
    use splitter::charges::{Charge, ParseChargeError};

    #[test]
    fn parse_charge_from_string_fails_with_invalid_data() {
        let charge_str = String::from("20 brisket");
        let parse_err = charge_str.parse::<Charge>();
        assert_eq!(parse_err, Err(ParseChargeError));
    }
    #[test]
    fn can_parse_charge_from_string() {
        let charge_str = String::from("brisket 20");
        let actual_charge = charge_str.parse::<Charge>().unwrap();
        assert_eq!(
            actual_charge,
            Charge {
                name: "brisket".to_owned(),
                cost: 20.0,
                is_assigned: false
            }
        )
    }
    #[test]
    fn can_parse_charge_from_string_with_numbers_on_each_side() {
        let charge_str = String::from("20 brisket 20");
        let actual_charge = charge_str.parse::<Charge>().unwrap();
        let expected_charge = Charge::new("20 brisket".to_owned(), 20.0);
        assert_eq!(actual_charge, expected_charge);
    }
}
