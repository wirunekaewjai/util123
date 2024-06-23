use html_to_string_macro::html;
use serde_json::Value;

use crate::functions;

pub fn debug_scripts(asset_map: &Value) -> String {
    match cfg!(debug_assertions) {
        true => html!(
            <script async defer src={functions::get_asset_path(asset_map, "/assets/auto-reload.js")}></script>
        ),

        false => "".into(),
    }
}
