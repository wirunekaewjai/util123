use html_to_string_macro::html;

pub fn heading(icon_name: &str, content: &str) -> String {
    return html!(
        <div class="grid grid-cols-[16px_1fr] gap-x-4 px-4 py-2 items-center">
            <i class={format!("fa-solid {icon_name}")}></i>
            <h1 class="font-bold text-xl">
                {content}
            </h1>
        </div>
    );
}
