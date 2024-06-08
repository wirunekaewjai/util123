use std::fs::read_to_string;

use serde_json::{from_str, Value};

pub fn get_asset_map() -> Value {
    let text = read_to_string(".cache/map.json").expect("failed to read .cache/map.json");
    let json = from_str(&text).expect("invalid .cache/map.json");

    return json;
}
