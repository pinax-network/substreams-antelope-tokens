use antelope::Asset;

pub fn to_value(quantity: &Asset) -> f64 {
    quantity
        .to_string()
        .split_whitespace()
        .next()
        .and_then(|value| value.parse().ok())
        .unwrap_or(0.0)
}
