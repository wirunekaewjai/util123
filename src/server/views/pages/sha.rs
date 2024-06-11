use html_to_string_macro::html;

pub fn sha() -> String {
    return html!(
          <div>
            <div
                class="p-4 space-y-2"
                x-component="sha1"
            >
                <h3 class="font-medium">
                    {"# SHA-1"}
                </h3>
                <textarea
                    class="border p-2 w-full"
                    placeholder="Input"
                    rows={3}
                />
                <textarea
                    class="border p-2 w-full"
                    placeholder="Output"
                    readonly=""
                    rows={3}
                    x-component="copy"
                />
            </div>
            <hr />
            <div
                class="p-4 space-y-2"
                x-component="sha256"
            >
                <h3 class="font-medium">
                    {"# SHA-256"}
                </h3>
                <textarea
                    class="border p-2 w-full"
                    placeholder="Input"
                    rows={3}
                />
                <textarea
                    class="border p-2 w-full"
                    placeholder="Output"
                    readonly=""
                    rows={3}
                    x-component="copy"
                />
            </div>
            <hr />
            <div
                class="p-4 space-y-2"
                x-component="sha512"
            >
                <h3 class="font-medium">
                    {"# SHA-512"}
                </h3>
                <textarea
                    class="border p-2 w-full"
                    placeholder="Input"
                    rows={3}
                />
                <textarea
                    class="border p-2 w-full"
                    placeholder="Output"
                    readonly=""
                    rows={3}
                    x-component="copy"
                />
            </div>
        </div>
    );
}
