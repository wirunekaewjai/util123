use html_to_string_macro::html;

pub fn gzip() -> String {
    return html!(
        <div>
            <div
                class="p-4 space-y-2"
                x-component="gzip"
                x-function="compress"
            >
                <h3 class="font-medium">
                    {"# Gzip Compress"}
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
                <div
                    class="p-2 grid grid-cols-[60px_auto_auto] gap-x-1 justify-start"
                >
                    <span>{"Input:"}</span>
                    <span name="count" class="text-right">0</span>
                    <span>{"char(s)"}</span>
                    <span>{"Output:"}</span>
                    <span name="count" class="text-right">0</span>
                    <span>{"char(s)"}</span>
                </div>
            </div>
            <hr />
            <div
                class="p-4 space-y-2"
                x-component="gzip"
                x-function="decompress"
            >
                <h3 class="font-medium">
                    {"# Gzip Decompress"}
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
                <div
                    class="p-2 grid grid-cols-[60px_auto_auto] gap-x-1 justify-start"
                >
                    <span>{"Input:"}</span>
                    <span name="count" class="text-right">0</span>
                    <span>{"char(s)"}</span>
                    <span>{"Output:"}</span>
                    <span name="count" class="text-right">0</span>
                    <span>{"char(s)"}</span>
                </div>
            </div>
        </div>
    );
}
