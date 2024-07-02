mod pb;
mod address;

use anyhow::Result;
use address::address_from_scriptpubkey;
use pb::btc::cap_table::v1::{CapTable, CapTableEntry, Block};
use std::collections::HashMap;
use substreams_macro::map; 
use substreams::errors::Error;

#[map]
fn map_cap_table(
    block: Block,
    addresses: Vec<String>,
) -> Result<CapTable, Error> {
    let mut cap_table = CapTable { entries: vec![] };
    let mut address_map: HashMap<String, u64> = HashMap::new();

    for tx in block.tx {
        for vout in tx.vout {
            if let Some(address) = vout.address() {
                if addresses.contains(&address) {
                    let entry = address_map.entry(address).or_insert(0);
                    *entry += vout.value as u64;
                }
            } else {
                // Log or handle the case where address extraction fails
                eprintln!("Failed to extract address from vout: {:?}", vout);
            }
        }
    }

    for (address, amount) in address_map {
        cap_table.entries.push(CapTableEntry { address, amount });
    }

    Ok(cap_table)
}

impl pb::btc::cap_table::v1::Vout {
    pub fn address(&self) -> Option<String> {
        self.script_pub_key
            .as_ref()
            .map(|script_pub_key| hex::encode(script_pub_key.as_bytes()))
            .and_then(|script_pub_key_hex| address_from_scriptpubkey(&script_pub_key_hex))
    }
}