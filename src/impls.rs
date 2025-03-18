// src/impls.rs

// This file contains implementations of ToCadenceValue and FromCadenceValue
// for standard Rust types

use crate::{CadenceValue, Error, FromCadenceValue, Result, ToCadenceValue};
use std::collections::{BTreeMap, HashMap};

impl FromCadenceValue for CadenceValue {
    fn from_cadence_value(value: &CadenceValue) -> Result<Self> {
        Ok(value.clone())
    }
}

// String implementations
impl ToCadenceValue for String {
    fn to_cadence_value(&self) -> Result<CadenceValue> {
        Ok(CadenceValue::String {
            value: self.clone(),
        })
    }
}

impl ToCadenceValue for &str {
    fn to_cadence_value(&self) -> Result<CadenceValue> {
        Ok(CadenceValue::String {
            value: self.to_string(),
        })
    }
}

impl FromCadenceValue for String {
    fn from_cadence_value(value: &CadenceValue) -> Result<Self> {
        match value {
            CadenceValue::String { value } => Ok(value.clone()),
            _ => Err(Error::TypeMismatch {
                expected: "String".to_string(),
                got: format!("{:?}", value),
            }),
        }
    }
}

// Boolean implementations
impl ToCadenceValue for bool {
    fn to_cadence_value(&self) -> Result<CadenceValue> {
        Ok(CadenceValue::Bool { value: *self })
    }
}

impl FromCadenceValue for bool {
    fn from_cadence_value(value: &CadenceValue) -> Result<Self> {
        match value {
            CadenceValue::Bool { value } => Ok(*value),
            _ => Err(Error::TypeMismatch {
                expected: "Bool".to_string(),
                got: format!("{:?}", value),
            }),
        }
    }
}

// Integer implementations
macro_rules! impl_int_to_cadence {
    ($t:ty, $variant:ident) => {
        impl ToCadenceValue for $t {
            fn to_cadence_value(&self) -> Result<CadenceValue> {
                Ok(CadenceValue::$variant {
                    value: self.to_string(),
                })
            }
        }

        impl FromCadenceValue for $t {
            fn from_cadence_value(value: &CadenceValue) -> Result<Self> {
                match value {
                    CadenceValue::$variant { value } => value.parse().map_err(|e| {
                        Error::Custom(format!("Failed to parse {}: {}", stringify!($t), e))
                    }),
                    CadenceValue::Int { value } => value.parse().map_err(|e| {
                        Error::Custom(format!("Failed to parse {}: {}", stringify!($t), e))
                    }),
                    CadenceValue::UInt { value } => value.parse().map_err(|e| {
                        Error::Custom(format!("Failed to parse {}: {}", stringify!($t), e))
                    }),
                    _ => Err(Error::TypeMismatch {
                        expected: stringify!($variant).to_string(),
                        got: format!("{:?}", value),
                    }),
                }
            }
        }
    };
}

impl_int_to_cadence!(u8, UInt8);
impl_int_to_cadence!(u16, UInt16);
impl_int_to_cadence!(u32, UInt32);
impl_int_to_cadence!(u64, UInt64);
impl_int_to_cadence!(i8, Int8);
impl_int_to_cadence!(i16, Int16);
impl_int_to_cadence!(i32, Int32);
impl_int_to_cadence!(i64, Int64);

// Float implementations
impl ToCadenceValue for f32 {
    fn to_cadence_value(&self) -> Result<CadenceValue> {
        Ok(CadenceValue::Fix64 {
            value: self.to_string(),
        })
    }
}

impl ToCadenceValue for f64 {
    fn to_cadence_value(&self) -> Result<CadenceValue> {
        Ok(CadenceValue::Fix64 {
            value: self.to_string(),
        })
    }
}

impl FromCadenceValue for f32 {
    fn from_cadence_value(value: &CadenceValue) -> Result<Self> {
        match value {
            CadenceValue::Fix64 { value } => value
                .parse()
                .map_err(|e| Error::Custom(format!("Failed to parse f32: {}", e))),
            CadenceValue::UFix64 { value } => value
                .parse()
                .map_err(|e| Error::Custom(format!("Failed to parse f32: {}", e))),
            _ => Err(Error::TypeMismatch {
                expected: "Fix64 or UFix64".to_string(),
                got: format!("{:?}", value),
            }),
        }
    }
}

impl FromCadenceValue for f64 {
    fn from_cadence_value(value: &CadenceValue) -> Result<Self> {
        match value {
            CadenceValue::Fix64 { value } => value
                .parse()
                .map_err(|e| Error::Custom(format!("Failed to parse f64: {}", e))),
            CadenceValue::UFix64 { value } => value
                .parse()
                .map_err(|e| Error::Custom(format!("Failed to parse f64: {}", e))),
            _ => Err(Error::TypeMismatch {
                expected: "Fix64 or UFix64".to_string(),
                got: format!("{:?}", value),
            }),
        }
    }
}

// Vec implementations
impl<T: ToCadenceValue> ToCadenceValue for Vec<T> {
    fn to_cadence_value(&self) -> Result<CadenceValue> {
        let mut values = Vec::with_capacity(self.len());
        for item in self {
            values.push(item.to_cadence_value()?);
        }
        Ok(CadenceValue::Array { value: values })
    }
}

