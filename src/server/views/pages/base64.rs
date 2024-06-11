use html_to_string_macro::html;

pub fn base64() -> String {
    return html!(
        <div>
            <div
                class="p-4 space-y-2"
                x-component="base64-encode"
            >
                <h3 class="font-medium">
                    {"# Base 64 Encode"}
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
                x-component="base64-decode"
            >
                <h3 class="font-medium">
                    {"# Base 64 Decode"}
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
