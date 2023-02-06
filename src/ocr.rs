use crate::actions::InputAction;
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

pub fn process_receipt(receipt_path: PathBuf) {
    let image_text = get_text_from_receipt(receipt_path);

    let mut approved_charges: Vec<Charge> = image_text
        .lines()
        .filter(|line| !line.is_empty())
        .filter_map(|line| {
            if let InputAction::AddCharge { charge } = InputAction::parse(line) {
                return Some(charge);
            } else {
                return None;
            }
        })
        .collect::<Vec<Charge>>();

    println!("Parsed the following charges:");
    let mut input = String::new();
    loop {
        // FIXME: How do we avoid cloning the vector?
        for (idx, approved_charge) in approved_charges.clone().into_iter().enumerate() {
            println!("#{}: {:?}", idx + 1, approved_charge);
        }
        println!("Need to delete any items?");
        read_from_stdin(&mut input, "Unable to parse message");
        let action = InputAction::parse(&input);
        match action {
            InputAction::Done => {
                println!("approved_charges: {:?}", approved_charges);

                input.clear();
                break;
            }
            InputAction::DeleteByIndex { indices } => {
                let mut remove_indices = indices
                    .iter()
                    .map(|idx| idx.parse::<usize>().ok().unwrap())
                    .into_iter()
                    .collect::<Vec<usize>>();

                // Sort indices in reverse order so that they can be removed
                // properly from the vector
                remove_indices.sort_by(|a, b| b.cmp(a));

                for index in remove_indices.iter() {
                    let removed_charge = approved_charges.remove(index - 1);

                    let removed_msg =
                        format!("Removed: {} - {}", removed_charge.name, removed_charge.cost);

                    println!("{}", removed_msg.red());
                }
            }
            InputAction::AssignCharge { name, index } => println!("assign {} to {}", name, index),
            _ => println!("unrecognized input"),
        }
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn read_test_receipt() {
        let _expected = vec![
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
