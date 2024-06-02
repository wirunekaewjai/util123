use actix_web::{get, HttpRequest, HttpResponse};
use mime::TEXT_HTML;

use crate::views;

#[get("/")]
pub async fn handle(req: HttpRequest) -> HttpResponse {
    let items = vec![
        //
        views::topbar(),
        views::heading("list", "Utilities"),
        views::utility_list(vec![
            //
            ("qrcode", "qrcode", "QR Code Generator"),
            ("sha", "hashtag", "SHA Hash"),
            ("base64", "codecompare", "Base64 Encode / Decode"),
        ]),
    ];

    let html = views::doc("Utility 123", items);

    return jetpack::create_etag_response(&req, &TEXT_HTML, html.into_bytes());
}
