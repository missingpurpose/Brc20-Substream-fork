mod btc_utils;

use substreams::pb::btc::cap_table::v1::{Block, CapTable, CapTableEntry};
use crate::btc_utils::get_balance_for_address;
use std::collections::HashMap;

pub fn generate_cap_table(addresses: Vec<String>, address_balances: &HashMap<String, u64>) -> CapTable {
    let mut cap_table_map: HashMap<String, u64> = HashMap::new();

    for address in addresses {
        if let Some(balance) = get_balance_for_address(&address, address_balances) {
            cap_table_map.insert(address, balance);
        }
    }

    let entries = cap_table_map
        .into_iter()
        .map(|(address, amount)| CapTableEntry { address, amount })
        .collect();

    CapTable { entries }
}