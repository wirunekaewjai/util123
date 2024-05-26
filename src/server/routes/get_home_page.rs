use actix_web::{get, HttpRequest, HttpResponse};
use mime::TEXT_HTML;

use crate::{
    functions::create_etag_response,
    views::{self, icons},
};

#[get("/")]
pub async fn handle(req: HttpRequest) -> HttpResponse {
    let items = vec![
        //
        views::topbar(),
        views::heading(icons::list(), "Utilities"),
        views::utility_list(vec![
            //
            ("qrcode", icons::qrcode(), "QR Code Generator"),
            ("sha", icons::hashtag(), "SHA Hash"),
            ("base64", icons::codecompare(), "Base64 Encode / Decode"),
        ]),
    ];

    let html = views::doc("Utility 123", items);

    return create_etag_response(&req, TEXT_HTML, html.into_bytes());
}
