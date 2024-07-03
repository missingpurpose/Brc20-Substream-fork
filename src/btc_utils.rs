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
    fn test_get_balance_for_address() {
        let mut address_balances = HashMap::new();
        address_balances.insert("address1".to_string(), 1000);
        address_balances.insert("address2".to_string(), 2000);

        assert_eq!(get_balance_for_address("address1", &address_balances), Some(1000));
        assert_eq!(get_balance_for_address("address2", &address_balances), Some(2000));
        assert_eq!(get_balance_for_address("address3", &address_balances), None);
    }
}