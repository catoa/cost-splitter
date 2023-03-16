use crate::actions::InputAction;
use crate::billing::print_charge_breakdown;
use crate::charges::Charge;
use crate::utils::read_from_stdin;
use aws_config::load_from_env;
use aws_sdk_textract::client::Client as AWSClient;
use aws_sdk_textract::model::Document;
use aws_sdk_textract::types::Blob;
use colored::Colorize;
use std::collections::HashMap;
use std::path::PathBuf;

async fn get_analyze_expenses_result(receipt_path: PathBuf) -> Vec<Charge> {
    let config = load_from_env().await;
    let client = AWSClient::new(&config);
    let file = std::fs::read(receipt_path).unwrap();
    let blob = Blob::new(file);
    let document = Document::builder().set_bytes(Some(blob)).build();

    let result = client
        .analyze_expense()
        .document(document)
        .send()
        .await
        .unwrap();

    let expense_docs = result.expense_documents().unwrap();
    expense_docs
        .iter()
        .map(|doc| doc.line_item_groups.as_ref())
        .flat_map(|line_item_groups| line_item_groups.unwrap())
        .flat_map(|line_item_group| line_item_group.line_items.as_ref().unwrap())
        .flat_map(|line_item_field| line_item_field.line_item_expense_fields.as_ref().unwrap())
        .filter(|expense_field| {
            expense_field
                .r#type
                .as_ref()
                .unwrap()
                .text
                .as_ref()
                .unwrap()
                == "EXPENSE_ROW"
        })
        .filter_map(|expense_row| {
            match InputAction::parse(
                expense_row
                    .value_detection
                    .as_ref()
                    .unwrap()
                    .text
                    .as_ref()
                    .unwrap(),
            ) {
                InputAction::AddCharge { charge } => Some(charge),
                _ => None,
            }
        })
        .collect::<Vec<Charge>>()
}

pub fn get_charges_from_text(s: String) -> Vec<Charge> {
    s.lines()
        .filter(|line| !line.is_empty())
        .filter_map(|line| {
            if let InputAction::AddCharge { charge } = InputAction::parse(line) {
                Some(charge)
            } else {
                None
            }
        })
        .collect::<Vec<Charge>>()
}

fn get_leptess_result(receipt_path: PathBuf) -> Vec<Charge> {
    let mut lt = leptess::LepTess::new(None, "eng").unwrap();
    match lt.set_image(receipt_path) {
        Ok(_) => {
            let image_text = lt.get_utf8_text().unwrap();
            get_charges_from_text(image_text)
        }
        Err(_) => panic!("Unable to parse image content"),
    }
}

async fn get_charges_from_receipt(receipt_path: PathBuf, use_textract: bool) -> Vec<Charge> {
    if use_textract {
        get_analyze_expenses_result(receipt_path).await
    } else {
        get_leptess_result(receipt_path)
    }
}

pub async fn process_receipt(receipt_path: PathBuf, use_textract: bool) {
    let mut should_print_prompt = true;
    let mut approved_charges = get_charges_from_receipt(receipt_path, use_textract).await;
    let mut charges_map: HashMap<String, Vec<Charge>> = HashMap::new();

    println!("Parsed the following charges:");
    let mut input = String::new();
    loop {
        for (idx, approved_charge) in approved_charges.iter().enumerate() {
            println!("#{}: {:?}", idx + 1, approved_charge);
        }

        if should_print_prompt {
            println!("Need to add or delete any items?");
            should_print_prompt = false;
        }
        read_from_stdin(&mut input, "Unable to parse message");
        let action = InputAction::parse(&input);
        match action {
            InputAction::Done => {
                input.clear();
                let unapproved_charges = approved_charges
                    .iter()
                    .filter(|charge| !charge.is_assigned)
                    .collect::<Vec<&Charge>>();
                if !unapproved_charges.is_empty() {
                    eprintln!("There are still unassigned charges");
                    continue;
                } else {
                    let approved_subtotal = approved_charges.iter().map(|charge| charge.cost).sum();
                    print_charge_breakdown(&mut input, &charges_map, approved_subtotal);
                    break;
                }
            }
            InputAction::DeleteByIndex { indices } => {
                let mut remove_indices = indices
                    .iter()
                    .map(|idx| idx.parse::<usize>().ok().unwrap())
                    .collect::<Vec<usize>>(); // Sort indices in reverse order so that they can be removed
                                              // properly from the vector
                remove_indices.sort_by(|a, b| b.cmp(a));

                for index in remove_indices.iter() {
                    let removed_charge = approved_charges.remove(index - 1);

                    let removed_msg =
                        format!("Removed: {} - {}", removed_charge.name, removed_charge.cost);

                    println!("{}", removed_msg.red());
                    input.clear();
                }
            }
            InputAction::AssignCharge { name, indices } => {
                let assign_indices = indices
                    .iter()
                    .map(|idx| idx.parse::<usize>().ok().unwrap())
                    .collect::<Vec<usize>>();

                for index in assign_indices.iter() {
                    let charge = approved_charges.get_mut(index - 1).unwrap();
                    charge.is_assigned = true;

                    let assigned_msg = format!("Assigned: {} - {}", charge.name, name);

                    println!("{}", assigned_msg.blue());
                    input.clear();

                    charges_map
                        .entry(name.to_string())
                        .or_insert_with(Vec::default)
                        .push(charge.to_owned());
                }
                input.clear();
            }
            InputAction::AddCharge { charge } => {
                approved_charges.push(charge);
                input.clear();
            }
            _ => {
                println!("unrecognized input")
            }
        }
    }
}

