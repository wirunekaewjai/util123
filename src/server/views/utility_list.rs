use html_to_string_macro::html;

use crate::functions;

pub struct UtilityListItem {
    pub id: String,
    pub icon: String,
    pub name: String,
}

pub fn utility_list(items: Vec<(&str, String, &str)>) -> String {
    return html!(
        <div
            class="space-y-2 divide-y"
            hx-boost="true"
        >
            {
                functions::map(&items, &|(id, icon, name)| html!(
                    <a
                        class="hover:text-blue-400 grid grid-cols-[16px_1fr] gap-x-4 px-4 py-2 items-center"
                        href={format!("/utils/{}", id)}
                    >
                        {icon}
                        {name}
                    </a>
                ))
            }
        </div>
    );
}
