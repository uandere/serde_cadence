// examples/derive_example.rs
use cadence_json::{self, FromCadenceValue, Result, ToCadenceValue};
use serde::{Deserialize, Serialize};

// Define a struct and derive both Serde and our custom Cadence traits
#[derive(Debug, Serialize, Deserialize, ToCadenceValue, FromCadenceValue)]
struct NFT {
    id: String,
    name: String,
    owner: String,
    metadata: Metadata,
}

#[derive(Debug, Serialize, Deserialize, ToCadenceValue, FromCadenceValue)]
struct Metadata {
    artist: String,
    creation_date: String,
    tags: Vec<String>,
}

fn main() -> Result<()> {
    // Create an NFT instance
    let nft = NFT {
        id: "nft-123".to_string(),
        name: "Cosmic Dream #42".to_string(),
        owner: "0x1234567890abcdef".to_string(),
        metadata: Metadata {
            artist: "DigitalArtist99".to_string(),
            creation_date: "2023-04-15".to_string(),
            tags: vec![
                "abstract".to_string(),
                "colorful".to_string(),
                "digital".to_string(),
            ],
        },
    };

    // Convert to Cadence-JSON
    let cadence_value = nft.to_cadence_value()?;

    // Serialize to JSON string
    let json = serde_json::to_string_pretty(&cadence_value)?;
    println!("Cadence-JSON representation:\n{}", json);

    // Deserialize back to Rust
    let deserialized_nft = NFT::from_cadence_value(&cadence_value)?;
    println!("Deserialized NFT: {:#?}", deserialized_nft);

    Ok(())
}
