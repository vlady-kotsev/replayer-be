use base64::{Engine, engine::general_purpose::STANDARD};
use serde::{Deserialize, Deserializer, de};
use solana_keypair::{Address, Signature};

pub fn deserialize_signature<'de, D>(deserializer: D) -> Result<Signature, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    let signature: Signature = s
        .parse()
        .map_err(|_| de::Error::custom("Failed to deserialize signature"))?;

    Ok(signature)
}

pub fn deserialize_address<'de, D>(deserializer: D) -> Result<Address, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    let address: Address = s
        .parse()
        .map_err(|_| de::Error::custom("Failed to deserialize address"))?;

    Ok(address)
}

pub fn deserialize_keypair<'de, D>(deserializer: D) -> Result<[u8; 64], D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    let bytes = STANDARD.decode(s).map_err(de::Error::custom)?;
    bytes
        .try_into()
        .map_err(|_| de::Error::custom("Failed to deserialize keypair"))
}
