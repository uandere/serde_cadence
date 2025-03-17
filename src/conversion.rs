// src/conversion.rs - Updated implementation

use crate::{CadenceValue, CompositeField, CompositeValue, DictionaryEntry, Error, Result};
use serde::{Deserialize};
use serde_json::Value;

/// Convert a serde_json::Value to a CadenceValue
pub fn value_to_cadence_value(value: &Value) -> Result<CadenceValue> {
    match value {
        Value::Null => Ok(CadenceValue::Optional { value: None }),
        Value::Bool(b) => Ok(CadenceValue::Bool { value: *b }),
        Value::Number(n) => {
            if n.is_i64() {
                Ok(CadenceValue::Int { value: n.to_string() })
            } else if n.is_u64() {
                Ok(CadenceValue::UInt { value: n.to_string() })
            } else {
                // For floating point, we use Fix64
                Ok(CadenceValue::Fix64 { value: n.to_string() })
            }
        }
        Value::String(s) => Ok(CadenceValue::String { value: s.clone() }),
        Value::Array(arr) => {
            let mut cadence_values = Vec::with_capacity(arr.len());
            for item in arr {
                cadence_values.push(value_to_cadence_value(item)?);
            }
            Ok(CadenceValue::Array { value: cadence_values })
        }
        Value::Object(obj) => {
            // Check if this is a structured Cadence value with a "type" field
            if let Some(Value::String(type_name)) = obj.get("type") {
                if let Some(value_field) = obj.get("value") {
                    // This is a pre-structured Cadence value
                    return parse_structured_cadence_value(type_name, value_field);
                }
            }

            // Otherwise, treat it as a Dictionary
            let mut entries = Vec::new();
            for (key, value) in obj {
                let key_value = CadenceValue::String { value: key.clone() };
                let value_value = value_to_cadence_value(value)?;
                entries.push(DictionaryEntry {
                    key: key_value,
                    value: value_value,
                });
            }

            Ok(CadenceValue::Dictionary { value: entries })
        }
    }
}

