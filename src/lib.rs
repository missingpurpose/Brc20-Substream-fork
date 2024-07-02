mod pb;
mod address;

use anyhow::Result;
use address::address_from_scriptpubkey;
use pb::sf::bitcoin::r#type::v1 as btc;
use pb::btc::cap_table::v1::{CapTable, CapTableEntry};
use std::collections::HashMap;
use substreams_macro::map; 

#[map] // procedural macro correct
fn map_cap_table(
    block: btc::Block,
    addresses: Vec<String>,
) -> Result<CapTable, substreams::errors::Error> {
    let mut cap_table = CapTable { entries: vec![] };
    let mut address_map: HashMap<String, u64> = HashMap::new();

    for tx in block.tx {
        for vout in tx.vout {
            if let Some(address) = vout.address() {
                if addresses.contains(&address) {
                    let entry = address_map.entry(address).or_insert(0);
                    *entry += vout.value as u64; // Convert f64 to u64
                }
            }
        }
    }

    for (address, amount) in address_map {
        cap_table.entries.push(CapTableEntry { address, amount });
    }

    Ok(cap_table)
}

impl btc::Vout {
    pub fn address(&self) -> Option<String> {
        self.script_pub_key
            .as_ref()
            .map(|script_pub_key| hex::encode(script_pub_key.as_bytes()))  // Convert to bytes
            .and_then(|script_pub_key_hex| address_from_scriptpubkey(&script_pub_key_hex))
    }
}