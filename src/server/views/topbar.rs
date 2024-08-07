use html_to_string_macro::html;

pub fn topbar() -> String {
    return html!(
        <div class="bg-black text-white grid grid-cols-[32px_auto] gap-x-4 p-2 items-center [&_svg]:fill-current">
            <a
                aria-label="go to home"
                class="hover:bg-white/20 rounded-full flex p-2 w-8 h-8"
                href="/"
                x-component="link"
            >
                <i class="fa-solid fa-house"></i>
            </a>
        </div>
    );
}
