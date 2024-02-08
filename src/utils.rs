use antelope::Asset;

pub fn to_value(quantity: Asset) -> f64 {
    quantity.amount as f64 / 10_i64.pow(quantity.symbol.precision() as u32) as f64
}

pub fn to_key(trx_id: &str, action_index: u32) -> String {
    format!("{}-{}", trx_id, action_index)
}