use html_to_string_macro::html;

pub fn sha() -> String {
    return html!(
          <div>
            <div
                class="p-4 space-y-2"
                x-data="{ input: '', output: '' }"
                x-init="
                $watch('input', async (value) => { output = await window.createSHA(1, value); });
                input = $refs.sha1_input.value;
                "
            >
                <h3 class="font-medium">
                    {"# SHA-1"}
                </h3>
                <textarea
                    class="border p-2 w-full"
                    placeholder="Input"
                    rows={3}
                    x-ref="sha1_input"
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
            <hr />
            <div
                class="p-4 space-y-2"
                x-data="{ input: '', output: '' }"
                x-init="
                $watch('input', async (value) => { output = await window.createSHA(256, value); });
                input = $refs.sha256_input.value;
                "
            >
                <h3 class="font-medium">
                    {"# SHA-256"}
                </h3>
                <textarea
                    class="border p-2 w-full"
                    placeholder="Input"
                    rows={3}
                    x-ref="sha256_input"
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
            <hr />
            <div
                class="p-4 space-y-2"
                x-data="{ input: '', output: '' }"
                x-init="
                $watch('input', async (value) => { output = await window.createSHA(512, value); });
                input = $refs.sha512_input.value;
                "
            >
                <h3 class="font-medium">
                    {"# SHA-512"}
                </h3>
                <textarea
                    class="border p-2 w-full"
                    placeholder="Input"
                    rows={3}
                    x-ref="sha512_input"
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
