use antelope::Asset;
use substreams::pb::substreams::Clock;

pub fn to_value(quantity: &Asset) -> f64 {
    quantity
        .to_string()
        .split_whitespace()
        .next()
        .and_then(|value| value.parse().ok())
        .unwrap_or(0.0)
}

// Clock to date string
// ex: Clock => 2015-07-30
pub fn to_date(clock: &Clock) -> String {
    let timestamp = clock.timestamp.as_ref().expect("missing timestamp");
    timestamp
        .to_string()
        .split('T')
        .next()
        .expect("missing date")
        .to_string()
}

// pub fn to_key(trx_id: &str, action_index: u32) -> String {
//     format!("{}-{}", trx_id, action_index)
// }
