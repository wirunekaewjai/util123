use html_to_string_macro::html;

pub fn base64() -> String {
    return html!(
        <div>
            <div
                class="p-4 space-y-2"
                x-data="{ input: '', output: '' }"
                x-init="
                $watch('input', (value) => { output = btoa(value); });
                input = $refs.b64e_input.value;
                "
            >
                <h3 class="font-medium">
                    {"# Base 64 Encode"}
                </h3>
                <textarea
                    class="border p-2 w-full"
                    placeholder="Input"
                    rows={3}
                    x-ref="b64e_input"
                    x-model="input"
                >
                </textarea>
                <textarea
                    class="border p-2 w-full"
                    placeholder="Output"
                    readonly=""
                    rows={3}
                    x-bind:value="output"
                    x-on:click="await window.copyText(output)"
                >
                </textarea>
            </div>
            <hr />
            <div
                class="p-4 space-y-2"
                x-data="{ input: '', output: '' }"
                x-init="
                $watch('input', (value) => { output = atob(value); });
                input = $refs.b64d_input.value;
                "
            >
                <h3 class="font-medium">
                    {"# Base 64 Decode"}
                </h3>
                <textarea
                    class="border p-2 w-full"
                    placeholder="Input"
                    rows={3}
                    x-ref="b64d_input"
                    x-model="input"
                />
                <textarea
                    class="border p-2 w-full"
                    placeholder="Output"
                    readonly=""
                    rows={3}
                    x-bind:value="output"
                    x-on:click="await window.copyText(output)"
                />
            </div>
        </div>
    );
}
