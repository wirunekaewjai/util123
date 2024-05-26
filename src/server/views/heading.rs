use html_to_string_macro::html;

pub fn heading(icon: String, content: &str) -> String {
    return html!(
        <div class="grid grid-cols-[16px_1fr] gap-x-4 px-4 py-2 items-center">
            {icon}
            <h1 class="font-bold text-xl">
                {content}
            </h1>
        </div>
    );
}
