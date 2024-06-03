use html_to_string_macro::html;
use serde_json::Value;

use super::icon;

pub fn heading(map: &Value, icon_name: &str, content: &str) -> String {
    return html!(
        <div class="grid grid-cols-[16px_1fr] gap-x-4 px-4 py-2 items-center">
            {icon(map, icon_name)}
            <h1 class="font-bold text-xl">
                {content}
            </h1>
        </div>
    );
}
