#[cfg(test)]
mod tests {
    use splitter::{actions::InputAction, charges::Charge};

    #[test]
    fn test_parse_input_action() {
        assert_eq!(InputAction::parse("done"), InputAction::Done);
        assert_eq!(
            InputAction::parse(" "),
            InputAction::Invalid {
                msg: String::from("Supplied string did not match any pattern"),
            }
        );
        assert_eq!(
            InputAction::parse(""),
            InputAction::Invalid {
                msg: String::from("Supplied string did not match any pattern"),
            }
        );
        assert_eq!(InputAction::parse("last"), InputAction::PrintLastCharge);
        assert_eq!(InputAction::parse("delete"), InputAction::DeleteLastCharge);

        assert_eq!(
            InputAction::parse("Steak Sandwich 20"),
            InputAction::AddCharge {
                charge: Charge {
                    name: String::from("Steak Sandwich"),
                    cost: 20.00,
                    is_assigned: false
                }
            }
        );
        assert_eq!(
            InputAction::parse("social smoker 8"),
            InputAction::AddCharge {
                charge: Charge {
                    name: String::from("social smoker"),
                    cost: 8.00,
                    is_assigned: false
                }
            }
        );

        assert_eq!(
            InputAction::parse("we don't have a price"),
            InputAction::Invalid {
                msg: String::from("Value could not be parsed from string"),
            }
        );

        assert_eq!(
            InputAction::parse("48"),
            InputAction::Invalid {
                msg: String::from("Could not parse line item because name was not supplied"),
            }
        );
    }
}
