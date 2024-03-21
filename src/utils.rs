use antelope::Asset;

pub fn to_value(quantity: &Asset) -> f64 {
    let as_str = quantity.to_string();
    let as_str = match as_str.split_whitespace().next() {
        Some(value) => value,
        None => return 0.0,
    };
    match as_str.parse::<f64>() {
        Ok(value) => value,
        Err(_) => 0.0,
    }
}

pub fn to_key(trx_id: &str, action_index: u32) -> String {
    format!("{}-{}", trx_id, action_index)
}