/// Parse a pre-structured Cadence value
fn parse_structured_cadence_value(type_name: &str, value: &Value) -> Result<CadenceValue> {
    match type_name {
        "Void" => Ok(CadenceValue::Void {}),

        "Optional" => {
            if value.is_null() {
                Ok(CadenceValue::Optional { value: None })
            } else {
                let inner_value = value_to_cadence_value(value)?;
                Ok(CadenceValue::Optional {
                    value: Some(Box::new(inner_value)),
                })
            }
        }

        "Bool" => {
            if let Value::Object(obj) = value {
                if let Some(Value::Bool(b)) = obj.get("value") {
                    Ok(CadenceValue::Bool { value: *b })
                } else {
                    Err(Error::InvalidCadenceValue("Bool value must be a boolean".to_string()))
                }
            } else {
                Err(Error::InvalidCadenceValue("Bool value must be an object".to_string()))
            }
        }

        "String" => {
            if let Value::Object(obj) = value {
                if let Some(Value::String(s)) = obj.get("value") {
                    Ok(CadenceValue::String { value: s.clone() })
                } else {
                    Err(Error::InvalidCadenceValue("String value must be a string".to_string()))
                }
            } else {
                Err(Error::InvalidCadenceValue("String value must be an object".to_string()))
            }
        }

        "Address" => {
            if let Value::Object(obj) = value {
                if let Some(Value::String(s)) = obj.get("value") {
                    Ok(CadenceValue::Address { value: s.clone() })
                } else {
                    Err(Error::InvalidCadenceValue("Address value must be a string".to_string()))
                }
            } else {
                Err(Error::InvalidCadenceValue("Address value must be an object".to_string()))
            }
        }

        // Integer types
        "Int" | "Int8" | "Int16" | "Int32" | "Int64" | "Int128" | "Int256" |
        "UInt" | "UInt8" | "UInt16" | "UInt32" | "UInt64" | "UInt128" | "UInt256" |
        "Word8" | "Word16" | "Word32" | "Word64" | "Word128" | "Word256" => {
            if let Value::Object(obj) = value {
                if let Some(Value::String(s)) = obj.get("value") {
                    match type_name {
                        "Int" => Ok(CadenceValue::Int { value: s.clone() }),
                        "Int8" => Ok(CadenceValue::Int8 { value: s.clone() }),
                        "Int16" => Ok(CadenceValue::Int16 { value: s.clone() }),
                        "Int32" => Ok(CadenceValue::Int32 { value: s.clone() }),
                        "Int64" => Ok(CadenceValue::Int64 { value: s.clone() }),
                        "Int128" => Ok(CadenceValue::Int128 { value: s.clone() }),
                        "Int256" => Ok(CadenceValue::Int256 { value: s.clone() }),
                        "UInt" => Ok(CadenceValue::UInt { value: s.clone() }),
                        "UInt8" => Ok(CadenceValue::UInt8 { value: s.clone() }),
                        "UInt16" => Ok(CadenceValue::UInt16 { value: s.clone() }),
                        "UInt32" => Ok(CadenceValue::UInt32 { value: s.clone() }),
                        "UInt64" => Ok(CadenceValue::UInt64 { value: s.clone() }),
                        "UInt128" => Ok(CadenceValue::UInt128 { value: s.clone() }),
                        "UInt256" => Ok(CadenceValue::UInt256 { value: s.clone() }),
                        "Word8" => Ok(CadenceValue::Word8 { value: s.clone() }),
                        "Word16" => Ok(CadenceValue::Word16 { value: s.clone() }),
                        "Word32" => Ok(CadenceValue::Word32 { value: s.clone() }),
                        "Word64" => Ok(CadenceValue::Word64 { value: s.clone() }),
                        "Word128" => Ok(CadenceValue::Word128 { value: s.clone() }),
                        "Word256" => Ok(CadenceValue::Word256 { value: s.clone() }),
                        _ => unreachable!(),
                    }
                } else {
                    Err(Error::InvalidCadenceValue(format!("{} value must be a string", type_name)))
                }
            } else {
                Err(Error::InvalidCadenceValue(format!("{} value must be an object", type_name)))
            }
        }

        // Fixed point numbers
        "Fix64" | "UFix64" => {
            if let Value::Object(obj) = value {
                if let Some(Value::String(s)) = obj.get("value") {
                    match type_name {
                        "Fix64" => Ok(CadenceValue::Fix64 { value: s.clone() }),
                        "UFix64" => Ok(CadenceValue::UFix64 { value: s.clone() }),
                        _ => unreachable!(),
                    }
                } else {
                    Err(Error::InvalidCadenceValue(format!("{} value must be a string", type_name)))
                }
            } else {
                Err(Error::InvalidCadenceValue(format!("{} value must be an object", type_name)))
            }
        }

        "Array" => {
            if let Value::Object(obj) = value {
                if let Some(Value::Array(arr)) = obj.get("value") {
                    let mut cadence_values = Vec::with_capacity(arr.len());
                    for item in arr {
                        cadence_values.push(value_to_cadence_value(item)?);
                    }
                    Ok(CadenceValue::Array { value: cadence_values })
                } else {
                    Err(Error::InvalidCadenceValue("Array value must be an array".to_string()))
                }
            } else {
                Err(Error::InvalidCadenceValue("Array value must be an object".to_string()))
            }
        }

        "Dictionary" => {
            if let Value::Object(obj) = value {
                if let Some(Value::Array(arr)) = obj.get("value") {
                    let mut entries = Vec::with_capacity(arr.len());
                    for item in arr {
                        if let Value::Object(entry_obj) = item {
                            let key = entry_obj.get("key").ok_or_else(|| {
                                Error::InvalidCadenceValue("Dictionary entry missing key".to_string())
                            })?;
                            let value = entry_obj.get("value").ok_or_else(|| {
                                Error::InvalidCadenceValue("Dictionary entry missing value".to_string())
                            })?;

                            entries.push(DictionaryEntry {
                                key: value_to_cadence_value(key)?,
                                value: value_to_cadence_value(value)?,
                            });
                        } else {
                            return Err(Error::InvalidCadenceValue(
                                "Dictionary entry must be an object".to_string()
                            ));
                        }
                    }
                    Ok(CadenceValue::Dictionary { value: entries })
                } else {
                    Err(Error::InvalidCadenceValue("Dictionary value must be an array".to_string()))
                }
            } else {
                Err(Error::InvalidCadenceValue("Dictionary value must be an object".to_string()))
            }
        }

        // Composite types
        "Struct" | "Resource" | "Event" | "Contract" | "Enum" => {
            if let Value::Object(obj) = value {
                if let Some(Value::Object(composite_obj)) = obj.get("value") {
                    let id = composite_obj.get("id").ok_or_else(|| {
                        Error::InvalidCadenceValue(format!("{} value missing id", type_name))
                    })?;

                    let id = if let Value::String(s) = id {
                        s.clone()
                    } else {
                        return Err(Error::InvalidCadenceValue(format!("{} id must be a string", type_name)));
                    };

                    let fields = composite_obj.get("fields").ok_or_else(|| {
                        Error::InvalidCadenceValue(format!("{} value missing fields", type_name))
                    })?;

                    let fields = if let Value::Array(arr) = fields {
                        let mut field_vec = Vec::with_capacity(arr.len());
                        for field in arr {
                            if let Value::Object(field_obj) = field {
                                let name = field_obj.get("name").ok_or_else(|| {
                                    Error::InvalidCadenceValue("Field missing name".to_string())
                                })?;

                                let name = if let Value::String(s) = name {
                                    s.clone()
                                } else {
                                    return Err(Error::InvalidCadenceValue("Field name must be a string".to_string()));
                                };

                                let value = field_obj.get("value").ok_or_else(|| {
                                    Error::InvalidCadenceValue("Field missing value".to_string())
                                })?;

                                field_vec.push(CompositeField {
                                    name,
                                    value: value_to_cadence_value(value)?,
                                });
                            } else {
                                return Err(Error::InvalidCadenceValue("Field must be an object".to_string()));
                            }
                        }
                        field_vec
                    } else {
                        return Err(Error::InvalidCadenceValue("Fields must be an array".to_string()));
                    };

                    let composite_value = CompositeValue { id, fields };

                    match type_name {
                        "Struct" => Ok(CadenceValue::Struct { value: composite_value }),
                        "Resource" => Ok(CadenceValue::Resource { value: composite_value }),
                        "Event" => Ok(CadenceValue::Event { value: composite_value }),
                        "Contract" => Ok(CadenceValue::Contract { value: composite_value }),
                        "Enum" => Ok(CadenceValue::Enum { value: composite_value }),
                        _ => unreachable!(),
                    }
                } else {
                    Err(Error::InvalidCadenceValue(format!("{} value must be an object", type_name)))
                }
            } else {
                Err(Error::InvalidCadenceValue(format!("{} value must be an object", type_name)))
            }
        }

        // Other types would be implemented similarly
        // This is a partial implementation for the most common types

        _ => Err(Error::UnsupportedType(type_name.to_string())),
    }
}

