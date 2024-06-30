use actix_web::{get, web, HttpRequest, HttpResponse};
use html_to_string_macro::html;

use crate::{functions, structs, views};

#[get("/")]
pub async fn handle(req: HttpRequest, state: web::Data<structs::AppState>) -> HttpResponse {
    let items = vec![
        //
        views::topbar(),
        views::heading("fa-list", "Utilities"),
        views::utility_list(
            //
            vec![
                //
                ("qrcode", "fa-qrcode", "QR Code Generator"),
                ("sha", "fa-hashtag", "SHA Hash"),
                ("base64", "fa-code-compare", "Base64 Encode / Decode"),
                ("gzip", "fa-file-zipper", "Gzip Compress / Decompress"),
            ],
        ),
    ];

    if functions::get_is_fragment_rendering(&req) {
        let html = html!(
            <body>
                {items.join("")}
            </body>
        );

        return functions::send_html_response(&req, &html);
    }

    let html = views::doc(&state.asset_map, "Utility 123", items);
    return functions::send_html_response(&req, &html);
}
