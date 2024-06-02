use html_to_string_macro::html;

pub fn icon(name: &str) -> String {
    return html!(
        <div
            hx-get={format!("/icons/{name}.svg")}
            hx-trigger="load"
            hx-swap="outerHTML"
        />
    );
}
