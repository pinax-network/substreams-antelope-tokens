pub fn token_key(contract: &str, symcode: &str) -> String {
    format!("{contract}:{symcode}")
}

pub fn account_key(name: &str) -> String {
    format!("{name}")
}

pub fn account_balance_key(contract: &str, symcode: &str, account: &str) -> String {
    format!("{contract}:{symcode}:{account}")
}
