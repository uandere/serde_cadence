// examples/basic.rs
use serde_cadence::{CadenceValue, Result, ToCadenceValue};
use serde::{Deserialize, Serialize};

// Define a Rust struct that we want to serialize to Cadence-JSON
#[derive(Debug, Serialize, Deserialize)]
struct Person {
    name: String,
    age: u8,
    is_active: bool,
    tags: Vec<String>,
}

// Implement manual conversion to CadenceValue
impl ToCadenceValue for Person {
    fn to_cadence_value(&self) -> Result<CadenceValue> {
        // Create a Struct CadenceValue with appropriate fields
        use serde_cadence::{CompositeField, CompositeValue};

        let name_field = CompositeField {
            name: "name".to_string(),
            value: self.name.to_cadence_value()?,
        };

        let age_field = CompositeField {
            name: "age".to_string(),
            value: CadenceValue::UInt8 {
                value: self.age.to_string(),
            },
        };

        let is_active_field = CompositeField {
            name: "isActive".to_string(),
            value: self.is_active.to_cadence_value()?,
        };

        let mut tag_values = Vec::new();
        for tag in &self.tags {
            tag_values.push(tag.to_cadence_value()?);
        }

        let tags_field = CompositeField {
            name: "tags".to_string(),
            value: CadenceValue::Array { value: tag_values },
        };

        Ok(CadenceValue::Struct {
            value: CompositeValue {
                id: "Person".to_string(),
                fields: vec![name_field, age_field, is_active_field, tags_field],
            },
        })
    }
}

fn main() -> Result<()> {
    // Create a Person instance
    let person = Person {
        name: "Alice".to_string(),
        age: 30,
        is_active: true,
        tags: vec!["developer".to_string(), "rust".to_string()],
    };

    // Convert to Cadence-JSON
    let cadence_value = person.to_cadence_value()?;

    // Serialize to JSON string
    let json = serde_json::to_string_pretty(&cadence_value)?;
    println!("Cadence-JSON representation:\n{}", json);

    Ok(())
}
