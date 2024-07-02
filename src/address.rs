use bitcoin::network::constants::Network;
use bitcoin::address::Address;
use bitcoin::Script;
use hex::FromHex;

pub fn address_from_scriptpubkey(script_pubkey: &str) -> Option<String> {
    let hex_data = Vec::from_hex(script_pubkey).ok()?;
    let script = Script::from(hex_data.as_slice());
    Address::from_script(&script, Network::Bitcoin).map(|addr| addr.to_string())
}

#[cfg(test)]
mod tests {
    use super::address_from_scriptpubkey;

    #[test]
    fn test_address_from_scriptpubkey() {
        assert_eq!(
            address_from_scriptpubkey("76a914534e48e9a49ce7ebf8d84c8313e4edfa48852fa188ac"),
            Some("18bUsFHLgFotUqAL9ftLBVenJDVP7M64Nu".into())
        )
    }
}