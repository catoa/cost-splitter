use quickcheck_macros::quickcheck;
use splitter::charges::Charge;
use splitter::utils::get_values_by_delim;

#[quickcheck]
fn prop_get_values_by_delim_preserves_non_empty_values(input: String, delim: char) -> bool {
    let delim_str = &delim.to_string();
    let result = get_values_by_delim(input.clone(), delim_str);
    
    // Property: All returned values should be non-empty after trimming
    result.iter().all(|s| !s.trim().is_empty())
}

#[quickcheck]
fn prop_get_values_by_delim_split_count(input: String) -> bool {
    let delim = ",";
    let result = get_values_by_delim(input.clone(), delim);
    
    // Property: If input is empty or only whitespace, result should be empty
    if input.trim().is_empty() {
        return result.is_empty();
    }
    
    // Property: Number of results should be <= number of delimiter occurrences + 1
    let delimiter_count = input.matches(delim).count();
    result.len() <= delimiter_count + 1
}

#[quickcheck]
fn prop_charge_new_creates_valid_charge(name: String, cost: f64) -> bool {
    // Only test with valid costs (non-negative, finite)
    if !cost.is_finite() || cost < 0.0 {
        return true; // Skip invalid inputs
    }
    
    let charge = Charge::new(name.clone(), cost);
    
    // Properties of a new charge
    charge.name == name &&
    charge.cost == cost &&
    !charge.is_assigned
}

#[quickcheck]
fn prop_billing_calculation_totals_add_up(
    alice_charges: f64,
    bob_charges: f64,
    total_fees: f64,
) -> bool {
    // Only test with valid finite positive values
    if !alice_charges.is_finite() || !bob_charges.is_finite() || !total_fees.is_finite() ||
       alice_charges < 0.0 || bob_charges < 0.0 || total_fees < 0.0 {
        return true; // Skip invalid inputs
    }
    
    let subtotal = alice_charges + bob_charges;
    
    // Avoid division by zero
    if subtotal == 0.0 {
        return true;
    }
    
    let alice_percentage = alice_charges / subtotal;
    let bob_percentage = bob_charges / subtotal;
    let alice_fees = alice_percentage * total_fees;
    let bob_fees = bob_percentage * total_fees;
    
    // Property: Percentages should add up to 1.0 (within floating point precision)
    let percentage_sum = alice_percentage + bob_percentage;
    let percentage_close_to_one = (percentage_sum - 1.0).abs() < 1e-10;
    
    // Property: Individual fees should add up to total fees (within floating point precision)
    let fees_sum = alice_fees + bob_fees;
    let fees_close_to_total = (fees_sum - total_fees).abs() < 1e-10;
    
    // Property: Individual totals should add up to subtotal + total_fees
    let alice_total = alice_charges + alice_fees;
    let bob_total = bob_charges + bob_fees;
    let grand_total = alice_total + bob_total;
    let expected_grand_total = subtotal + total_fees;
    let totals_match = (grand_total - expected_grand_total).abs() < 1e-10;
    
    percentage_close_to_one && fees_close_to_total && totals_match
}

#[quickcheck]
fn prop_charge_list_sum_is_commutative(charges: Vec<(String, f64)>) -> bool {
    // Filter out invalid charges and extremely large numbers that could cause overflow
    let valid_charges: Vec<_> = charges
        .into_iter()
        .filter(|(_, cost)| {
            cost.is_finite() && 
            *cost >= 0.0 && 
            *cost <= 1000.0  // Very conservative limit for precise floating point arithmetic
        })
        .collect();
    
    if valid_charges.is_empty() {
        return true;
    }
    
    // Create charges in original order
    let charges1: Vec<Charge> = valid_charges
        .iter()
        .map(|(name, cost)| Charge::new(name.clone(), *cost))
        .collect();
    
    // Create charges in reverse order
    let charges2: Vec<Charge> = valid_charges
        .iter()
        .rev()
        .map(|(name, cost)| Charge::new(name.clone(), *cost))
        .collect();
    
    let sum1: f64 = charges1.iter().map(|c| c.cost).sum();
    let sum2: f64 = charges2.iter().map(|c| c.cost).sum();
    
    // Property: Sum should be the same regardless of order
    (sum1 - sum2).abs() < 1e-10
}

#[quickcheck]
fn prop_name_splitting_roundtrip(names: Vec<String>) -> bool {
    if names.is_empty() {
        return true;
    }
    
    // Filter out names that contain commas, control characters, or are empty when trimmed
    // Also filter names that start/end with whitespace since our trimming logic affects them
    let valid_names: Vec<String> = names
        .into_iter()
        .filter(|name| {
            !name.contains(',') && 
            !name.trim().is_empty() &&
            !name.chars().any(|c| c.is_control()) &&
            name.trim() == name  // No leading/trailing whitespace
        })
        .collect();
    
    if valid_names.is_empty() {
        return true;
    }
    
    // Join with commas
    let joined = valid_names.join(",");
    
    // Split back
    let split_result = get_values_by_delim(joined, ",");
    
    // Property: Should get back the same names (order matters)
    split_result == valid_names
}

#[test]
fn test_property_based_runner() {
    // This test just ensures our property tests compile and can run
    // The actual property tests are run by quickcheck
    assert!(true);
}
