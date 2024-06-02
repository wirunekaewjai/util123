use html_to_string_macro::html;

pub fn icon(name: &str) -> String {
    let path = format!("/icons/{name}.svg");

    return html!(
        <svg
            hx-get={path}
            hx-trigger="load"
            hx-swap="outerHTML"
        />
    );
}