/// Convert a CadenceValue to a serde_json::Value
pub fn cadence_value_to_value(cadence_value: &CadenceValue) -> Result<Value> {
    match cadence_value {
        CadenceValue::Void {} => {
            let mut obj = serde_json::Map::new();
            obj.insert("type".to_string(), Value::String("Void".to_string()));
            Ok(Value::Object(obj))
        }

        CadenceValue::Optional { value } => {
            let mut obj = serde_json::Map::new();
            obj.insert("type".to_string(), Value::String("Optional".to_string()));

            match value {
                Some(v) => {
                    let inner_value = cadence_value_to_value(v)?;
                    obj.insert("value".to_string(), inner_value);
                }
                None => {
                    obj.insert("value".to_string(), Value::Null);
                }
            }

            Ok(Value::Object(obj))
        }

        CadenceValue::Bool { value } => {
            let mut obj = serde_json::Map::new();
            obj.insert("type".to_string(), Value::String("Bool".to_string()));
            obj.insert("value".to_string(), Value::Bool(*value));
            Ok(Value::Object(obj))
        }

        CadenceValue::String { value } => {
            let mut obj = serde_json::Map::new();
            obj.insert("type".to_string(), Value::String("String".to_string()));
            obj.insert("value".to_string(), Value::String(value.clone()));
            Ok(Value::Object(obj))
        }

        CadenceValue::Address { value } => {
            let mut obj = serde_json::Map::new();
            obj.insert("type".to_string(), Value::String("Address".to_string()));
            obj.insert("value".to_string(), Value::String(value.clone()));
            Ok(Value::Object(obj))
        }

        // Integer types - handle all as string values
        CadenceValue::Int { value } |
        CadenceValue::Int8 { value } |
        CadenceValue::Int16 { value } |
        CadenceValue::Int32 { value } |
        CadenceValue::Int64 { value } |
        CadenceValue::Int128 { value } |
        CadenceValue::Int256 { value } |
        CadenceValue::UInt { value } |
        CadenceValue::UInt8 { value } |
        CadenceValue::UInt16 { value } |
        CadenceValue::UInt32 { value } |
        CadenceValue::UInt64 { value } |
        CadenceValue::UInt128 { value } |
        CadenceValue::UInt256 { value } |
        CadenceValue::Word8 { value } |
        CadenceValue::Word16 { value } |
        CadenceValue::Word32 { value } |
        CadenceValue::Word64 { value } |
        CadenceValue::Word128 { value } |
        CadenceValue::Word256 { value } => {
            let mut obj = serde_json::Map::new();

            // Set the type based on which variant we have
            let type_name = match cadence_value {
                CadenceValue::Int { .. } => "Int",
                CadenceValue::Int8 { .. } => "Int8",
                CadenceValue::Int16 { .. } => "Int16",
                CadenceValue::Int32 { .. } => "Int32",
                CadenceValue::Int64 { .. } => "Int64",
                CadenceValue::Int128 { .. } => "Int128",
                CadenceValue::Int256 { .. } => "Int256",
                CadenceValue::UInt { .. } => "UInt",
                CadenceValue::UInt8 { .. } => "UInt8",
                CadenceValue::UInt16 { .. } => "UInt16",
                CadenceValue::UInt32 { .. } => "UInt32",
                CadenceValue::UInt64 { .. } => "UInt64",
                CadenceValue::UInt128 { .. } => "UInt128",
                CadenceValue::UInt256 { .. } => "UInt256",
                CadenceValue::Word8 { .. } => "Word8",
                CadenceValue::Word16 { .. } => "Word16",
                CadenceValue::Word32 { .. } => "Word32",
                CadenceValue::Word64 { .. } => "Word64",
                CadenceValue::Word128 { .. } => "Word128",
                CadenceValue::Word256 { .. } => "Word256",
                _ => unreachable!(),
            };

            obj.insert("type".to_string(), Value::String(type_name.to_string()));
            obj.insert("value".to_string(), Value::String(value.clone()));
            Ok(Value::Object(obj))
        }

        // Fixed point numbers
        CadenceValue::Fix64 { value } => {
            let mut obj = serde_json::Map::new();
            obj.insert("type".to_string(), Value::String("Fix64".to_string()));
            obj.insert("value".to_string(), Value::String(value.clone()));
            Ok(Value::Object(obj))
        }

        CadenceValue::UFix64 { value } => {
            let mut obj = serde_json::Map::new();
            obj.insert("type".to_string(), Value::String("UFix64".to_string()));
            obj.insert("value".to_string(), Value::String(value.clone()));
            Ok(Value::Object(obj))
        }

        CadenceValue::Array { value } => {
            let mut obj = serde_json::Map::new();
            obj.insert("type".to_string(), Value::String("Array".to_string()));

            let mut array_values = Vec::with_capacity(value.len());
            for item in value {
                array_values.push(cadence_value_to_value(item)?);
            }

            obj.insert("value".to_string(), Value::Array(array_values));
            Ok(Value::Object(obj))
        }

        CadenceValue::Dictionary { value } => {
            let mut obj = serde_json::Map::new();
            obj.insert("type".to_string(), Value::String("Dictionary".to_string()));

            let mut dict_entries = Vec::with_capacity(value.len());
            for entry in value {
                let mut entry_obj = serde_json::Map::new();
                entry_obj.insert("key".to_string(), cadence_value_to_value(&entry.key)?);
                entry_obj.insert("value".to_string(), cadence_value_to_value(&entry.value)?);
                dict_entries.push(Value::Object(entry_obj));
            }

            obj.insert("value".to_string(), Value::Array(dict_entries));
            Ok(Value::Object(obj))
        }

        // Composite types
        CadenceValue::Struct { value } |
        CadenceValue::Resource { value } |
        CadenceValue::Event { value } |
        CadenceValue::Contract { value } |
        CadenceValue::Enum { value } => {
            let mut obj = serde_json::Map::new();

            // Set the type based on which variant we have
            let type_name = match cadence_value {
                CadenceValue::Struct { .. } => "Struct",
                CadenceValue::Resource { .. } => "Resource",
                CadenceValue::Event { .. } => "Event",
                CadenceValue::Contract { .. } => "Contract",
                CadenceValue::Enum { .. } => "Enum",
                _ => unreachable!(),
            };

            obj.insert("type".to_string(), Value::String(type_name.to_string()));

            let mut value_obj = serde_json::Map::new();
            value_obj.insert("id".to_string(), Value::String(value.id.clone()));

            let mut fields_arr = Vec::with_capacity(value.fields.len());
            for field in &value.fields {
                let mut field_obj = serde_json::Map::new();
                field_obj.insert("name".to_string(), Value::String(field.name.clone()));
                field_obj.insert("value".to_string(), cadence_value_to_value(&field.value)?);
                fields_arr.push(Value::Object(field_obj));
            }

            value_obj.insert("fields".to_string(), Value::Array(fields_arr));
            obj.insert("value".to_string(), Value::Object(value_obj));

            Ok(Value::Object(obj))
        }

        // Add implementations for other types as needed...

        _ => Err(Error::UnsupportedType(format!("Unsupported type for conversion to JSON: {:?}", cadence_value))),
    }
}

