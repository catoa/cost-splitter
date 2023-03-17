use crate::charges::Charge;
use crate::utils::get_values_by_delim;
use std::str::FromStr;
use std::vec::Vec;

#[derive(Debug, PartialEq, Clone)]
pub enum InputAction {
    AddCharge { charge: Charge },
    AssignCharge { name: String, indices: Vec<String> },
    PrintLastCharge,
    DeleteLastCharge,
    DeleteByIndex { indices: Vec<String> },
    Done,
}

#[derive(Debug, PartialEq)]
pub struct ParseActionError;

impl FromStr for InputAction {
    type Err = ParseActionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim() {
            "done" | "Done" => Ok(InputAction::Done),
            "delete" | "Delete" => Ok(InputAction::DeleteLastCharge),
            "last" | "Last" => Ok(InputAction::PrintLastCharge),
            _ => {
                if let Some((val, text)) = s.split_whitespace().collect::<Vec<&str>>().split_last()
                {
                    match text {
                        ["assign", name] => {
                            let val = get_values_by_delim(val.to_string(), ",");
                            Ok(InputAction::AssignCharge {
                                name: name.to_string(),
                                indices: val,
                            })
                        }
                        ["delete"] => {
                            let val = get_values_by_delim(val.to_string(), ",");
                            Ok(InputAction::DeleteByIndex { indices: val })
                        }
                        _ => {
                            let cost = val.parse::<f64>().unwrap_or(-1.0);
                            if cost > 0.0 {
                                let name = text.join(" ");
                                let charge = Charge::new(name, cost);
                                Ok(InputAction::AddCharge { charge })
                            } else {
                                Err(ParseActionError)
                            }
                        }
                    }
                } else {
                    Err(ParseActionError)
                }
            }
        }
    }
}
