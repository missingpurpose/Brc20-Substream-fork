use crate::pb::sf::bitcoin::r#type::v1 as btc;
use anyhow::Result;
use bitcoin::{
    address::Address, blockdata::script::Script, consensus::deserialize, hashes::hex::FromHex,
    network::Network, Transaction,
};

// ================================================================
// Sat utils
// ================================================================
pub fn btc_to_sats(btc_amount: f64) -> u64 {
    let s = format!("{:.8}", btc_amount);
    s.replace(".", "").parse::<u64>().unwrap()
}

// From https://github.com/ordinals/ord/blob/master/bip.mediawiki
pub fn subsidy(height: u64) -> u64 {
    50 * 100_000_000 >> (height / 210_000)
}

pub fn block_supply(height: u64) -> u64 {
    let mut supply = 0;

    for i in 0.. {
        if height < (i + 1) * 210_000 {
            supply += (1 + height % 210_000) * subsidy(height);
            break;
        } else {
            supply += 210_000 * subsidy(i * 210_000);
        }
    }
    supply
}

// ================================================================
// Address utils
// ================================================================
pub fn address_from_script(script: &Script) -> Option<String> {
    Address::from_script(script, Network::Bitcoin)
        .map(|address| address.to_string())
        .ok()
}

// ================================================================
// BTC protobuf models utils
// ================================================================
impl btc::Transaction {
    /// Returns the nth satoshi UTXO in the transaction along with its offset within the UTXO.
    pub fn nth_sat_utxo(&self, offset: u64) -> Option<(btc::Vout, u64)> {
        let mut sat = 0;
        for (idx, output) in self.vout.iter().enumerate() {
            let utxo_sats = btc_to_sats(output.value);
            if sat + utxo_sats > offset {
                return Some((self.vout[idx].clone(), offset - sat));
            }
            sat += utxo_sats;
        }
        None
    }
}

impl btc::Vout {
    pub fn address(&self) -> Option<String> {
        self.script_pub_key
            .as_ref()
            .and_then(|script_pub_key| address_from_script(&Script::from(script_pub_key.as_slice())))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_btc_to_sats() {
        assert_eq!(btc_to_sats(0.00000001), 1);
    }

    #[test]
    fn test_subsidy() {
        assert_eq!(subsidy(0), 5000000000);
        assert_eq!(subsidy(209999), 5000000000);
        assert_eq!(subsidy(210000), 2500000000);
        assert_eq!(subsidy(419999), 2500000000);
        assert_eq!(subsidy(420000), 1250000000);
    }

    #[test]
    fn test_block_supply() {
        assert_eq!(block_supply(0), 5000000000, "Block 0 has 5000000000 sats");
        assert_eq!(block_supply(1), 10000000000, "Block 1 has 10000000000 sats");
        assert_eq!(block_supply(4), 25000000000, "Block 4 has 25000000000 sats");
        assert_eq!(
            block_supply(209999),
            1050000000000000,
            "Block 209999 has 1050000000000000 sats"
        );
        assert_eq!(
            block_supply(210000),
            1050002500000000,
            "Block 210000 has 1050002500000000 sats"
        );
        assert_eq!(
            block_supply(210001),
            1050005000000000,
            "Block 210001 has 1050005000000000 sats"
        );
        assert_eq!(
            block_supply(419999),
            1575000000000000,
            "Block 419999 has 1575000000000000 sats"
        );
        assert_eq!(
            block_supply(420000),
            1575001250000000,
            "Block 420000 has 1575001250000000 sats"
        );
    }

    #[test]
    fn test_address_from_script() {
        let script_pub_key_hex = "76a914534e48e9a49ce7ebf8d84c8313e4edfa48852fa188ac";
        let hex_data = hex::decode(script_pub_key_hex).expect("Valid hex script");
        let script = Script::from(hex_data.as_slice());
        assert_eq!(
            address_from_script(&script),
            Some("18bUsFHLgFotUqAL9ftLBVenJDVP7M64Nu".into())
        )
    }
}