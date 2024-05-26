use html_to_string_macro::html;

pub fn sha() -> String {
    return html!(
          <div>
            <div class="p-4 space-y-2">
                <h3 class="font-medium">
                    {"# SHA-1"}
                </h3>
                <textarea
                    class="border p-2 w-full"
                    name="input1"
                    placeholder="Input"
                    rows={3}
                    hx-get="/@sha1?id=input1"
                    hx-trigger="input changed delay:50ms"
                    hx-target="#output1"
                    hx-swap="innerHTML"
                />
                <textarea
                    class="border p-2 w-full"
                    id="output1"
                    placeholder="Output"
                    readonly=""
                    rows={3}
                    hx-get="/@copy?id=output1"
                    hx-trigger="click"
                    hx-swap="beforeend"
                />
            </div>
            <hr />
            <div class="p-4 space-y-2">
                <h3 class="font-medium">
                    {"# SHA-256"}
                </h3>
                <textarea
                    class="border p-2 w-full"
                    name="input2"
                    placeholder="Input"
                    rows={3}
                    hx-get="/@sha256?id=input2"
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
            <hr />
            <div class="p-4 space-y-2">
                <h3 class="font-medium">
                    {"# SHA-512"}
                </h3>
                <textarea
                    class="border p-2 w-full"
                    name="input3"
                    placeholder="Input"
                    rows={3}
                    hx-get="/@sha512?id=input3"
                    hx-trigger="input changed delay:50ms"
                    hx-target="#output3"
                    hx-swap="innerHTML"
                />
                <textarea
                    class="border p-2 w-full"
                    id="output3"
                    placeholder="Output"
                    readonly=""
                    rows={3}
                    hx-get="/@copy?id=output3"
                    hx-trigger="click"
                    hx-swap="beforeend"
                />
            </div>
        </div>
    );
}
