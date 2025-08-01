use splitter::charges::{process_individual_charges_with_io, Charge};
use splitter::io::{MockInputReader, MockOutputWriter};

#[test]
fn test_process_individual_charges_single_person_single_item() {
    let mut reader = MockInputReader::new(vec![
        "pizza 15".to_string(),
        "done".to_string(),
    ]);
    let mut writer = MockOutputWriter::new();

    let result = process_individual_charges_with_io(
        "Alice".to_string(),
        &mut reader,
        &mut writer,
    );

    assert!(result.is_ok());
    let charges = result.unwrap();
    assert_eq!(charges.len(), 1);
    assert!(charges.contains_key("Alice"));
    
    let alice_charges = &charges["Alice"];
    assert_eq!(alice_charges.len(), 1);
    assert_eq!(alice_charges[0].name, "pizza");
    assert_eq!(alice_charges[0].cost, 15.0);
}

#[test]
fn test_process_individual_charges_multiple_people() {
    let mut reader = MockInputReader::new(vec![
        "burger 12".to_string(),
        "fries 5".to_string(),
        "done".to_string(),
        "salad 8".to_string(),
        "done".to_string(),
    ]);
    let mut writer = MockOutputWriter::new();

    let result = process_individual_charges_with_io(
        "Alice,Bob".to_string(),
        &mut reader,
        &mut writer,
    );

    assert!(result.is_ok());
    let charges = result.unwrap();
    assert_eq!(charges.len(), 2);
    
    let alice_charges = &charges["Alice"];
    assert_eq!(alice_charges.len(), 2);
    assert_eq!(alice_charges[0].name, "burger");
    assert_eq!(alice_charges[1].name, "fries");
    
    let bob_charges = &charges["Bob"];
    assert_eq!(bob_charges.len(), 1);
    assert_eq!(bob_charges[0].name, "salad");
}

#[test]
fn test_process_individual_charges_with_delete() {
    let mut reader = MockInputReader::new(vec![
        "pizza 15".to_string(),
        "burger 12".to_string(),
        "delete".to_string(),
        "done".to_string(),
    ]);
    let mut writer = MockOutputWriter::new();

    let result = process_individual_charges_with_io(
        "Alice".to_string(),
        &mut reader,
        &mut writer,
    );

    assert!(result.is_ok());
    let charges = result.unwrap();
    let alice_charges = &charges["Alice"];
    assert_eq!(alice_charges.len(), 1);
    assert_eq!(alice_charges[0].name, "pizza");
}

#[test]
fn test_process_individual_charges_with_print_last() {
    let mut reader = MockInputReader::new(vec![
        "pizza 15".to_string(),
        "last".to_string(),
        "done".to_string(),
    ]);
    let mut writer = MockOutputWriter::new();

    let result = process_individual_charges_with_io(
        "Alice".to_string(),
        &mut reader,
        &mut writer,
    );

    assert!(result.is_ok());
    // Check that the output contains information about the last item
    let output_text = writer.output.join("");
    assert!(output_text.contains("pizza"));
    assert!(output_text.contains("15"));
}

#[test]
fn test_charge_new() {
    let charge = Charge::new("test item".to_string(), 10.5);
    assert_eq!(charge.name, "test item");
    assert_eq!(charge.cost, 10.5);
    assert!(!charge.is_assigned);
}

#[test]
fn test_process_individual_charges_invalid_input() {
    let mut reader = MockInputReader::new(vec![
        "invalid input".to_string(),
        "pizza 15".to_string(),
        "done".to_string(),
    ]);
    let mut writer = MockOutputWriter::new();

    let result = process_individual_charges_with_io(
        "Alice".to_string(),
        &mut reader,
        &mut writer,
    );

    assert!(result.is_ok());
    let charges = result.unwrap();
    let alice_charges = &charges["Alice"];
    assert_eq!(alice_charges.len(), 1);
    assert_eq!(alice_charges[0].name, "pizza");
}

#[test]
fn test_process_individual_charges_empty_person_name() {
    let mut reader = MockInputReader::new(vec![
        "done".to_string(),
    ]);
    let mut writer = MockOutputWriter::new();

    let result = process_individual_charges_with_io(
        "".to_string(),
        &mut reader,
        &mut writer,
    );

    assert!(result.is_ok());
    let charges = result.unwrap();
    assert_eq!(charges.len(), 0);
}
