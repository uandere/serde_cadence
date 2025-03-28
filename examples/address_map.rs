use serde_cadence::{CadenceValue, Error, FromCadenceValue, ToCadenceValue};
use std::collections::HashMap;

fn main() -> Result<(), Error> {
    // Sample Cadence-JSON representing a dictionary with Address keys and String values
    let cadence_json = r#"{
        "type": "Dictionary",
        "value": [
            {
                "key": {
                    "type": "Address",
                    "value": "0x1234567890abcdef"
                },
                "value": {
                    "type": "String",
                    "value": "Alice's Account"
                }
            },
            {
                "key": {
                    "type": "Address",
                    "value": "0xf1e2d3c4b5a69780"
                },
                "value": {
                    "type": "String",
                    "value": "Bob's Account"
                }
            },
            {
                "key": {
                    "type": "Address",
                    "value": "0xa1b2c3d4e5f67890"
                },
                "value": {
                    "type": "String",
                    "value": "Charlie's Account"
                }
            }
        ]
    }"#;

    // Parse the JSON string into a CadenceValue
    let cadence_value: CadenceValue = serde_json::from_str(cadence_json)?;
    println!("Original Cadence-JSON Address Dictionary:\n{}\n", cadence_json);

    // Convert CadenceValue to our HashMap<String, String>
    // We use String keys for addresses since we don't have a dedicated Address type in Rust
    let address_map = HashMap::<String, String>::from_cadence_value(&cadence_value)?;
    println!("Deserialized to Rust HashMap<String, String>:\n{:#?}\n", address_map);

    // Let's modify the map
    let mut modified_map = address_map.clone();
    modified_map.insert("0xdeadbeef00000000".to_string(), "Dave's Account".to_string());
    modified_map.remove("0xf1e2d3c4b5a69780");
    println!("Modified HashMap:\n{:#?}\n", modified_map);

    // Convert back to CadenceValue
    let new_cadence_value = modified_map.to_cadence_value()?;

    // Serialize to Cadence-JSON
    let new_json = serde_json::to_string_pretty(&new_cadence_value)?;
    println!("Serialized back to Cadence-JSON:\n{}\n", new_json);

    // Let's test that we can round-trip our data
    let round_trip_value: CadenceValue = serde_json::from_str(&new_json)?;
    let round_trip_map = HashMap::<String, String>::from_cadence_value(&round_trip_value)?;

    // Ensure our data made it through correctly
    assert_eq!(
        modified_map, round_trip_map,
        "Round-trip test failed!"
    );
    println!("Round-trip test successful!");

    Ok(())
}