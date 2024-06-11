use html_to_string_macro::html;

pub fn qrcode() -> String {
    return html!(
        <div x-component="qrcode">
            <div class="p-4 space-y-2">
                <h3 class="font-medium">
                    {"# Input"}
                </h3>
                <textarea
                    class="border p-2 w-full"
                    placeholder="Enter URL / Text / Message"
                    rows={3}
                />
            </div>
            <hr />
            <div class="p-4 space-y-2">
                <h3 class="font-medium">
                    {"# Output"}
                </h3>
                <div class="w-40 h-40 border rounded-sm flex">
                    <img
                        alt="qrcode"
                        class="w-full h-full hidden"
                    />
                </div>
            </div>
        </div>
    );
}
