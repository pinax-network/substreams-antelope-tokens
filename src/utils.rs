use antelope::{Asset, Name};
use substreams::pb::substreams::Clock;
use substreams_antelope::pb::PermissionLevel;

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

pub fn authorization_to_string(authorization: &Vec<PermissionLevel>) -> Vec<String> {
    authorization
        .iter()
        .map(|a| format!("{}@{}", a.actor, a.permission))
        .collect()
}

pub fn parse_json_asset(data_json: &str, key: &str) -> Option<Asset> {
    let v = serde_json::from_str::<serde_json::Value>(data_json);
    match v {
        Ok(data) => {
            let value_str = data[key].as_str().unwrap_or("");
            let value = value_str.parse::<Asset>();
            match value {
                Ok(asset) => Some(asset),
                Err(_e) => None,
            }
        }
        Err(_e) => None,
    }
}

pub fn parse_json_name(data_json: &str, key: &str) -> Option<Name> {
    let v = serde_json::from_str::<serde_json::Value>(data_json);
    match v {
        Ok(data) => {
            let value_str = data[key].as_str().unwrap_or("");
            let value = value_str.parse::<Name>();
            match value {
                Ok(name) => Some(name),
                Err(_e) => None,
            }
        }
        Err(_e) => None,
    }
}
