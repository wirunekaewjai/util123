use html_to_string_macro::html;

use crate::functions::map_children;

pub fn utility_list(items: Vec<(&str, &str, &str)>) -> String {
    return html!(
        <div
            class="space-y-2 divide-y"
            hx-boost="true"
        >
            {
                map_children(&items, &|(id, icon_name, name)| html!(
                    <a
                        class="hover:text-blue-400 grid grid-cols-[16px_1fr] gap-x-4 px-4 py-2 items-center"
                        href={format!("/utils/{id}")}
                    >
                        <i class={format!("fa-solid {icon_name}")}></i>
                        {name}
                    </a>
                ))
            }
        </div>
    );
}
