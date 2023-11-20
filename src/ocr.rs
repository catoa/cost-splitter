use crate::actions::InputAction;
use crate::billing::print_charge_breakdown;
use crate::charges::Charge;
use crate::utils::read_from_stdin;
use anyhow::Result;
use aws_config::load_from_env;
use aws_sdk_textract::client::Client as AWSClient;
use aws_sdk_textract::error::DisplayErrorContext;
use aws_sdk_textract::primitives::Blob;
use aws_sdk_textract::types::Document;
use colored::Colorize;
use std::collections::HashMap;
use std::path::PathBuf;

async fn get_aws_client() -> AWSClient {
    let config = load_from_env().await;
    AWSClient::new(&config)
}

fn build_document(path: PathBuf) -> Document {
    let file = std::fs::read(path).expect("file exists and is readable");
    let blob = Blob::new(file);
    Document::builder().bytes(blob).build()
}

async fn get_analyze_expenses_result(receipt_path: PathBuf) -> Result<Vec<Charge>> {
    let client = get_aws_client().await;
    let document = build_document(receipt_path);
    let analyze_expense_output = client.analyze_expense().document(document).send().await;
    match analyze_expense_output {
        Ok(expense_output) => {
            let charges = expense_output
                .expense_documents()
                .iter()
                .cloned()
                .flat_map(|doc| doc.line_item_groups.unwrap())
                .flat_map(|group| group.line_items.unwrap())
                .flat_map(|line_item| line_item.line_item_expense_fields.unwrap())
                .filter_map(|expense| {
                    if expense.r#type.unwrap().text.unwrap() == "EXPENSE_ROW" {
                        match expense
                            .value_detection
                            .unwrap()
                            .text
                            .unwrap()
                            .parse::<InputAction>()
                        {
                            Ok(InputAction::AddCharge { charge }) => Some(charge),
                            _ => None,
                        }
                    } else {
                        None
                    }
                })
                .collect::<Vec<Charge>>();
            Ok(charges)
        }
        Err(err) => {
            println!("Could not analyze document {}", DisplayErrorContext(&err));
            panic!("Error parsing document");
        }
    }
}

pub async fn process_receipt(receipt_path: PathBuf) {
    let mut should_print_prompt = true;
    let mut approved_charges = get_analyze_expenses_result(receipt_path).await.unwrap();
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
        let action = input.parse::<InputAction>();
        match action {
            Ok(InputAction::Done) => {
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
            Ok(InputAction::DeleteByIndex { indices }) => {
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
            Ok(InputAction::AssignCharge { name, indices }) => {
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
                        .or_default()
                        .push(charge.to_owned());
                }
                input.clear();
            }
            Ok(InputAction::AddCharge { charge }) => {
                approved_charges.push(charge);
                input.clear();
            }
            Ok(InputAction::DeleteLastCharge) | Ok(InputAction::PrintLastCharge) => {
                println!("unsupported action")
            }
            Err(e) => {
                println!("Unrecognized input: {e:?}")
            }
        }
    }
}