// Now the important part - actually implementing the from_cadence_value function
// This function needs to be updated to correctly handle dictionary types
pub fn from_cadence_value<T>(cadence_value: &CadenceValue) -> Result<T>
where
    T: for<'de> Deserialize<'de>,
{
    // Check if we're deserializing to a HashMap or BTreeMap
    let type_name = std::any::type_name::<T>();
    let is_map = type_name.contains("HashMap") || type_name.contains("BTreeMap");

    // Special handling for dictionaries being deserialized to maps
    if is_map && matches!(cadence_value, CadenceValue::Dictionary { .. }) {
        if let CadenceValue::Dictionary { value: entries } = cadence_value {
            // Create a map that serde can deserialize into a HashMap/BTreeMap
            let mut map = serde_json::Map::new();

            for entry in entries {
                // Convert key to a string (the key for our JSON object)
                let key_str = match &entry.key {
                    CadenceValue::String { value } => value.clone(),
                    // For other types, convert to string
                    _ => {
                        let key_json = cadence_value_to_value(&entry.key)?;
                        if let Value::String(s) = extract_primitive_value(&key_json) {
                            s
                        } else {
                            // If not a string, use JSON representation
                            serde_json::to_string(&key_json)?
                        }
                    }
                };

                // Convert value and handle numeric conversions
                let value_json = cadence_value_to_value(&entry.value)?;
                let processed_value = process_numeric_values(value_json);
                let final_value = extract_primitive_value(&processed_value);

                // Add to our map
                map.insert(key_str, final_value);
            }

            // Deserialize the map directly to T (HashMap/BTreeMap)
            return serde_json::from_value(Value::Object(map))
                .map_err(|e| Error::SerdeJson(e));
        }
    }

    // Special handling for numeric types
    let is_numeric = type_name == "i8" || type_name == "i16" || type_name == "i32" ||
        type_name == "i64" || type_name == "i128" || type_name == "u8" ||
        type_name == "u16" || type_name == "u32" || type_name == "u64" ||
        type_name == "u128" || type_name == "f32" || type_name == "f64";

    if is_numeric {
        match cadence_value {
            CadenceValue::Int { value } |
            CadenceValue::Int8 { value } |
            CadenceValue::Int16 { value } |
            CadenceValue::Int32 { value } |
            CadenceValue::Int64 { value } |
            CadenceValue::Int128 { value } |
            CadenceValue::Int256 { value } => {
                if type_name.starts_with('i') || type_name.starts_with('u') {
                    if let Ok(n) = value.parse::<i64>() {
                        return serde_json::from_value(Value::Number(serde_json::Number::from(n)))
                            .map_err(|e| Error::SerdeJson(e));
                    }
                }
            },
            CadenceValue::UInt { value } |
            CadenceValue::UInt8 { value } |
            CadenceValue::UInt16 { value } |
            CadenceValue::UInt32 { value } |
            CadenceValue::UInt64 { value } |
            CadenceValue::UInt128 { value } |
            CadenceValue::UInt256 { value } => {
                if type_name.starts_with('u') || type_name.starts_with('i') {
                    if let Ok(n) = value.parse::<u64>() {
                        return serde_json::from_value(Value::Number(serde_json::Number::from(n)))
                            .map_err(|e| Error::SerdeJson(e));
                    }
                }
            },
            _ => {}
        }
    }

    // Struct types with numeric fields
    if std::any::type_name::<T>().contains("::") && !std::any::type_name::<T>().starts_with("std::") {
        let json_value = cadence_value_to_value(cadence_value)?;
        let processed = process_numeric_values(json_value);

        // For composite types, we need to create a flat object with field names
        if let CadenceValue::Struct { value } = cadence_value {
            let mut obj = serde_json::Map::new();
            for field in &value.fields {
                let field_json = cadence_value_to_value(&field.value)?;
                let processed_field = process_numeric_values(field_json);
                obj.insert(field.name.clone(), extract_primitive_value(&processed_field));
            }
            return serde_json::from_value(Value::Object(obj))
                .map_err(|e| Error::SerdeJson(e));
        }

        return serde_json::from_value(processed)
            .map_err(|e| Error::SerdeJson(e));
    }

    // Standard path for other types
    let json_value = cadence_value_to_value(cadence_value)?;
    let processed = process_numeric_values(json_value);
    let final_value = extract_primitive_value(&processed);

    serde_json::from_value(final_value)
        .map_err(|e| Error::SerdeJson(e))
}

