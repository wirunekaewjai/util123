use html_to_string_macro::html;

pub fn base64() -> String {
    return html!(
        <div>
            <div class="p-4 space-y-2">
                <h3 class="font-medium">
                    {"# Base 64 Encode"}
                </h3>
                <textarea
                    class="border p-2 w-full"
                    name="input1"
                    placeholder="Input"
                    rows={3}
                    hx-get="/@base64-encode"
                    hx-trigger="input changed delay:50ms"
                    hx-target="#output1"
                    hx-swap="innerHTML"
                >
                </textarea>
                <textarea
                    class="border p-2 w-full"
                    id="output1"
                    placeholder="Output"
                    readonly=""
                    rows={3}
                    hx-get="/@copy?id=output1"
                    hx-trigger="click"
                    hx-swap="beforeend"
                >
                </textarea>
            </div>
            <hr />
            <div class="p-4 space-y-2">
                <h3 class="font-medium">
                    {"# Base 64 Decode"}
                </h3>
                <textarea
                    class="border p-2 w-full"
                    name="input2"
                    placeholder="Input"
                    rows={3}
                    hx-get="/@base64-decode"
                    hx-trigger="input changed delay:50ms"
                    hx-target="#output2"
                    hx-swap="innerHTML"
                />
                <textarea
                    class="border p-2 w-full"
                    id="output2"
                    placeholder="Output"
                    readonly=""
                    rows={3}
                    hx-get="/@copy?id=output2"
                    hx-trigger="click"
                    hx-swap="beforeend"
                />
            </div>
        </div>
    );
}
