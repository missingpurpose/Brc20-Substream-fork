use std::collections::HashMap;

pub fn btc_to_sats(btc_amount: f64) -> u64 {
    let s = format!("{:.8}", btc_amount);
    s.replace(".", "").parse::<u64>().unwrap()
}

pub fn get_balance_for_address(address: &str, address_balances: &HashMap<String, u64>) -> Option<u64> {
    address_balances.get(address).cloned()
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
    fn test_get_balance_for_address() {
        let mut address_balances = HashMap::new();
        address_balances.insert("address1".to_string(), 1000);
        address_balances.insert("address2".to_string(), 2000);

        assert_eq!(get_balance_for_address("address1", &address_balances), Some(1000));
        assert_eq!(get_balance_for_address("address2", &address_balances), Some(2000));
        assert_eq!(get_balance_for_address("address3", &address_balances), None);
    }
}