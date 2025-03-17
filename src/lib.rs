#![allow(unused_variables)]
use std::fmt;
pub use serde::{Deserialize, Serialize};

#[cfg(feature = "derive")]
pub use cadence_json_derive::{ToCadenceValue, FromCadenceValue};

pub mod impls;


/// A Cadence value as represented in JSON
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum CadenceValue {
    #[serde(rename = "Void")]
    Void {},

    #[serde(rename = "Optional")]
    Optional {
        value: Option<Box<CadenceValue>>,
    },

    #[serde(rename = "Bool")]
    Bool {
        value: bool,
    },

    #[serde(rename = "String")]
    String {
        value: String,
    },

    #[serde(rename = "Address")]
    Address {
        value: String, // Hex-encoded string with 0x prefix
    },

    // Integer types
    #[serde(rename = "Int")]
    Int {
        value: String, // Decimal string representation
    },

    #[serde(rename = "Int8")]
    Int8 {
        value: String,
    },

    #[serde(rename = "Int16")]
    Int16 {
        value: String,
    },

    #[serde(rename = "Int32")]
    Int32 {
        value: String,
    },

    #[serde(rename = "Int64")]
    Int64 {
        value: String,
    },

    #[serde(rename = "Int128")]
    Int128 {
        value: String,
    },

    #[serde(rename = "Int256")]
    Int256 {
        value: String,
    },

    #[serde(rename = "UInt")]
    UInt {
        value: String,
    },

    #[serde(rename = "UInt8")]
    UInt8 {
        value: String,
    },

    #[serde(rename = "UInt16")]
    UInt16 {
        value: String,
    },

    #[serde(rename = "UInt32")]
    UInt32 {
        value: String,
    },

    #[serde(rename = "UInt64")]
    UInt64 {
        value: String,
    },

    #[serde(rename = "UInt128")]
    UInt128 {
        value: String,
    },

    #[serde(rename = "UInt256")]
    UInt256 {
        value: String,
    },

    #[serde(rename = "Word8")]
    Word8 {
        value: String,
    },

    #[serde(rename = "Word16")]
    Word16 {
        value: String,
    },

    #[serde(rename = "Word32")]
    Word32 {
        value: String,
    },

    #[serde(rename = "Word64")]
    Word64 {
        value: String,
    },

    #[serde(rename = "Word128")]
    Word128 {
        value: String,
    },

    #[serde(rename = "Word256")]
    Word256 {
        value: String,
    },

    // Fixed point numbers
    #[serde(rename = "Fix64")]
    Fix64 {
        value: String, // Decimal string representation
    },

    #[serde(rename = "UFix64")]
    UFix64 {
        value: String,
    },

    #[serde(rename = "Array")]
    Array {
        value: Vec<CadenceValue>,
    },

    #[serde(rename = "Dictionary")]
    Dictionary {
        value: Vec<DictionaryEntry>,
    },

    // Composite types
    #[serde(rename = "Struct")]
    Struct {
        value: CompositeValue,
    },

    #[serde(rename = "Resource")]
    Resource {
        value: CompositeValue,
    },

    #[serde(rename = "Event")]
    Event {
        value: CompositeValue,
    },

    #[serde(rename = "Contract")]
    Contract {
        value: CompositeValue,
    },

    #[serde(rename = "Enum")]
    Enum {
        value: CompositeValue,
    },

    #[serde(rename = "Path")]
    Path {
        value: PathValue,
    },

    #[serde(rename = "Type")]
    Type {
        value: TypeValue,
    },

    #[serde(rename = "InclusiveRange")]
    InclusiveRange {
        value: RangeValue,
    },

    #[serde(rename = "Capability")]
    Capability {
        value: CapabilityValue,
    },

    #[serde(rename = "Function")]
    Function {
        value: FunctionValue,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DictionaryEntry {
    pub key: CadenceValue,
    pub value: CadenceValue,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompositeField {
    pub name: String,
    pub value: CadenceValue,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompositeValue {
    pub id: String, // Fully qualified type identifier
    pub fields: Vec<CompositeField>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PathValue {
    pub domain: PathDomain,
    pub identifier: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PathDomain {
    Storage,
    Private,
    Public,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TypeValue {
    pub static_type: CadenceType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RangeValue {
    pub start: Box<CadenceValue>,
    pub end: Box<CadenceValue>,
    pub step: Box<CadenceValue>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapabilityValue {
    pub id: String,
    pub address: String,
    pub borrow_type: CadenceType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionValue {
    pub function_type: CadenceType,
}

/// Represents a Cadence type in JSON format
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "kind")]
pub enum CadenceType {
    // Simple types
    Account,
    AccountCapabilityController,
    AccountKey,
    Address,
    AnyResource,
    AnyResourceAttachment,
    AnyStruct,
    AnyStructAttachment,
    Block,
    Bool,
    Capability { type_: Box<CadenceType> },
    CapabilityPath,
    Character,
    DeployedContract,
    DeploymentResult,
    Fix64,
    FixedPoint,
    FixedSizeUnsignedInteger,
    HashAlgorithm,
    HashableStruct,
    Int,
    Int8,
    Int16,
    Int32,
    Int64,
    Int128,
    Int256,
    Integer,
    Never,
    Number,
    Path,
    PrivatePath,
    PublicKey,
    PublicPath,
    SignatureAlgorithm,
    SignedFixedPoint,
    SignedInteger,
    SignedNumber,
    StorageCapabilityController,
    StoragePath,
    String,
    Type,
    UFix64,
    UInt,
    UInt8,
    UInt16,
    UInt32,
    UInt64,
    UInt128,
    UInt256,
    Void,
    Word8,
    Word16,
    Word32,
    Word64,
    Word128,
    Word256,

    // Complex types
    Optional { type_: Box<CadenceType> },

    VariableSizedArray { type_: Box<CadenceType> },

    ConstantSizedArray {
        type_: Box<CadenceType>,
        size: usize,
    },

    Dictionary {
        key: Box<CadenceType>,
        value: Box<CadenceType>,
    },

    // Composite types - could be enum variants
    Struct {
        #[serde(rename = "type")]
        type_: String,
        type_id: String,
        initializers: Vec<Vec<ParameterType>>,
        fields: Vec<FieldType>,
    },

    Resource {
        #[serde(rename = "type")]
        type_: String,
        type_id: String,
        initializers: Vec<Vec<ParameterType>>,
        fields: Vec<FieldType>,
    },

    Event {
        #[serde(rename = "type")]
        type_: String,
        type_id: String,
        initializers: Vec<Vec<ParameterType>>,
        fields: Vec<FieldType>,
    },

    Contract {
        #[serde(rename = "type")]
        type_: String,
        type_id: String,
        initializers: Vec<Vec<ParameterType>>,
        fields: Vec<FieldType>,
    },

    StructInterface {
        #[serde(rename = "type")]
        type_: String,
        type_id: String,
        initializers: Vec<Vec<ParameterType>>,
        fields: Vec<FieldType>,
    },

    ResourceInterface {
        #[serde(rename = "type")]
        type_: String,
        type_id: String,
        initializers: Vec<Vec<ParameterType>>,
        fields: Vec<FieldType>,
    },

    ContractInterface {
        #[serde(rename = "type")]
        type_: String,
        type_id: String,
        initializers: Vec<Vec<ParameterType>>,
        fields: Vec<FieldType>,
    },

    Function {
        type_id: String,
        parameters: Vec<ParameterType>,
        purity: Option<String>,
        return_: Box<CadenceType>,
    },

    Reference {
        authorization: Authorization,
        type_: Box<CadenceType>,
    },

    Intersection {
        type_id: String,
        types: Vec<CadenceType>,
    },

    Enum {
        #[serde(rename = "type")]
        type_: Box<CadenceType>,
        type_id: String,
        initializers: Vec<Vec<ParameterType>>,
        fields: Vec<FieldType>,
    },

    InclusiveRange {
        element: Box<CadenceType>,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FieldType {
    pub id: String,
    pub type_: CadenceType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParameterType {
    pub label: String,
    pub id: String,
    pub type_: CadenceType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "kind")]
pub enum Authorization {
    Unauthorized { entitlements: Option<Vec<Entitlement>> },
    EntitlementMapAuthorization { entitlements: Vec<Entitlement> },
    EntitlementConjunctionSet { entitlements: Vec<Entitlement> },
    EntitlementDisjunctionSet { entitlements: Vec<Entitlement> },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "kind")]
pub enum Entitlement {
    EntitlementMap { type_id: String },
    // Add other entitlement types as needed
}

/// Error types for the Cadence-JSON serialization/deserialization
#[derive(Debug)]
pub enum Error {
    SerdeJson(serde_json::Error),
    InvalidCadenceValue(String),
    TypeMismatch { expected: String, got: String },
    UnsupportedType(String),
    Custom(String),
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Error::SerdeJson(err)
    }
}

impl std::error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::SerdeJson(err) => write!(f, "JSON error: {}", err),
            Error::InvalidCadenceValue(msg) => write!(f, "Invalid Cadence value: {}", msg),
            Error::TypeMismatch { expected, got } => {
                write!(f, "Type mismatch: expected {}, got {}", expected, got)
            }
            Error::UnsupportedType(msg) => write!(f, "Unsupported type: {}", msg),
            Error::Custom(msg) => write!(f, "{}", msg),
        }
    }
}

/// Result type for Cadence-JSON operations
pub type Result<T> = std::result::Result<T, Error>;

/// Serializes a Rust type to a Cadence-JSON string
pub fn to_string<T>(value: &T) -> Result<String>
where
    T: Serialize + ?Sized,
{
    let cadence_value = to_cadence_value(value)?;
    let json = serde_json::to_string(&cadence_value)?;
    Ok(json)
}

/// Serializes a Rust type to a pretty-printed Cadence-JSON string
pub fn to_string_pretty<T>(value: &T) -> Result<String>
where
    T: Serialize + ?Sized,
{
    let cadence_value = to_cadence_value(value)?;
    let json = serde_json::to_string_pretty(&cadence_value)?;
    Ok(json)
}

/// Serializes a Rust type to a Cadence-JSON byte vector
pub fn to_vec<T>(value: &T) -> Result<Vec<u8>>
where
    T: Serialize + ?Sized,
{
    let cadence_value = to_cadence_value(value)?;
    let json = serde_json::to_vec(&cadence_value)?;
    Ok(json)
}

/// Serializes a Rust type to a pretty-printed Cadence-JSON byte vector
pub fn to_vec_pretty<T>(value: &T) -> Result<Vec<u8>>
where
    T: Serialize + ?Sized,
{
    let cadence_value = to_cadence_value(value)?;
    let json = serde_json::to_vec_pretty(&cadence_value)?;
    Ok(json)
}

/// Deserializes a Cadence-JSON string to a Rust type
pub fn from_str<'a, T>(s: &'a str) -> Result<T>
where
    T: for<'de> Deserialize<'de>,
{
    let cadence_value: CadenceValue = serde_json::from_str(s)?;
    from_cadence_value(&cadence_value)
}

/// Deserializes a Cadence-JSON byte slice to a Rust type
pub fn from_slice<'a, T>(v: &'a [u8]) -> Result<T>
where
    T: for<'de> Deserialize<'de>,
{
    let cadence_value: CadenceValue = serde_json::from_slice(v)?;
    from_cadence_value(&cadence_value)
}

/// Deserializes a Cadence-JSON reader to a Rust type
pub fn from_reader<R, T>(rdr: R) -> Result<T>
where
    R: std::io::Read,
    T: for<'de> Deserialize<'de>,
{
    let cadence_value: CadenceValue = serde_json::from_reader(rdr)?;
    from_cadence_value(&cadence_value)
}

// Helper functions for conversion
fn to_cadence_value<T>(value: &T) -> Result<CadenceValue>
where
    T: Serialize + ?Sized,
{
    // This is a placeholder implementation.
    // A real implementation would need to analyze the Rust value
    // and convert it to the appropriate CadenceValue variant.
    // This would likely need custom serialization logic.

    // For now, we'll just return an error
    Err(Error::Custom("to_cadence_value not fully implemented".to_string()))
}

fn from_cadence_value<T>(cadence_value: &CadenceValue) -> Result<T>
where
    T: for<'de> Deserialize<'de>,
{
    // This is a placeholder implementation.
    // A real implementation would need to convert the CadenceValue
    // to the appropriate Rust type.
    // This would likely need custom deserialization logic.

    // For now, we'll just return an error
    Err(Error::Custom("from_cadence_value not fully implemented".to_string()))
}

// Additional helper functions for specific type conversions

/// Convert a Rust value to CadenceValue::String
pub fn to_cadence_string<T: ToString>(value: T) -> CadenceValue {
    CadenceValue::String {
        value: value.to_string(),
    }
}

/// Convert a Rust bool to CadenceValue::Bool
pub fn to_cadence_bool(value: bool) -> CadenceValue {
    CadenceValue::Bool { value }
}

/// Convert a Rust Option to CadenceValue::Optional
pub fn to_cadence_optional<T>(value: Option<T>) -> Result<CadenceValue>
where
    T: Serialize,
{
    match value {
        Some(v) => {
            let cadence_value = to_cadence_value(&v)?;
            Ok(CadenceValue::Optional {
                value: Some(Box::new(cadence_value)),
            })
        }
        None => Ok(CadenceValue::Optional { value: None }),
    }
}

/// Convert a Rust Vec to CadenceValue::Array
pub fn to_cadence_array<T>(values: &[T]) -> Result<CadenceValue>
where
    T: Serialize,
{
    let mut cadence_values = Vec::with_capacity(values.len());
    for value in values {
        cadence_values.push(to_cadence_value(value)?);
    }
    Ok(CadenceValue::Array {
        value: cadence_values,
    })
}

/// Convert a Rust Map to CadenceValue::Dictionary
pub fn to_cadence_dictionary<K, V, M>(map: &M) -> Result<CadenceValue>
where
    K: Serialize,
    V: Serialize,
    M: std::ops::Deref<Target = dyn std::ops::Index<K, Output = V>>,
    M::Target: IntoIterator<Item = (K, V)>,
{
    // This is a placeholder; a real implementation would need to
    // iterate over the map and convert each key-value pair
    Err(Error::Custom("to_cadence_dictionary not fully implemented".to_string()))
}

// Trait for types that can be converted to a CadenceValue
pub trait ToCadenceValue {
    fn to_cadence_value(&self) -> Result<CadenceValue>;
}


// Trait for types that can be created from a CadenceValue
pub trait FromCadenceValue: Sized {
    fn from_cadence_value(value: &CadenceValue) -> Result<Self>;
}
