<<<<<<< HEAD
mod pb;
mod address;

use anyhow::Result;
use address::address_from_scriptpubkey;
use pb::sf::bitcoin::r#type::v1 as btc;
use pb::btc::cap_table::v1::{CapTable, CapTableEntry};
use std::collections::HashMap;

#[substreams::handlers::map]
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
=======
mod btc_utils;
mod pb;
mod tables_utils;

use std::collections::HashSet;
use std::str::FromStr;

use anyhow::Result;
use btc_utils::{btc_to_sats, parse_inscriptions};
use pb::bitcoin::v1::{CapTable, CapTableEntry}; // Updated path
use pb::sf::bitcoin::r#type::v1 as btc;
use substreams::pb::substreams::store_delta::Operation;
use substreams::pb::substreams::Clock;
use substreams::scalar::BigInt;
use substreams::store::{DeltaBigInt, Deltas, StoreAddBigInt, StoreGetProto, StoreSetProto, StoreNew}; // Added StoreNew
use substreams_entity_change::pb::entity::EntityChanges;
use substreams_entity_change::tables::Tables;

struct AddressEntry {
    pub address: String,
    pub balance: u64,
}

fn parse_query_params() -> HashSet<String> {
    // This function should parse the query parameters and return a set of addresses
    // For example, you can use environment variables or other means to get the query parameters
    // Here, we use a placeholder implementation
    let addresses = std::env::var("ADDRESSES").unwrap_or_default();
    addresses.split(',').map(|s| s.to_string()).collect()
}

#[substreams::handlers::map]
fn map_cap_table(block: btc::Block) -> Result<CapTable, substreams::errors::Error> {
    let query_addresses = parse_query_params();
    let entries = block
        .tx
        .into_iter()
        .flat_map(|tx| {
            tx.vout.into_iter().filter_map(|vout| {
                let address = vout.address()?;
                if query_addresses.contains(&address) {
                    let balance = btc_to_sats(vout.value);
                    Some(AddressEntry { address, balance })
                } else {
                    None
                }
            })
        })
        .collect::<Vec<_>>();

    Ok(CapTable {
        entries: entries
            .into_iter()
            .map(|entry| CapTableEntry {
                address: entry.address,
                balance: entry.balance.to_string(),
            })
            .collect(),
    })
}

#[substreams::handlers::store]
fn store_cap_table(entries: CapTable, store: StoreSetProto<CapTableEntry>) {
    entries.entries.iter().for_each(|entry| {
        store.set(
            0,
            entry.address.clone(),
            &CapTableEntry {
                address: entry.address.clone(),
                balance: entry.balance.clone(),
            },
        );
    });
}

#[substreams::handlers::map]
fn graph_out(
    clock: Clock,
    cap_table: CapTable,
    balances_store: Deltas<DeltaBigInt>,
) -> Result<EntityChanges, substreams::errors::Error> {
    let mut tables = Tables::new();

    cap_table.entries.iter().for_each(|entry| {
        tables
            .create_row("CapTableEntry", entry.address.clone())
            .set("address", entry.address.clone())
            .set_bigint("balance", &entry.balance)
            .set("block", clock.number.clone())
            .set(
                "timestamp",
                clock
                    .timestamp
                    .as_ref()
                    .map(|t| t.seconds)
                    .unwrap_or_default(),
            );
    });

    balances_store.deltas.iter().for_each(|delta| match delta.operation {
        Operation::Create => {
            tables
                .create_row("CapTableEntry", delta.key.clone())
                .set("address", delta.key.clone())
                .set_bigint("balance", &delta.new_value.to_string());
        }
        Operation::Update => {
            tables
                .update_row("CapTableEntry", delta.key.clone())
                .set_bigint("balance", &delta.new_value.to_string());
        }
        _ => (),
    });

    Ok(tables.to_entity_changes())
>>>>>>> 0e6ab0d1b3e5365ae7cc9595fd97694521c5461e
}