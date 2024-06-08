use html_to_string_macro::html;

pub fn qrcode() -> String {
    return html!(
        <div>
            <div class="p-4 space-y-2">
                <h3 class="font-medium">
                    {"# Input"}
                </h3>
                <textarea
                    class="border p-2 w-full"
                    name="input"
                    placeholder="Enter URL / Text / Message"
                    rows={3}
                    hx-get="/@qrcode"
                    hx-trigger="load, input changed delay:500ms"
                    hx-target="#qrcode"
                    hx-swap="innerHTML"
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
                />
            </div>
        </div>
    );
}