// Helper function to recursively process JSON values and convert string numbers to actual JSON numbers
fn process_numeric_values(value: Value) -> Value {
    match value {
        Value::Object(mut obj) => {
            // Check if this is a Cadence type/value structure
            if let (Some(Value::String(type_name)), Some(inner_value)) = (obj.get("type").cloned(), obj.get("value").cloned()) {
                // Handle specific types
                match type_name.as_str() {
                    // Integer types - convert string to number
                    "Int" => {
                        if let Value::String(s) = &inner_value {
                            if let Ok(num) = s.parse::<i64>() {
                                obj.insert("value".to_string(), Value::Number(serde_json::Number::from(num)));
                            }
                        }
                    },
                    "UInt" => {
                        if let Value::String(s) = &inner_value {
                            if let Ok(num) = s.parse::<u64>() {
                                obj.insert("value".to_string(), Value::Number(serde_json::Number::from(num)));
                            }
                        }
                    },
                    // Composite type with fields
                    "Struct" | "Resource" | "Event" | "Contract" | "Enum" => {
                        if let Value::Object(mut inner_obj) = inner_value {
                            if let Some(Value::Array(fields)) = inner_obj.get("fields").cloned() {
                                // Process each field
                                let processed_fields: Vec<Value> = fields.into_iter()
                                    .map(|field| {
                                        if let Value::Object(mut field_obj) = field {
                                            // Process the field value
                                            if let Some(field_value) = field_obj.get("value").cloned() {
                                                field_obj.insert("value".to_string(), process_numeric_values(field_value));
                                            }
                                            Value::Object(field_obj)
                                        } else {
                                            field
                                        }
                                    })
                                    .collect();

                                // Update fields array
                                inner_obj.insert("fields".to_string(), Value::Array(processed_fields));
                                obj.insert("value".to_string(), Value::Object(inner_obj));
                            }
                        }
                    },
                    // Array type
                    "Array" => {
                        if let Value::Array(items) = inner_value {
                            // Process each item
                            let processed_items: Vec<Value> = items.into_iter()
                                .map(|item| process_numeric_values(item))
                                .collect();

                            // Update array
                            obj.insert("value".to_string(), Value::Array(processed_items));
                        }
                    },
                    // Dictionary type
                    "Dictionary" => {
                        if let Value::Array(entries) = inner_value {
                            // Process each dictionary entry
                            let processed_entries: Vec<Value> = entries.into_iter()
                                .map(|entry| {
                                    if let Value::Object(mut entry_obj) = entry {
                                        // Process key and value
                                        if let Some(key) = entry_obj.get("key").cloned() {
                                            entry_obj.insert("key".to_string(), process_numeric_values(key));
                                        }
                                        if let Some(value) = entry_obj.get("value").cloned() {
                                            entry_obj.insert("value".to_string(), process_numeric_values(value));
                                        }
                                        Value::Object(entry_obj)
                                    } else {
                                        entry
                                    }
                                })
                                .collect();

                            // Update entries array
                            obj.insert("value".to_string(), Value::Array(processed_entries));
                        }
                    },
                    _ => {
                        // For other types, recursively process the value
                        obj.insert("value".to_string(), process_numeric_values(inner_value));
                    }
                }
            } else {
                // For regular objects, process all values
                for (key, val) in obj.iter_mut() {
                    if key != "type" { // Don't process the type field
                        *val = process_numeric_values(val.clone());
                    }
                }
            }
            Value::Object(obj)
        },
        Value::Array(items) => {
            // Process each item in the array
            let processed_items: Vec<Value> = items.into_iter()
                .map(|item| process_numeric_values(item))
                .collect();

            Value::Array(processed_items)
        },
        // Other value types don't need processing
        _ => value,
    }
}

// Helper function to extract primitive value from a Cadence type/value structure
fn extract_primitive_value(value: &Value) -> Value {
    if let Value::Object(obj) = value {
        if let (Some(Value::String(_)), Some(inner_value)) = (obj.get("type"), obj.get("value")) {
            // For primitive types, extract the inner value
            match inner_value {
                Value::String(_) | Value::Number(_) | Value::Bool(_) | Value::Null => inner_value.clone(),
                _ => value.clone(),
            }
        } else {
            value.clone()
        }
    } else {
        value.clone()
    }
}