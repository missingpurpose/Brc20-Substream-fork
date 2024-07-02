use bitcoin::network::constants::Network;
use bitcoin::address::Address;
use bitcoin::Script;
use hex::FromHex;
use std::error::Error;

/// Converts a scriptPubKey (hexadecimal string) to a Bitcoin address.
/// 
/// # Arguments
/// 
/// * `script_pubkey` - A string slice that holds the scriptPubKey in hexadecimal format.
/// 
/// # Returns
/// 
/// * `Result<String, Box<dyn Error>>` - On success, returns the Bitcoin address as a string. On failure, returns an error.
pub fn address_from_scriptpubkey(script_pubkey: &str) -> Result<String, Box<dyn Error>> {
    let hex_data = Vec::from_hex(script_pubkey)?;
    let script = Script::from(hex_data.as_slice());
    let address = Address::from_script(&script, Network::Bitcoin)
        .ok_or("Failed to convert script to address")?;
    Ok(address.to_string())
}

#[cfg(test)]
mod tests {
    use super::address_from_scriptpubkey;

    #[test]
    fn test_address_from_scriptpubkey() {
        assert_eq!(
            address_from_scriptpubkey("76a914534e48e9a49ce7ebf8d84c8313e4edfa48852fa188ac").unwrap(),
            "18bUsFHLgFotUqAL9ftLBVenJDVP7M64Nu"
        );
    }

    #[test]
    fn test_invalid_scriptpubkey() {
        assert!(address_from_scriptpubkey("invalid_hex").is_err());
    }

    #[test]
    fn test_empty_scriptpubkey() {
        assert!(address_from_scriptpubkey("").is_err());
    }
}