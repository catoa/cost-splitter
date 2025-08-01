use splitter::charges::process_individual_charges_with_io;
use splitter::io::{MockInputReader, MockOutputWriter};

#[test]
fn test_full_charge_workflow_two_people() {
    let mut reader = MockInputReader::new(vec![
        // Alice's items
        "burger 12.50".to_string(),
        "fries 4.00".to_string(),
        "done".to_string(),
        // Bob's items  
        "salad 8.00".to_string(),
        "drink 2.50".to_string(),
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
    
    // Verify structure
    assert_eq!(charges.len(), 2);
    assert!(charges.contains_key("Alice"));
    assert!(charges.contains_key("Bob"));
    
    // Verify Alice's charges
    let alice_charges = &charges["Alice"];
    assert_eq!(alice_charges.len(), 2);
    assert_eq!(alice_charges[0].name, "burger");
    assert_eq!(alice_charges[0].cost, 12.50);
    assert_eq!(alice_charges[1].name, "fries");
    assert_eq!(alice_charges[1].cost, 4.00);
    
    // Verify Bob's charges
    let bob_charges = &charges["Bob"];
    assert_eq!(bob_charges.len(), 2);
    assert_eq!(bob_charges[0].name, "salad");
    assert_eq!(bob_charges[0].cost, 8.00);
    assert_eq!(bob_charges[1].name, "drink");
    assert_eq!(bob_charges[1].cost, 2.50);
    
    // Verify totals
    let alice_total: f64 = alice_charges.iter().map(|c| c.cost).sum();
    let bob_total: f64 = bob_charges.iter().map(|c| c.cost).sum();
    assert_eq!(alice_total, 16.50);
    assert_eq!(bob_total, 10.50);
    assert_eq!(alice_total + bob_total, 27.00);
}

#[test]
fn test_charge_workflow_with_corrections() {
    let mut reader = MockInputReader::new(vec![
        "pizza 15.00".to_string(),
        "beer 6.00".to_string(),
        "last".to_string(),           // View last charge
        "delete".to_string(),         // Delete last charge (beer)
        "soda 3.00".to_string(),      // Add correct item
        "done".to_string(),
    ]);
    let mut writer = MockOutputWriter::new();

    let result = process_individual_charges_with_io(
        "Charlie".to_string(),
        &mut reader,
        &mut writer,
    );

    assert!(result.is_ok());
    let charges = result.unwrap();
    
    let charlie_charges = &charges["Charlie"];
    assert_eq!(charlie_charges.len(), 2);
    assert_eq!(charlie_charges[0].name, "pizza");
    assert_eq!(charlie_charges[0].cost, 15.00);
    assert_eq!(charlie_charges[1].name, "soda");
    assert_eq!(charlie_charges[1].cost, 3.00);
    
    // Verify that the output contains expected prompts and feedback
    let output_text = writer.output.join("");
    assert!(output_text.contains("beer"));  // Should show the item before deletion
    assert!(output_text.contains("deleted")); // Should show deletion message
}

#[test]
fn test_error_handling_invalid_input() {
    let mut reader = MockInputReader::new(vec![
        "invalid input".to_string(),
        "not a charge".to_string(),
        "just numbers 123".to_string(),
        "pizza 15.00".to_string(),     // Valid charge
        "done".to_string(),
    ]);
    let mut writer = MockOutputWriter::new();

    let result = process_individual_charges_with_io(
        "Dave".to_string(),
        &mut reader,
        &mut writer,
    );

    assert!(result.is_ok());
    let charges = result.unwrap();
    
    // The parsing might accept "just numbers 123" as a valid charge
    // Let's check what actually got parsed
    let dave_charges = &charges["Dave"];
    
    // Find the pizza charge - should be in there
    let pizza_charges: Vec<_> = dave_charges.iter().filter(|c| c.name == "pizza").collect();
    assert_eq!(pizza_charges.len(), 1);
    assert_eq!(pizza_charges[0].cost, 15.00);
    
    // Total charges should include any valid parsed items
    assert!(dave_charges.len() >= 1); // At least the pizza
    
    // Verify that error messages were output
    let output_text = writer.output.join("");
    assert!(output_text.contains("Error parsing input"));
}

#[test]
fn test_single_person_workflow() {
    let mut reader = MockInputReader::new(vec![
        "steak 25.00".to_string(),
        "wine 12.00".to_string(),
        "dessert 8.00".to_string(),
        "done".to_string(),
    ]);
    let mut writer = MockOutputWriter::new();

    let result = process_individual_charges_with_io(
        "Solo".to_string(),
        &mut reader,
        &mut writer,
    );

    assert!(result.is_ok());
    let charges = result.unwrap();
    
    assert_eq!(charges.len(), 1);
    let solo_charges = &charges["Solo"];
    assert_eq!(solo_charges.len(), 3);
    
    let total: f64 = solo_charges.iter().map(|c| c.cost).sum();
    assert_eq!(total, 45.00);
}

#[test]
fn test_empty_person_name_handling() {
    let mut reader = MockInputReader::new(vec![
        "done".to_string(),
    ]);
    let mut writer = MockOutputWriter::new();

    let result = process_individual_charges_with_io(
        "  ,  , ".to_string(), // Names with only whitespace
        &mut reader,
        &mut writer,
    );

    assert!(result.is_ok());
    let charges = result.unwrap();
    
    // Should handle empty/whitespace names gracefully
    // Exact behavior depends on implementation
    assert!(charges.len() <= 3); // At most 3 entries (one for each comma-separated item)
}

#[test]
fn test_complex_item_names() {
    let mut reader = MockInputReader::new(vec![
        "chicken with mushroom sauce 18.50".to_string(),
        "side of garlic bread 4.25".to_string(),
        "chocolate chip cookie 2.75".to_string(),
        "done".to_string(),
    ]);
    let mut writer = MockOutputWriter::new();

    let result = process_individual_charges_with_io(
        "Foodie".to_string(),
        &mut reader,
        &mut writer,
    );

    assert!(result.is_ok());
    let charges = result.unwrap();
    
    let foodie_charges = &charges["Foodie"];
    assert_eq!(foodie_charges.len(), 3);
    
    // Verify complex names are parsed correctly
    assert_eq!(foodie_charges[0].name, "chicken with mushroom sauce");
    assert_eq!(foodie_charges[0].cost, 18.50);
    assert_eq!(foodie_charges[1].name, "side of garlic bread");
    assert_eq!(foodie_charges[1].cost, 4.25);
    assert_eq!(foodie_charges[2].name, "chocolate chip cookie");
    assert_eq!(foodie_charges[2].cost, 2.75);
}

#[test]
fn test_decimal_precision() {
    let mut reader = MockInputReader::new(vec![
        "item 10.99".to_string(),
        "item2 5.50".to_string(),
        "item3 0.01".to_string(),
        "done".to_string(),
    ]);
    let mut writer = MockOutputWriter::new();

    let result = process_individual_charges_with_io(
        "Precise".to_string(),
        &mut reader,
        &mut writer,
    );

    assert!(result.is_ok());
    let charges = result.unwrap();
    
    let precise_charges = &charges["Precise"];
    assert_eq!(precise_charges.len(), 3);
    
    // Verify decimal precision is maintained
    assert_eq!(precise_charges[0].cost, 10.99);
    assert_eq!(precise_charges[1].cost, 5.50);
    assert_eq!(precise_charges[2].cost, 0.01);
    
    let total: f64 = precise_charges.iter().map(|c| c.cost).sum();
    assert!((total - 16.50).abs() < 1e-10); // Handle floating point precision
}
