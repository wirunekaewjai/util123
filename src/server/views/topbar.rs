use html_to_string_macro::html;

use super::icons;

pub fn topbar() -> String {
    return html!(
        <div
            class="bg-black text-white grid grid-cols-[32px_auto] gap-x-4 p-2 items-center [&_svg]:fill-current"
            hx-boost="true"
        >
            <a
                class="hover:bg-white/20 rounded-full flex p-2 w-8 h-8"
                href="/"
            >
                {icons::home()}
            </a>
        </div>
    );
}