impl<T: FromCadenceValue> FromCadenceValue for Vec<T> {
    fn from_cadence_value(value: &CadenceValue) -> Result<Self> {
        match value {
            CadenceValue::Array { value } => {
                let mut result = Vec::with_capacity(value.len());
                for item in value {
                    result.push(T::from_cadence_value(item)?);
                }
                Ok(result)
            }
            _ => Err(Error::TypeMismatch {
                expected: "Array".to_string(),
                got: format!("{:?}", value),
            }),
        }
    }
}

// Option implementations
impl<T: ToCadenceValue> ToCadenceValue for Option<T> {
    fn to_cadence_value(&self) -> Result<CadenceValue> {
        match self {
            Some(value) => {
                let cadence_value = value.to_cadence_value()?;
                Ok(CadenceValue::Optional {
                    value: Some(Box::new(cadence_value)),
                })
            }
            None => Ok(CadenceValue::Optional { value: None }),
        }
    }
}

impl<T: FromCadenceValue> FromCadenceValue for Option<T> {
    fn from_cadence_value(value: &CadenceValue) -> Result<Self> {
        match value {
            CadenceValue::Optional { value } => match value {
                Some(inner_value) => Ok(Some(T::from_cadence_value(inner_value)?)),
                None => Ok(None),
            },
            _ => Err(Error::TypeMismatch {
                expected: "Optional".to_string(),
                got: format!("{:?}", value),
            }),
        }
    }
}

// HashMap implementations
impl<K, V> ToCadenceValue for HashMap<K, V>
where
    K: ToCadenceValue,
    V: ToCadenceValue,
{
    fn to_cadence_value(&self) -> Result<CadenceValue> {
        let mut entries = Vec::with_capacity(self.len());
        for (key, value) in self {
            entries.push(crate::DictionaryEntry {
                key: key.to_cadence_value()?,
                value: value.to_cadence_value()?,
            });
        }
        Ok(CadenceValue::Dictionary { value: entries })
    }
}

impl<K, V> FromCadenceValue for HashMap<K, V>
where
    K: FromCadenceValue + Eq + std::hash::Hash,
    V: FromCadenceValue,
{
    fn from_cadence_value(value: &CadenceValue) -> Result<Self> {
        match value {
            CadenceValue::Dictionary { value } => {
                let mut result = HashMap::with_capacity(value.len());
                for entry in value {
                    let key = K::from_cadence_value(&entry.key)?;
                    let value = V::from_cadence_value(&entry.value)?;
                    result.insert(key, value);
                }
                Ok(result)
            }
            _ => Err(Error::TypeMismatch {
                expected: "Dictionary".to_string(),
                got: format!("{:?}", value),
            }),
        }
    }
}

// BTreeMap implementations
impl<K, V> ToCadenceValue for BTreeMap<K, V>
where
    K: ToCadenceValue,
    V: ToCadenceValue,
{
    fn to_cadence_value(&self) -> Result<CadenceValue> {
        let mut entries = Vec::with_capacity(self.len());
        for (key, value) in self {
            entries.push(crate::DictionaryEntry {
                key: key.to_cadence_value()?,
                value: value.to_cadence_value()?,
            });
        }
        Ok(CadenceValue::Dictionary { value: entries })
    }
}

impl<K, V> FromCadenceValue for BTreeMap<K, V>
where
    K: FromCadenceValue + Ord,
    V: FromCadenceValue,
{
    fn from_cadence_value(value: &CadenceValue) -> Result<Self> {
        match value {
            CadenceValue::Dictionary { value } => {
                let mut result = BTreeMap::new();
                for entry in value {
                    let key = K::from_cadence_value(&entry.key)?;
                    let value = V::from_cadence_value(&entry.value)?;
                    result.insert(key, value);
                }
                Ok(result)
            }
            _ => Err(Error::TypeMismatch {
                expected: "Dictionary".to_string(),
                got: format!("{:?}", value),
            }),
        }
    }
}

// Tuple implementations (for common sizes)
impl<T1, T2> ToCadenceValue for (T1, T2)
where
    T1: ToCadenceValue,
    T2: ToCadenceValue,
{
    fn to_cadence_value(&self) -> Result<CadenceValue> {
        let mut values = Vec::with_capacity(2);
        values.push(self.0.to_cadence_value()?);
        values.push(self.1.to_cadence_value()?);
        Ok(CadenceValue::Array { value: values })
    }
}

impl<T1, T2> FromCadenceValue for (T1, T2)
where
    T1: FromCadenceValue,
    T2: FromCadenceValue,
{
    fn from_cadence_value(value: &CadenceValue) -> Result<Self> {
        match value {
            CadenceValue::Array { value } => {
                if value.len() != 2 {
                    return Err(Error::Custom(format!(
                        "Expected array of length 2 for tuple, got {}",
                        value.len()
                    )));
                }
                Ok((
                    T1::from_cadence_value(&value[0])?,
                    T2::from_cadence_value(&value[1])?,
                ))
            }
            _ => Err(Error::TypeMismatch {
                expected: "Array".to_string(),
                got: format!("{:?}", value),
            }),
        }
    }
}

// Add more tuple implementations as needed for (T1, T2, T3), etc.
