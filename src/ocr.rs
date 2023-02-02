use crate::actions;
use crate::actions::Action;
use crate::charges::Charge;
use crate::utils::read_from_stdin;
use colored::Colorize;
use std::path::PathBuf;

fn get_text_from_receipt(receipt_path: PathBuf) -> String {
    let mut lt = leptess::LepTess::new(None, "eng").unwrap();
    match lt.set_image(receipt_path) {
        Ok(_) => lt.get_utf8_text().unwrap(),
        Err(_) => panic!("Unable to parse image content"),
    }
}

fn handle_amend_action(input: &mut String, print_message: bool) -> Charge {
    if print_message {
        println!("{}", String::from("Write the name of the item followed by its price. e.g. ribeye steak 20 for \"Ribeye Steak for $20\"").yellow());
    }
    loop {
        println!("parsing: {}", input);

        // println!("parsed: {:?}", action);

        println!("Type the correct name of the item and its cost");
        read_from_stdin(input, "Could not parse input");
        let action = actions::Action::parse_input(&input);
        match action {
            Action::AddCharge { charge } => {
                input.clear();
                return charge;
            }
            Action::Invalid { msg } => {
                println!("invalid: {}, received: {}", msg, input);

                return handle_amend_action(input, print_message);
            }
            _ => {
                println!("exiting");
                continue;
            }
        }
    }
}

// enum OcrAction {
//     Delete { index: usize },
// }

pub fn process_receipt(receipt_path: PathBuf) {
    let image_text = get_text_from_receipt(receipt_path);

    // let mut should_print_amend_message = true;
    let mut approved_charges: Vec<Charge> = Vec::new();

    for line in image_text.lines() {
        if line.is_empty() {
            continue;
        }

        if let Action::AddCharge { charge } = Action::parse_input(line) {
            approved_charges.push(charge);
            // println!("Include this charge, [y]es or [n]o? To amend, type 'a'");
            // println!("Found: {} with a cost of ${}", charge.name, charge.cost);

            // read_from_stdin(&mut input, "Could not parse input");
            // input = input.trim().to_lowercase();
            // if input == "y" || input == "yes" {
            //     println!("adding item");
            //     approved_charges.push(charge);
            // } else if input == "n" || input == "no" {
            //     continue;

            // // } else if input == "a" {
            // //     println!("amending transaction");
            // //     input.clear();
            // //     let charge: Charge = handle_amend_action(&mut input, should_print_amend_message);
            // //     should_print_amend_message = false;
            // //     approved_charges.push(charge);
            // } else if input == "done" {
            //     break;
            // }
            // println!("Current input buffer {}", input);
        }
    }

    println!("Parsed the following charges:");
    for (idx, approved_charge) in approved_charges.clone().into_iter().enumerate() {
        println!("#{}: {:?}", idx + 1, approved_charge);
    }

    let mut input = String::new();
    // println!("{}", image_text);
}

#[cfg(test)]
mod tests {

    #[test]
    fn read_test_receipt() {
        let expected = vec![
            "Lorem 6.50",
            "Ipsum 7.50",
            "Dolor Sit 48.00",
            "Amet 9.30",
            "Consectetur 11.90",
            "Adipiscing Elit 1.20",
            "Sed Do 0.40",
        ];
    }
}
