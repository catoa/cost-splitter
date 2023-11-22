#[cfg(test)]
mod tests {
    use splitter::{actions::InputAction, actions::ParseInputActionError, charges::Charge};

    #[test]
    fn test_parse_input_action() {
        assert_eq!("done".parse::<InputAction>().unwrap(), InputAction::Done);
        assert_eq!(" ".parse::<InputAction>(), Err(ParseInputActionError));
        assert_eq!("".parse::<InputAction>(), Err(ParseInputActionError));
        assert_eq!(
            "last".parse::<InputAction>().unwrap(),
            InputAction::PrintLastCharge
        );
        assert_eq!(
            "delete".parse::<InputAction>().unwrap(),
            InputAction::DeleteLastCharge
        );

        assert_eq!(
            "Steak Sandwich 20".parse::<InputAction>().unwrap(),
            InputAction::AddCharge {
                charge: Charge {
                    name: String::from("Steak Sandwich"),
                    cost: 20.00,
                    is_assigned: false
                }
            }
        );
        assert_eq!(
            "social smoker 8".parse::<InputAction>().unwrap(),
            InputAction::AddCharge {
                charge: Charge {
                    name: String::from("social smoker"),
                    cost: 8.00,
                    is_assigned: false
                }
            }
        );

        assert_eq!(
            "we don't have a price".parse::<InputAction>(),
            Err(ParseInputActionError)
        );

        assert_eq!("48".parse::<InputAction>(), Err(ParseInputActionError));
    }
}
