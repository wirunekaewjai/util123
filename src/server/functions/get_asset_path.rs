use serde_json::Value;

pub fn get_asset_path(asset_map: &Value, route_key: &str) -> String {
    let route_path = asset_map[route_key].as_str().unwrap_or_default();

    if route_path.len() > 0 {
        return route_path.to_string();
    }

    if cfg!(debug_assertions) {
        println!("can't find asset: {route_key}");
    }

    route_key.to_string()
}
