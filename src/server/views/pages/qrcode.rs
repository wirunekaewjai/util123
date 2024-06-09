use html_to_string_macro::html;

pub fn qrcode() -> String {
    return html!(
        <div
            x-data="{ qrcode: '' }"
        >
            <div
                class="p-4 space-y-2"
                x-data="{ input: '' }"
                x-init="
                $watch('input', async (value) => { qrcode = await window.createQRCode(value); });
                input = $refs.qrcode_input.value;
                "
            >
                <h3 class="font-medium">
                    {"# Input"}
                </h3>
                <textarea
                    class="border p-2 w-full"
                    placeholder="Enter URL / Text / Message"
                    rows={3}
                    x-ref="qrcode_input"
                    x-model="input"
                />
            </div>
            <hr />
            <div class="p-4 space-y-2">
                <h3 class="font-medium">
                    {"# Output"}
                </h3>
                <div
                    class="w-40 h-40 border rounded-sm flex"
                    id="qrcode"
                >
                    <img
                        alt="qrcode"
                        class="w-full h-full hidden"
                        x-bind:class="{ '!block': !!qrcode }"
                        x-bind:src="qrcode"
                    />
                </div>
            </div>
        </div>
    );
}
