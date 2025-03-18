use std::collections::HashMap;
use std::error::Error;
use serde_cadence::{CadenceValue, FromCadenceValue, ToCadenceValue};


fn main() -> Result<(), Box<dyn Error>> {
    // The Cadence-JSON string provided
    let cadence_json = r#"{"value":[{"key":{"value":"banana","type":"String"},"value":{"value":"10","type":"Int"}},{"key":{"value":"cherry","type":"String"},"value":{"value":"15","type":"Int"}},{"key":{"value":"apple","type":"String"},"value":{"value":"5","type":"Int"}}],"type":"Dictionary"}"#;

    // Parse the JSON string into a CadenceValue
    let cadence_value: CadenceValue = serde_json::from_str(cadence_json)?;
    println!("Original Cadence-JSON Dictionary:\n{}\n", cadence_json);

    // Convert CadenceValue to our structured FruitInventory
    let fruit_inventory = HashMap::<String, i32>::from_cadence_value(&cadence_value)?;
    println!("Deserialized to Rust struct:\n{:#?}\n", fruit_inventory);

    // Let's modify the inventory
    let modified_inventory = fruit_inventory;

    println!("Modified inventory:\n{:#?}\n", modified_inventory);

    // Convert back to CadenceValue
    let new_cadence_value = modified_inventory.to_cadence_value()?;

    // Serialize to Cadence-JSON
    let new_json = serde_json::to_string_pretty(&new_cadence_value)?;
    println!("Serialized back to Cadence-JSON:\n{}\n", new_json);

    // Let's test that we can round-trip our data
    let round_trip_value: CadenceValue = serde_json::from_str(&new_json)?;
    let round_trip_inventory = HashMap::from_cadence_value(&round_trip_value)?;

    // Ensure our data made it through correctly
    assert_eq!(modified_inventory, round_trip_inventory, "Round-trip test failed!");
    println!("Round-trip test successful!");

    Ok(())
}