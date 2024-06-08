use serde_json::Value;

#[derive(Clone)]
pub struct AppState {
    pub asset_map: Value,
}
